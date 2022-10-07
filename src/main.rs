mod utils;
mod graphql;
mod prompt;

use std::{path::PathBuf, io::copy, fs::File};
use clap::{Parser, Subcommand};
use prompt::take_user_input;
use serde_json::Value;
use graphql::send_invitation;
use utils::{get_extension_from_filename, validate_path};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // used to pass url as argument
    // /// Authorizer url to operate on
    // url: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Subcommand)]
enum Commands {
    /// Invite users to app [--file-path <PATH>]
    InviteMembers {
        /// Sets path for input file
        #[arg(short, long, value_name = "FILE", alias = "file-path")]
        path: PathBuf,
    },
    /// Download samples [--file-path <PATH>]
    DownloadSamples {
        /// Sets path for input file
        #[arg(short, long, value_name = "FILE", alias = "file-path")]
        path: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let mut is_debug_mode_on = false;

    let cli = Cli::parse();

    // Set the value of authorizer url if passed as an argument
    // if let Some(url) = cli.url.as_deref() {
    //     authorizer_url = url;
    // }

    // check if debug flag was set
    match cli.debug {
        0 => (),
        _ => {
            is_debug_mode_on = true;
            println!("Debug mode is ON");
        },
    }

    // execute commands
    match &cli.command {
        Some(Commands::InviteMembers { path }) => {
            let mut user_emails = vec![];
            let path_str = &path.as_path().display().to_string();
            let file_extension = get_extension_from_filename(path_str);
            match file_extension {
                Some(ext) => {
                    if !validate_path(path) {
                        Err("Invalid path!")?;
                    }
                    match ext {
                        "csv" => {
                            let mut reader = csv::ReaderBuilder::new()
                            .has_headers(false)
                            .from_path(&path_str)?;
                            for record in reader.records() {
                                for email in record?.into_iter() {
                                    user_emails.push(Value::String(email.to_string()));
                                }
                            }
                        },
                        "txt" => {
                            let content = std::fs::read_to_string(&path_str)?;
                            for email in content.lines() {
                                user_emails.push(Value::String(email.to_string()));
                            };
                        },
                        _ => {
                            Err("Unsupported file format!")?;
                        },
                    }
                },
                None => {
                    Err("Invalid file!")?;
                },
            }
            let user_inputs = take_user_input();
            match user_inputs {
                Ok(data) => {
                    send_invitation(&data[0], &data[1], user_emails, is_debug_mode_on).await?;
                },
                Err(error) => {
                    Err(error.to_string())?;
                },
            }
        }
        Some(Commands::DownloadSamples { path }) => {
            let mut csv_target_path = path.to_owned();
            let mut txt_target_path = path.to_owned();
            csv_target_path.push("authorizer_sample.csv");
            txt_target_path.push("authorizer_sample.txt");

            let csv_sample_response = reqwest::get("https://gist.githubusercontent.com/anik-ghosh-au7/a0ec5bc432594438adc3e6f6fb42e520/raw/authorizer_invite_members_sample.csv").await?;
            let txt_sample_response = reqwest::get("https://gist.githubusercontent.com/anik-ghosh-au7/8a518f9ed11f2475087c5e0b22e67f80/raw/authorizer_invite_members_sample.txt").await?;

            let mut csv_output = File::create(csv_target_path)?;
            let mut txt_output = File::create(txt_target_path)?;

            let csv_sample_content =  csv_sample_response.text().await?;
            let txt_sample_content =  txt_sample_response.text().await?;

            copy(&mut csv_sample_content.as_bytes(), &mut csv_output)?;
            copy(&mut txt_sample_content.as_bytes(), &mut txt_output)?;

            println!("Success: sample files download complete",);
        }
        None => {}
    }
    Ok(())
}