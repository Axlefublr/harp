use std::error::Error;
use std::fs::OpenOptions;
use std::fs::{self};

use clap::Parser;

use crate::args::Args;
use crate::data::Entries;

mod args;
mod data;

const DATA_FILE: &str = "harp.yml";

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let data_dir = dirs::data_local_dir().ok_or("couldn't find local data directory")?;
    let harp_data_dir = data_dir.join("harp");
    fs::create_dir_all(&harp_data_dir)?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(harp_data_dir.join(DATA_FILE))?;
    let model: Entries = serde_yaml::from_reader(file)?;
    if args.path.is_none() && args.line.is_none() && args.column.is_none() {
        let entry = model
            .get(&args.parent)
            .ok_or_else(|| format!("couldn't find parent `{}`", args.parent))?
            .get(&args.child)
            .ok_or_else(|| {
                format!(
                    "couldn't find child `{}` under parent `{}`",
                    args.child, args.parent
                )
            })?;
        let joined = [
            entry.path.as_ref().map(|path| path.display().to_string()),
            entry.line.map(|line| line.to_string()),
            entry.column.map(|column| column.to_string()),
        ];
        let result = joined.into_iter().flatten().collect::<Vec<_>>().join(" ");
        print!("{result}");
    }
    Ok(())
}
