use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Authorizer url to operate on
    url: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Invite users to app
    InviteMembers {
        /// Sets path for input file
        #[arg(short, long, value_name = "FILE")]
        path: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(url) = cli.url.as_deref() {
        println!("Value for url: {}", url);
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => (),
        _ => println!("Debug mode is on"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::InviteMembers { path }) => {
            let content = std::fs::read_to_string(&path).expect(&format!("could not read file `{}`", &path.display()));
            for line in content.lines() {
                println!("{}", line);
            };
        }
        None => {}
    }

    // Continued program logic goes here...
}