use std::{fs::read_dir, path::{PathBuf, Path}};

use tokio::{fs::File, io::AsyncReadExt};

// use async_lock::RwLock;

use futures_util::future::try_join_all;

use structopt::StructOpt;

use once_cell::sync::Lazy;

use regex::Regex;

use log::error;

static ENUM_FINDER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"enum (?P<name>[^{\s]+)[\n\s]*\{(?P<content>[\s\S]+?)\}"#).unwrap());
static ENUM_ANALIZER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(\s*///\s*[^\n]+\n)+(\s*#\[serde\(rename\s*=\s*"(?P<rename>[^"]+)"\)\])?\s*(?P<symbol>\w+),"#).unwrap());

#[derive(Debug, StructOpt)]
#[structopt(name = "enum_checker", about = "Enum duplicate finder")]
struct Opt {
    dir: PathBuf,
}

enum DirOrFile<T: Iterator<Item=PathBuf>> {
    Dir(T),
    File(Option<PathBuf>),
}

impl<T: Iterator<Item=PathBuf>> Iterator for DirOrFile<T> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            DirOrFile::Dir(d) => d.next(),
            DirOrFile::File(f) => f.take(),
        }
    }
}

// why isn't it async?
// because I haven't found a simple async flat_map equivalent
fn dir_walk<P: AsRef<Path>>(dir: P) -> Vec<PathBuf> {
    read_dir(dir).unwrap()
        .flat_map(|rde| {
            let p = rde.unwrap().path();
            if p.is_dir() {
                DirOrFile::Dir(dir_walk(p).into_iter())
            }
            else {
                DirOrFile::File(Some(p))
            }
        })
        .collect()
}

struct EnumEntry {
    file: PathBuf,
    name: String,
    elements: Vec<String>,
}

async fn read_file(path: PathBuf) -> Result<Vec<EnumEntry>, ()> {
    let mut file = File::open(&path).await.map_err(|e| error!("Error opening file {}: {}", path.display(), e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.map_err(|e| error!("Error opening file {}: {}", path.display(), e))?;

    let mut entries = Vec::new();
    for capture in ENUM_FINDER.captures_iter(&contents) {
        let elements: Vec<String> = ENUM_ANALIZER.captures_iter(&capture["content"]).map(|c| c.name("rename").map(|m| m.as_str()).unwrap_or(&c["symbol"]).to_owned()).collect();
        if !elements.is_empty() {
            entries.push(EnumEntry {
                file: path.clone(),
                name: (&capture["name"]).to_owned(),
                elements,
            });
        }
    }

    Ok(entries)
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    env_logger::init();

    let opt = Opt::from_args();
    let files = dir_walk(opt.dir);

    let entries = try_join_all(files.into_iter().map(|path| async {
        read_file(path).await
    })).await?;
    let ref_entries = entries.iter().flatten();

    for entry1 in ref_entries.clone() {
        for entry2 in ref_entries.clone() {
            if entry1.file == entry2.file && entry1.name == entry2.name {
                continue;
            }
            if entry1.elements == entry2.elements {
                println!("Equal {}::{} == {}::{} ({:?})", entry1.file.display(), entry1.name, entry2.file.display(), entry2.name, entry1.elements);
            }
        }
    }

    Ok(())
}
