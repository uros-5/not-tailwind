use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct NtwArgs {
    /// Build for production. Specify in which files to search.
    #[arg(long, short, num_args(1..))]
    pub run: Vec<String>,

    /// Build TypeScript map
    #[arg(long)]
    pub build_ts_map: bool,

    /// Specify output directory(default directory is not-tw). Make sure it is in .gitignore
    #[arg(short, long)]
    pub output: Option<String>,

    /// CSS files to ignore
    #[arg(long, short, num_args(1..))]
    pub ignored: Option<Vec<String>>,
}
