use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "offline-search")]
#[command(about = "Search a local cargo registry for crates")]
#[command(version)]
pub struct Args {
    /// Search query (partial crate name match, case-insensitive)
    pub query: String,

    /// Maximum number of results to display
    #[arg(short, long)]
    pub limit: Option<usize>,
}
