use clap::Parser;
use clap::Subcommand;

/// A section is the highest level key, that stores a bunch of registers inside of it.
/// A register holds an array of values.
#[derive(Parser)]
#[command(version)]
pub struct Args {
    /// Don't print error messages (while still exiting with a non-zero exitcode in case of error).
    /// Useful for when the program where you want to use `harp` in makes it difficult to
    /// differentiate between successful stdout and unsuccessful stderr.
    #[arg(short, long)]
    pub quiet: bool,
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand)]
pub enum Action {
    /// Print the values of a register, separated by newlines.
    Get {
        section: String,
        register: String,
        /// Separate values with null byte instead.
        #[arg(short = '0', long)]
        null: bool,
    },
    /// Replace all the values of a register with some other ones.
    Replace {
        section: String,
        register: String,
        #[clap(allow_hyphen_values = true)]
        values: Vec<String>,
    },
}
