use anyhow::Ok;
use clap::Parser;
use anyhow::{Context, Result};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}


fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
    .with_context(|| format!("could not read file `{}`", &args.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    };
    Ok(())
}
