use std::error::Error;
use std::process::ExitCode;

use axleharp::HarpReady;
use clap::Parser;

use crate::args::Action;
use crate::args::Args;

mod args;

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
    let mut harp = HarpReady::build()?;
    match action {
        Action::Get {
            section,
            register,
            path,
            line,
            column,
            extra,
        } => {
            let entry = harp.get(&section, &register, path, line, column, extra)?;
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
                entry.extra.as_ref().map(|entry_column| {
                    if column {
                        entry_column.to_string()
                    } else {
                        Default::default()
                    }
                }),
            ];
            // Considering that we guaranteed the user specified at *least* one flag,
            // We print all passed and available properties, in the order of path, line, column, extra.
            // (Order is the same regardless of the order of flags)
            let result = joined.into_iter().flatten().collect::<Vec<_>>().join("\n");
            print!("{result}");
            Ok(())
        },
        Action::Update {
            section,
            register,
            path,
            line,
            column,
            extra,
        } => harp.update(section, register, path, line, column, extra),
        Action::Clear { section, register } => harp.clear(section, register),
    }
}
