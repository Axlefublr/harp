use std::process::ExitCode;

use anyhow::ensure;
use axleharp::HarpConnection;
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

fn _main(action: Action) -> anyhow::Result<()> {
    let mut harp = HarpConnection::build()?;
    match action {
        Action::Get {
            section,
            register,
            null,
        } => {
            let values = harp.entry_mut(section.clone(), register.clone());
            ensure!(
                !values.is_empty(),
                "section `{section}` register `{register}` is empty"
            );
            if null {
                print!("{}", values.join("\0"));
            } else {
                println!("{}", values.join("\n"));
            }
        },
        Action::Replace {
            section,
            register,
            mut values,
        } => {
            let old_values = harp.entry_mut(section, register);
            old_values.clear();
            old_values.append(&mut values);
            harp.save()?;
        },
    }
    Ok(())
}
