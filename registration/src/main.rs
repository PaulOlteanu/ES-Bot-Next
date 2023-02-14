extern crate dotenv;

use std::env;

use clap::{Parser, Subcommand};
use dotenv::dotenv;
use twilight_model::application::command::Command;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    prod: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Register,

    List,

    Delete {
        #[arg(short, long)]
        command_id: u64,
    },

    DeleteAll,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Cli::parse();

    let application_id = env::var("DISCORD_APPLICATION_ID")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let bot_token = env::var("DISCORD_BOT_TOKEN").unwrap();

    let url = if cli.prod {
        format!("https://discord.com/api/v10/applications/{application_id}/commands")
    } else {
        let test_guild_id = 781256007430307870u64;
        format!("https://discord.com/api/v10/applications/{application_id}/guilds/{test_guild_id}/commands")
    };

    let client = reqwest::Client::new();

    match cli.command {
        Commands::Register => {
            let command_specs = es_bot::commands::command_specs();

            let res = client
                .put(url)
                .header("Authorization", format!("Bot {}", bot_token))
                .json(&command_specs)
                .send()
                .await;

            if let Ok(res) = res {
                println!("Success");
                println!(
                    "{}",
                    serde_json::to_string_pretty(&res.json::<Vec<Command>>().await.unwrap())
                        .unwrap()
                );
            } else if let Err(err) = res {
                println!("Failure");
                println!("{}", err);
            }
        }

        Commands::List => {
            let res = client
                .get(url)
                .header("Authorization", format!("Bot {}", bot_token))
                .send()
                .await;

            if let Ok(res) = res {
                println!("Success");
                println!(
                    "{}",
                    serde_json::to_string_pretty(&res.json::<Vec<Command>>().await.unwrap())
                        .unwrap()
                );
            } else if let Err(err) = res {
                println!("Failure");
                println!("{}", err);
            }
        }

        Commands::Delete { command_id } => {
            let url = format!("{url}/{command_id}");
            let res = client
                .delete(url)
                .header("Authorization", format!("Bot {}", bot_token))
                .send()
                .await;

            if let Ok(res) = res {
                if res.status().is_success() {
                    println!("Success");
                } else {
                    println!("Failure");
                }
                println!("{:?}", res);
            } else if let Err(err) = res {
                println!("Failure");
                println!("{}", err);
            }
        }

        Commands::DeleteAll => {
            let res = client
                .put(url)
                .header("Authorization", format!("Bot {}", bot_token))
                .json(&Vec::<bool>::new())
                .send()
                .await;

            if let Ok(res) = res {
                if res.status().is_success() {
                    println!("Success");
                } else {
                    println!("Failure");
                    println!("{}", res.text().await.unwrap());
                }
            } else if let Err(err) = res {
                println!("Failure");
                println!("{}", err);
            }
        }
    }
}
