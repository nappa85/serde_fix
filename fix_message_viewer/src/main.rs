use std::io::{self, Read};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "fix_message_viewer", about = "FiX Message viewer")]
struct Opt {
    #[structopt(short, long)]
    separator: Option<String>,
}

fn main() -> Result<(), io::Error> {
    let opt = Opt::from_args();

    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf)?;

    if let Some(sep)  = opt.separator {
        let s = String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        buf = s.replace(&sep, "\u{1}").into_bytes();
    }

    println!("{:#?}", serde_fix::from_bytes::<fix_message::Message>(&buf));
    Ok(())
}
