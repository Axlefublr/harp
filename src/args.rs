use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    pub quiet:  bool,
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand)]
pub enum Action {
    Clear {
        parent: String,
        child:  Option<String>,
    },
    Get {
        parent: String,
        child:  String,
        #[arg(short, long)]
        path:   bool,
        #[arg(short, long)]
        line:   bool,
        #[arg(short, long)]
        column: bool,
    },
    Update {
        parent: String,
        child:  String,
        #[arg(short, long)]
        path:   Option<String>, // this is not PathBuf…
        #[arg(short, long)]
        line:   Option<i32>, // …and this not u32, to let the user abuse `harp` and be able to store
        // more varied data, for example a relative line or a register of text they want to keep.
        // There's no particular reason we should be storing a string a two numbers anyway.
        #[arg(short, long)]
        column: Option<i32>,
    },
}
