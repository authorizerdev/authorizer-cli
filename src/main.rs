use std::{path::PathBuf};
use clap::{Parser, Subcommand};
use reqwest::Client;
use serde_json::{Value, Map};

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

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

async fn send_invitation() -> Result<()> {
    const SEND_INVITAION_MUTATION: &str = "mutation inviteMembers($params: InviteMemberInput!) {\n  _invite_members(params: $params) {\n    message\n    __typename\n  }\n}";
    let client = Client::new();
    let mut emails_map = Map::new();
    emails_map.insert("emails".to_string(), Value::Array(vec![
        Value::String("anik800@authorizer.dev".to_string()),
        Value::String("anik900@authorizer.dev".to_string()),
    ]));
    let mut params_map = Map::new();
    params_map.insert("params".to_string(), Value::Object(emails_map));
    let mut map = Map::new();
    map.insert("query".to_string(), Value::String(SEND_INVITAION_MUTATION.to_string()));
    map.insert("variables".to_string(), Value::Object(params_map));
    let req = client
        .post("http://localhost:8080/graphql")
        .header("x-authorizer-admin-secret", "admin")
        .json(&map);

    let res = req.send().await?;
    println!("request status: {}", res.status());

    let body = res.bytes().await?;

    let v = body.to_vec();
    let s = String::from_utf8_lossy(&v);
    println!("response: {} ", s);

    Ok(())
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

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
            send_invitation().await?;
        }
        None => {}
    }
    Ok(())
}