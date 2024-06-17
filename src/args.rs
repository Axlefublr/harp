use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(version, long_about = std::include_str!("help.txt"))]
pub struct Args {
    /// Don't print error messages (while still exiting with a non-zero exitcode in case of error).
    /// Useful for when the program where you want to use `harp` in makes it difficult to
    /// differentiate between successful stdout and unsuccessful stderr.
    #[arg(short, long)]
    pub quiet:  bool,
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand)]
pub enum Action {
    /// If REGISTER is specified, it's completely removed.
    /// If it isn't, the entire SECTION is removed instead.
    Clear {
        section:  String,
        register: Option<String>,
    },
    /// Print all available properties of a REGISTER in the order: path, line, column.
    /// Only the properties you specified with the `--path`, `--line`, `--column` flags
    /// are printed.
    /// At least one of those flags needs to be specified.
    Get {
        section:  String,
        register: String,
        #[arg(short, long)]
        path:     bool,
        #[arg(short, long)]
        line:     bool,
        #[arg(short, long)]
        column:   bool,
    },
    /// Update properties of a register, or create one.
    /// At least one of `--path`, `--line`, `--column` has to be specified.
    Update {
        section:  String,
        register: String,
        #[arg(short, long)]
        #[clap(allow_hyphen_values = true)]
        path:     Option<String>, // this is not PathBuf…
        #[arg(short, long)]
        #[clap(allow_hyphen_values = true)]
        line:     Option<i32>, // …and this not u32, to let the user abuse `harp` and be able to store
        // more varied data, for example a relative line or a register of text they want to keep.
        // There's no particular reason we should be storing a string a two numbers anyway.
        #[arg(short, long)]
        #[clap(allow_hyphen_values = true)]
        column:   Option<i32>,
    },
}
