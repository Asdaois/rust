use clap::Parser;

#[derive(Parser)]
#[command(author = "Jose Guevara", version = "0.0.1", about = "Pattern Engine")]
pub struct Cli {
    /// The pattern to search for
    #[arg(required = true, short)]
    pub pattern: String,

    /// The file to search
    #[arg(required = true, short)]
    pub file_path: String,
}

impl Cli {
    pub fn new() -> Cli {
        Cli::parse()
    }
}
