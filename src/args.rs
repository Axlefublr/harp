use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    pub parent: String,
    pub child: String,
    #[arg(short, long)]
    pub path: Option<PathBuf>,
    #[arg(short, long)]
    pub line: Option<u32>,
    #[arg(short, long)]
    pub column: Option<u32>,
}
