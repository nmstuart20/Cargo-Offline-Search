use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "offline-search")]
#[command(about = "Search a local cargo registry for crates")]
#[command(version, disable_version_flag = true)]
pub struct Args {
    /// Search query (partial crate name match, case-insensitive)
    pub query: String,

    /// Maximum number of results to display
    #[arg(short, long)]
    pub limit: Option<usize>,

    /// Registry name to search (from [registries] in .cargo/config.toml)
    #[arg(short, long, conflicts_with_all = ["url", "repo"])]
    pub registry: Option<String>,

    /// SonaType Nexus URL for remote search (requires --repo)
    #[arg(long, requires = "repo", conflicts_with = "registry")]
    pub url: Option<String>,

    /// Repository name in SonaType Nexus (required with --url)
    #[arg(long, requires = "url")]
    pub repo: Option<String>,

    /// Filter results by version (exact match)
    #[arg(short = 'v', long = "version-filter", id = "version_filter")]
    pub version: Option<String>,
}
