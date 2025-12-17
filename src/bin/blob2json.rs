use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufWriter, Write};

use clap::Parser;
use rs_blob2json::{Blob, BlobBuilder};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    name: String,

    #[arg(long)]
    content_type: Option<String>,

    #[arg(long)]
    content_encoding: Option<String>,

    #[arg(long)]
    max_bytes: Option<u64>,

    #[arg(long, value_parser = parse_key_val)]
    metadata: Vec<(String, String)>,
}

/// Parse a key-value pair, separated by '='
fn parse_key_val(s: &str) -> Result<(String, String), Box<dyn Error + Send + Sync + 'static>> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no '=' found in '{}'", s))?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut builder = BlobBuilder::default();
    if let Some(ctyp) = args.content_type {
        builder = builder.with_content_type(ctyp);
    }
    if let Some(enc) = args.content_encoding {
        builder = builder.with_content_encoding(enc);
    }
    if let Some(mb) = args.max_bytes {
        builder = builder.with_max_bytes(mb);
    }

    if !args.metadata.is_empty() {
        let metadata: HashMap<String, String> = args.metadata.into_iter().collect();
        builder = builder.with_metadata(metadata);
    }

    let stdin = io::stdin();
    let blob: Blob = builder.rdr2blob(stdin.lock(), args.name)?;

    let stdout = io::stdout();
    let locked = stdout.lock();
    let mut buffered = BufWriter::new(locked);
    serde_json::to_writer(&mut buffered, &blob)?;
    writeln!(buffered)?;
    let mut locked = buffered.into_inner().map_err(|e| e.into_error())?;
    locked.flush()?;

    Ok(())
}
