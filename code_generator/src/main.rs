use std::{borrow::Cow, path::PathBuf};

use tokio::{io::{AsyncWrite, AsyncWriteExt, stdout}, fs::{File, create_dir_all, remove_file}};

use futures_util::stream::{iter, StreamExt};

use async_lock::Mutex;

use structopt::StructOpt;

use reqwest::{Client, ClientBuilder};

use regex::{Captures, Regex};

use convert_case::{Case, Casing};

use once_cell::sync::Lazy;

use log::{error, info};

#[derive(Debug, StructOpt)]
#[structopt(name = "code_generator", about = "FiX code generator")]
struct Opt {
    #[structopt(short, long)]
    recursive: bool,
    url: String,
    #[structopt(short, long)]
    out: Option<PathBuf>,
    #[structopt(short, long, default_value = "10")]
    concurrency: usize,
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

    if opt.recursive {
        let base_dir = opt.out.ok_or_else(|| error!("Can't work recursively without an <out> dir"))?;
        if !base_dir.as_path().exists() {
            create_dir_all(&base_dir).await
                .map_err(|e| error!("Error creating folder {}: {}", base_dir.display(), e))?;
        }
        if !base_dir.as_path().is_dir() {
            error!("When working recursively, <out> must be a directory");
            return Err(());
        }

        let text = get_body(&opt.url, &client).await?;
        let base_url = get_base_url(&opt.url);

        let block_mod = Mutex::new(Vec::new());
        let message_mod = Mutex::new(Vec::new());

        let li = Regex::new(r#"<li><a target="main" href="(?P<url>[^"]+)">(\(\w+\)&nbsp;)?(?P<name>[^<&]+)(&nbsp;\(\w+\))?</a></li>"#)
            .map_err(|e| error!("Error building li Regex: {}", e))?;
        iter(li.captures_iter(&text)).for_each_concurrent(Some(opt.concurrency), |c| {
            let mut file = base_dir.clone();
            let base_url = &base_url;
            let client = &client;
            let block_mod = &block_mod;
            let message_mod = &message_mod;
            async move {
                let is_message;
                let name = (&c["name"]).replace(|c| {
                        static CHARS: &[char] = &['/', '(', ')'];
                        CHARS.contains(&c)
                    }, "").to_case(Case::Snake);
                if (&c["url"]).starts_with("message_") {
                    file.push("messages");
                    if !file.as_path().exists() {
                        if let Err(e) = create_dir_all(&file).await {
                            error!("Error creating folder {}: {}", file.display(), e);
                            return;
                        }
                    }
                    is_message = true;
                }
                else {
                    if !(&c["url"]).starts_with("block_") {
                        return;
                    }
                    is_message = false;
                }
                file.push(name.clone());
                file = file.with_extension("rs");
                if file.as_path().exists() {
                    return;
                }

                let url = format!("{}{}", base_url, &c["url"]);
                info!("{} => {}", url, file.display());
                match File::create(&file).await {
                    Ok(mut f) => {
                        if parse_url(&url, &client, is_message, &mut f).await.is_err() {
                            remove_file(&file).await
                                .map_err(|e| error!("File delete error {}: {}", file.display(), e))
                                .ok();
                        }
                        else {
                            if is_message {
                                let mut lock = message_mod.lock().await;
                                lock.push(name);
                            }
                            else {
                                let mut lock = block_mod.lock().await;
                                lock.push(name);
                            }
                        }
                    },
                    Err(e) => error!("File open error {}: {}", file.display(), e),
                }
            }
        }).await;

        let lock = message_mod.lock().await;
        let messages;
        if !lock.is_empty() {
            messages = true;
            let mut file = base_dir.clone();
            file.push("messages");
            file.push("mod.rs");
            if !file.as_path().exists() {
                let mut f = File::create(&file)
                    .await
                    .map_err(|e| error!("File open error {}: {}", file.display(), e))?;
                for mod_name in lock.iter() {
                    f.write_all(format!("pub mod {};\n", mod_name).as_bytes())
                        .await
                        .map_err(|e| error!("File write error {}: {}", file.display(), e))?;
                }
            }
        }
        else {
            messages = false;
        }

        let lock = block_mod.lock().await;
        if !lock.is_empty() {
            let mut file = base_dir.clone();
            file.push("mod.rs");
            if !file.as_path().exists() {
                let mut f = File::create(&file)
                    .await
                    .map_err(|e| error!("File open error {}: {}", file.display(), e))?;
                if messages {
                    f.write_all("pub mod messages;\n".as_bytes())
                        .await
                        .map_err(|e| error!("File write error {}: {}", file.display(), e))?;
                }
                for mod_name in lock.iter() {
                    f.write_all(format!("pub mod {};\n", mod_name).as_bytes())
                        .await
                        .map_err(|e| error!("File write error {}: {}", file.display(), e))?;
                }
            }
        }
    }
    else {
        if let Some(file) = opt.out {
            if file.as_path().is_dir() {
                error!("When working on a single <url>, <out> must be a file");
                return Err(());
            }

            let mut f = File::create(&file)
                .await
                .map_err(|e| error!("File open error {}: {}", file.display(), e))?;
            parse_url(&opt.url, &client, false, &mut f).await?;
        }
        else {
            let mut stdout = stdout();
            parse_url(&opt.url, &client, false, &mut stdout).await?;
        }
    }

    Ok(())
}

async fn parse_url<W: AsyncWrite + Unpin>(url: &str, client: &Client, is_message: bool, out: &mut W) -> Result<(), ()> {
    let text = get_body(url, client).await?;
    let base_url = get_base_url(url);

    // prepare regexs
    let table = Regex::new(r#"<h1>(&lt;)?(?P<name>\w+)(&gt;)?[\s\S]+?<table[^>]+>[\n\s]+<tr class="tbl-hdr">([\n\s]+<th[^>]*>[^<]+</th>)+[\n\s]+</tr>(?P<body>[\s\S]+?)</table>"#)
        .map_err(|e| error!("Error building table Regex: {}", e))?;
    let tr = Regex::new(r#"<tr[^>]+>(?P<body>[\s\S]+?)</tr>(?P<rows>([\n\s]+<tr[^>]+>[\n\s]+<td[^>]+>=&gt;</td>[\s\S]+?</tr>)+)?"#)
        .map_err(|e| error!("Error building tr Regex: {}", e))?;
    let block = Regex::new(r#"<td class="block[^>]+><a[\s\S]+?href="[^"]+">&lt;(?P<name>[\w\s]+)&gt;</a></td>[\n\s]+<td class="req[^>]+>(?P<req>[YNC])</td>[\n\s]+<td class="comment[^>]+>(?P<comment>[\s\S]*?)</td>"#)
        .map_err(|e| error!("Error building block Regex: {}", e))?;
    let tag = Regex::new(r#"<td class="tag[^>]+>(?P<id>\d+)</td>[\n\s]+<td class="field-name[^>]+><a[\s\S]+?href="(?P<url>[^"]+)">(?P<name>[\w\s]+)</a></td>[\s\S]+?<td class="req[^>]+>(?P<req>[YNC])</td>[\n\s]+<td class="comment[^>]+>(?P<comment>[\s\S]*?)</td>"#)
        .map_err(|e| error!("Error building tag Regex: {}", e))?;
    let vartype = Regex::new(r#"<h1>\w+<font[^>]+> \(Tag = \d+, Type: <a[\s\S]+?href="[^"]+">(?P<type>[\w-]+)</a>\)</font></h1>"#)
        .map_err(|e| error!("Error building vartype Regex: {}", e))?;
    let enum_body = Regex::new(r#"<p>Valid values:[\n\s]+<table[^>]+>(?P<enum>[\s\S]+?)</table>[\n\s]+</p>"#)
        .map_err(|e| error!("Error building enum_body Regex: {}", e))?;

    let mut enums = Vec::new();
    let mut arrays = Vec::new();

    // retrieve struct name and table body
    let captures = table.captures(&text).ok_or_else(|| error!("Unrecognized body table"))?;
    let name = clean_name(&captures["name"]);
    out.write_all(format!("\nuse serde::{{Deserialize, Serialize}};\n\n#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]\npub struct {} {{\n", name).as_bytes())
        .await
        .map_err(|e| error!("{}", e))?;
    for tr_match in tr.captures_iter(&captures["body"]) {
        let tr_body = &tr_match["body"];

        // an import block
        if let Some(b) = block.captures(tr_body) {
            let name = clean_name(&b["name"]);
            out.write_all(format!("\t/// {}\n\t#[serde(flatten)]\n\tpub {}: {},\n", clean_comment(&b["comment"], &b["name"]), to_snake_case(&name), maybe_option(&name, &b["req"], if is_message { 2 } else { 1 })).as_bytes())
                .await
                .map_err(|e| error!("{}", e))?;
        }
        // a normal tagvalue
        else if let Some(t) = tag.captures(tr_body) {
            let is_array = tr_match.name("rows").is_some();
            let name = tag_processor(&t, &client, &base_url, tr_body, &name, is_array, &mut enums, &vartype, &enum_body, out).await?;
            if is_array {
                arrays.push(((&name[0..(name.len() - 1)]).to_owned(), (&tr_match["rows"]).to_owned()));
            }
        }
        else {
            error!("Unrecognized tr: {}", tr_body);
            return Err(());
        }
    }
    out.write_all("}\n".as_bytes())
        .await
        .map_err(|e| error!("{}", e))?;

    // manage arrays
    if !arrays.is_empty() {
        for (name, list) in arrays {
            out.write_all(format!("\n#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]\npub struct {} {{\n", name).as_bytes())
                .await
                .map_err(|e| error!("{}", e))?;
            for t in tag.captures_iter(&list) {
                tag_processor(&t, &client, &base_url, &list, &name, false, &mut enums, &vartype, &enum_body, out).await?;
            }
            out.write_all("}\n".as_bytes())
                .await
                .map_err(|e| error!("{}", e))?;
        }
    }

    // manage enums
    if !enums.is_empty() {
        let enum_items = Regex::new(r#"<tr[^>]+>[\n\s]+<td class="val">'(?P<val>[^']+)'</td>[\n\s]+<td class="val-descr">(?P<desc>[\s\S]+?)</td>[\n\s]+</tr>"#)
            .map_err(|e| error!("Error building enum_items Regex: {}", e))?;
        for (name, list) in enums {
            let items = enum_items.captures_iter(&list).map(|cap| (clean_comment(&cap["desc"], &cap["val"]), (&cap["val"]).to_owned(), clean_enum_name(&cap["desc"]))).collect::<Vec<_>>();

            //check for duplicates
            let mut names = items.iter().map(|(_, _, name)| name).collect::<Vec<_>>();
            names.sort_unstable();
            names.dedup();
            let has_duplicates = items.len() != names.len();

            out.write_all(format!("\n#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]\npub enum {} {{\n", name).as_bytes())
                .await
                .map_err(|e| error!("{}", e))?;
            for (comment, val, name) in items {
                let label = if has_duplicates {
                    let temp = clean_enum_name(&val);
                    if temp.is_empty() {
                        name
                    }
                    // avoid upper-lower case duplicates
                    else if val.len() == 1 && val.chars().next().map(|c| c.is_ascii_lowercase()) == Some(true) {
                        format!("_{}", temp)
                    }
                    else {
                        temp
                    }
                }
                else {
                    name
                };
                out.write_all(format!(
                        "\t/// {}\n\t#[serde(rename = \"{}\")]\n\t{},\n",
                        comment,
                        val,
                        if label.chars().next().map(char::is_numeric) == Some(true) {
                            format!("N{}", label)
                        }
                        else {
                            label
                        }
                    ).as_bytes())
                    .await
                    .map_err(|e| error!("{}", e))?;
            }
            out.write_all("}\n".as_bytes())
                .await
                .map_err(|e| error!("{}", e))?;
        }
    }

    Ok(())
}

async fn tag_processor<W: AsyncWrite + Unpin>(t: &Captures<'_>, client: &Client, base_url: &str, tr_body: &str, parent: &str, is_array: bool, enums: &mut Vec<(String, String)>, vartype: &Regex, enum_body: &Regex, out: &mut W) -> Result<String, ()> {
    let mut name = clean_name(&t["name"]);
    let descr = client.get(format!("{}{}", base_url, &t["url"]))
        .send()
        .await
        .map_err(|e| error!("Child URL {} get error: {}", &t["url"], e))?
        .text()
        .await
        .map_err(|e| error!("Child body {} load error: {}", &t["url"], e))?;
    let t_type = vartype.captures(&descr).ok_or_else(|| error!("Unrecognized vartype on {}", &t["url"]))?;
    let mut workaround = false;
    let type_name: Cow<'_, str> = if let Some(list) = enum_body.captures(&descr) {
        if name.as_str() == parent {
            name += "Item";
        }
        enums.push((name.clone(), (&list["enum"]).to_owned()));
        if (&t_type["type"]).starts_with("Multiple") {
            format!("fix_common::SeparatedValues<{}>", name).into()
        }
        else {
            name.as_str().into()
        }
    }
    else {
        let mut temp: Cow<'_, str> = type_map(&t_type["type"]).map_err(|_| error!("Unmapped type {} in {}", &t_type["type"], &t["url"]))?.into();
        if is_array {
            if temp.as_ref() == "usize" {
                name = (&name[2..]).to_owned();
                temp = format!("fix_common::RepeatingValues<{}>", &name[0..(name.len() - 1)]).into();
            }
            else {
                error!("Found array without length: {}", tr_body);
                return Err(());
            }
        }
        else {
            workaround = needs_workaround(temp.as_ref());
        }
        temp
    };

    out.write_all(format!("\t/// {}{}\n\t#[serde(rename = \"{}\")]\n\tpub {}: {},\n", clean_comment(&t["comment"], &t["name"]), option_check(&t["req"], workaround), &t["id"], to_snake_case(&name), maybe_option(&type_name, &t["req"], 0)).as_bytes())
        .await
        .map_err(|e| error!("{}", e))?;

    Ok(name)
}

// prepare base_url for future relative calls
fn get_base_url(url: &str) -> String {
    let mut base_url = url.split('/').collect::<Vec<_>>();
    base_url.pop();
    base_url.push("");
    base_url.join("/")
}

// call url and retrieve body text
async fn get_body(url: &str, client: &Client) -> Result<String, ()> {
    client.get(url)
        .send()
        .await
        .map_err(|e| error!("URL get error {}: {}", url, e))?
        .text()
        .await
        .map_err(|e| error!("Body load error {}: {}", url, e))
}

fn clean_name(name: &str) -> String {
    name.replace(|c: char| !c.is_ascii_alphanumeric(), "")
}

fn clean_enum_name(name: &str) -> String {
    static CLEANUP: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\([^\)]+\)"#).unwrap());
    CLEANUP.replace_all(name, "").as_ref().replace(|c: char| !c.is_ascii_alphanumeric(), " ").to_case(Case::Pascal)
}

fn clean_comment<'a>(comment: &'a str, name: &'a str) -> String {
    let res = comment.split("\n")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("\n\t/// ");
    if res.is_empty() {
        name.to_owned()
    }
    else {
        res
    }
}

fn to_snake_case(name: &str) -> String {
    name.to_case(Case::Snake)
}

fn maybe_option<'a>(name: &'a str, req: &'a str, n: usize) -> Cow<'a, str> {
    if req == "Y" {
        if n == 0 {
            name.into()
        }
        else {
            format!("{}{}::{}", "super::".repeat(n), to_snake_case(name), name).into()
        }
    }
    else {
        if n == 0 {
            format!("Option<{}>", name).into()
        }
        else {
            format!("Option<{}{}::{}>", "super::".repeat(n), to_snake_case(name), name).into()
        }
    }
}

fn needs_workaround(var_type: &str) -> bool {
    static TYPES: &[&str] = &["i32", "f64", "f32", "usize"];
    TYPES.contains(&var_type)
}

fn option_check(req: &str, workaround: bool) -> &str {
    match (req == "Y", workaround) {
        (true, false) => "",
        (true, true) => "\n\t#[serde(deserialize_with = \"fix_common::workarounds::from_str\")]// https://github.com/serde-rs/serde/issues/1183",
        (false, false) => "\n\t#[serde(skip_serializing_if = \"Option::is_none\")]",
        (false, true) => "\n\t#[serde(skip_serializing_if = \"Option::is_none\")]\n\t#[serde(deserialize_with = \"fix_common::workarounds::from_opt_str\")]// https://github.com/serde-rs/serde/issues/1183\n\t#[serde(default)]",
    }
}

fn type_map(t: &str) -> Result<&str, ()> {
    Ok(match t {
        "String" => "String",
        "int" => "i32",
        "LocalMktDate" => "fix_common::LocalMktDate",
        "Price" => "f64",
        "Percentage" => "f32",
        "float" => "f64",
        "UTCTimestamp" => "fix_common::UTCTimestamp",
        "Boolean" => "fix_common::Boolean",
        "Qty" => "f64",
        "PriceOffset" => "f64",
        "Exchange" => "String",
        "Amt" => "f64",
        "Length" => "usize",
        "data" => "String",
        "month-year" => "fix_common::MonthYear",
        "TZTimeOnly" => "fix_common::TZTimeOnly",
        "TZTimestamp" => "fix_common::TZTimestamp",
        "UTCTimeOnly" => "fix_common::UTCTimeOnly",
        "UTCDateOnly" => "fix_common::UTCDateOnly",
        "char" => "char",
        "NumInGroup" => "usize",
        "Currency" => "String",
        "Country" => "String",
        "SeqNum" => "usize",
        "XMLData" => "String",
        "TagNum" => "u16",
        _ => return Err(()),
    })
}
