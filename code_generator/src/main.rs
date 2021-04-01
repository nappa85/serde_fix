use std::borrow::Cow;

use structopt::StructOpt;

use reqwest::ClientBuilder;

use regex::Regex;

use convert_case::{Case, Casing};

use log::error;

#[derive(Debug, StructOpt)]
#[structopt(name = "code_generator", about = "FiX code generator")]
struct Opt {
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    env_logger::init();

    let opt = Opt::from_args();

    // create http client
    let client = ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| error!("Client build error: {}", e))?;

    // prepare base_url for future relative calls
    let mut base_url = opt.url.split('/').collect::<Vec<_>>();
    base_url.pop();
    base_url.push("");
    let base_url = base_url.join("/");

    // call url and retrieve body text
    let text = client.get(&opt.url)
        .send()
        .await
        .map_err(|e| error!("URL get error: {}", e))?
        .text()
        .await
        .map_err(|e| error!("Body load error: {}", e))?;

    // prepare regexs
    let table = Regex::new(r#"<h1>(&lt;)?(?P<name>\w+)(&gt;)?[\s\S]+?<table[^>]+>[\n\s]+<tr class="tbl-hdr">([\n\s]+<th[^>]*>[^<]+</th>)+[\n\s]+</tr>(?P<body>[\s\S]+?)</table>"#)
        .map_err(|e| error!("Error building table Regex: {}", e))?;
    let tr = Regex::new(r#"<tr[^>]+>(?P<body>[\s\S]+?)</tr>"#)
        .map_err(|e| error!("Error building tr Regex: {}", e))?;
    let block = Regex::new(r#"<td class="block[^>]+><a href="[^"]+">&lt;(?P<name>[\w\s]+)&gt;</a></td>[\n\s]+<td class="req[^>]+>(?P<req>[YNC])</td>[\n\s]+<td class="comment[^>]+>(?P<comment>[\s\S]*?)</td>"#)
        .map_err(|e| error!("Error building block Regex: {}", e))?;
    let tag = Regex::new(r#"<td class="tag[^>]+>(?P<id>\d+)</td>[\n\s]+<td class="field-name[^>]+><a href="(?P<url>[^"]+)">(?P<name>[\w\s]+)</a></td>[\s\S]+?<td class="req[^>]+>(?P<req>[YNC])</td>[\n\s]+<td class="comment[^>]+>(?P<comment>[\s\S]*?)</td>"#)
        .map_err(|e| error!("Error building tag Regex: {}", e))?;
    let vartype = Regex::new(r#"<h1>\w+<font[^>]+> \(Tag = \d+, Type: <a href="[^"]+">(?P<type>\w+)</a>\)</font></h1>"#)
        .map_err(|e| error!("Error building vartype Regex: {}", e))?;
    let enum_body = Regex::new(r#"<p>Valid values:[\n\s]+<table[^>]+>(?P<enum>[\s\S]+?)</table>[\n\s]+</p>"#)
        .map_err(|e| error!("Error building enum_body Regex: {}", e))?;

    let mut enums = Vec::new();

    // retrieve struct name and table body
    let captures = table.captures(&text).ok_or_else(|| error!("Unrecognized body table"))?;
    println!("#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]\npub struct {} {{", clean_name(&captures["name"]));
    for tr_match in tr.captures_iter(&captures["body"]) {
        let tr_body = &tr_match["body"];
        // an import block
        if let Some(b) = block.captures(tr_body) {
            let name = clean_name(&b["name"]);
            println!("\t/// {}\n\t#[serde(flatten)]\n\tpub {}: {},", clean_comment(&b["comment"], &b["name"]), to_snake_case(&name), maybe_option(&name, &b["req"]));
        }
        // a normal tagvalue
        else if let Some(t) = tag.captures(tr_body) {
            let name = clean_name(&t["name"]);
            let descr = client.get(format!("{}{}", base_url, &t["url"]))
                .send()
                .await
                .map_err(|e| error!("Child URL {} get error: {}", &t["url"], e))?
                .text()
                .await
                .map_err(|e| error!("Child body {} load error: {}", &t["url"], e))?;
            let t_type = vartype.captures(&descr).ok_or_else(|| error!("Unrecognized vartype on {}", &t["url"]))?;
            let type_name = if let Some(list) = enum_body.captures(&descr) {
                enums.push((name.clone(), (&list["enum"]).to_owned()));
                &name
            }
            else {
                type_map(&t_type["type"])
            };
            println!("\t/// {}\n\t#[serde(rename = \"{}\")]\n\tpub {}: {},", clean_comment(&t["comment"], &t["name"]), &t["id"], to_snake_case(&name), maybe_option(&type_name, &t["req"]));
        }
        else {
            panic!("Unrecognized tr: {}", tr_body);
        }
    }
    println!("}}");

    // manage enums
    if !enums.is_empty() {
        let enum_items = Regex::new(r#"<tr[^>]+>[\n\s]+<td class="val">'(?P<val>[^']+)'</td>[\n\s]+<td class="val-descr">(?P<desc>[\s\S]+?)</td>[\n\s]+</tr>"#)
            .map_err(|e| error!("Error building enum_items Regex: {}", e))?;
        for (name, list) in enums {
            println!("\n#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]\npub enum {} {{", name);
            for cap in enum_items.captures_iter(&list) {
                let name = clean_enum_name(&cap["desc"], &cap["val"]).map_err(|_| error!("Unable to clean enum name for {:?}", cap))?;
                println!("\t/// {}\n\t#[serde(rename = \"{}\")]\n\t{},", clean_comment(&cap["desc"], &cap["val"]), &cap["val"], name);
            }
            println!("}}");
        }
    }

    Ok(())
}

fn clean_name(name: &str) -> String {
    name.replace(|c: char| !c.is_ascii_alphanumeric(), "")
}

fn clean_enum_name(name: &str, key: &str) -> Result<String, ()> {
    let mut temp = "";
    let normalized = name.replace(|c: char| !c.is_ascii_alphanumeric(), " ");
    for part in normalized.split("  ") {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        temp = part;
        break;
    }
    if temp.is_empty() {
        temp = key;
    }
    let temp2: Cow<'_, str> = match temp.chars().next().map(char::is_numeric) {
        Some(true) => format!("N{}", temp).into(),
        Some(false) => temp.into(),
        None => return Err(()),
    };
    Ok(temp2.to_case(Case::Pascal))
}

fn clean_comment<'a>(comment: &'a str, name: &'a str) -> Cow<'a, str> {
    let res = comment.split("\n")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("\n\t/// ");
    if res.is_empty() {
        name.into()
    }
    else {
        res.into()
    }
}

fn to_snake_case(name: &str) -> String {
    name.to_case(Case::Snake)
}

fn maybe_option<'a>(name: &'a str, req: &'a str) -> Cow<'a, str> {
    if req == "Y" {
        name.into()
    }
    else {
        format!("Option<{}>", name).into()
    }
}

fn type_map(t: &str) -> &str {
    match t {
        "String" => "String",
        "int" => "i32",
        "LocalMktDate" => "LocalMktDate",
        "Price" => "f64",
        "Percentage" => "f32",
        "float" => "f64",
        "UTCTimestamp" => "UTCTimestamp",
        "Boolean" => "Boolean",
        "Qty" => "f64",
        "PriceOffset" => "f64",
        "Exchange" => "String",
        "Amt" => "f64",
        "Length" => "usize",
        "data" => "String",
        _ => panic!("Unmapped type {}", t),
    }
}
