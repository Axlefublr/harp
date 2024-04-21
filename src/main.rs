use std::error::Error;
use std::fs::OpenOptions;
use std::fs::{self};
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;

use clap::Parser;

use crate::args::Action;
use crate::args::Args;
use crate::data::Entries;

mod args;
mod data;

const DATA_FILE: &str = "harp.yml";

fn main() -> ExitCode {
    let args = Args::parse();
    if let Err(err) = _main(args.action) {
        if !args.quiet {
            eprintln!("{}", err);
        }
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn _main(action: Action) -> Result<(), Box<dyn Error>> {
    let data_dir = dirs::data_local_dir().ok_or("couldn't find local data directory")?;
    let harp_data_dir = data_dir.join("harp");
    fs::create_dir_all(&harp_data_dir)?;
    let harp_data_file = harp_data_dir.join(DATA_FILE);
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&harp_data_file)?;
    let mut model: Entries = serde_yaml::from_reader(file)?;
    match action {
        Action::Get {
            section,
            register,
            path,
            line,
            column,
        } => {
            let entry = model
                .get(&section)
                .ok_or_else(|| format!("couldn't find parent `{}`", section))?
                .get(&register)
                .ok_or_else(|| format!("couldn't find child `{}` under parent `{}`", register, section))?;
            let joined = [
                entry.path.as_ref().map(|entry_path| {
                    if path {
                        entry_path.to_owned()
                    } else {
                        Default::default()
                    }
                }),
                entry.line.map(|entry_line| {
                    if line {
                        entry_line.to_string()
                    } else {
                        Default::default()
                    }
                }),
                entry.column.map(|entry_column| {
                    if column {
                        entry_column.to_string()
                    } else {
                        Default::default()
                    }
                }),
            ];
            // We print all available properties of an entry, separated by newlines,
            // if the user didn't specify the `--path`, `--line`, `--column`
            // If they did, only those properties are printed, in a predetermined order
            let result = joined.into_iter().flatten().collect::<Vec<_>>().join("\n");
            print!("{result}");
            Ok(())
        },
        Action::Update {
            section: parent,
            register: child,
            path,
            line,
            column,
        } => {
            if path.is_none() && line.is_none() && column.is_none() {
                return Err("specify at least one of `--path`, `--line`, `--column`".into());
            }
            let parent_entry = model.entry(parent).or_default();
            let child_entry = parent_entry.entry(child).or_default();
            if let Some(path) = path {
                child_entry.path = Some(path);
            }
            if let Some(line) = line {
                child_entry.line = Some(line);
            }
            if let Some(column) = column {
                child_entry.column = Some(column);
            }
            save(&model, &harp_data_file)
        },
        Action::Clear { section: parent, register: child } => {
            if let Some(child) = child {
                if let Some(parent_map) = model.get_mut(&parent) {
                    if parent_map.remove(&child).is_some() {
                        save(&model, &harp_data_file)?;
                    };
                };
            } else if model.remove(&parent).is_some() {
                save(&model, &harp_data_file)?;
            }
            // Because our intention is to clear an entry,
            // if we don't find one, we still succeed (because the entry is already "cleared")
            // This is also why we don't *always* save: we don't always need to.
            // This would usually be too anal of a decision, but because our data is a hashmap of
            // hashmaps, it's not ordered, and modifying the data file basically randomly is
            // wack, since the user might have it under version control.
            // If nothing changed, but there are diffs all over the place, you're more likely to
            // just trust that there was a change, which might not be the case.
            // I could also use IndexMap to make the diffs clean, but that introduces a dependency.
            // If you believe it's worth it, open an issue or PR the change directly.
            Ok(())
        },
    }
}

fn save(model: &Entries, file: &Path) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().write(true).truncate(true).open(file)?;
    file.write_all(
        serde_yaml::to_string(&model)
            .map_err(|_| "couldn't serialize data model")?
            .as_bytes(),
    )?;
    Ok(())
}
