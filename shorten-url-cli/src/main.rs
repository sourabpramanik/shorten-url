mod utils;

use core::time;
use std::thread::sleep;

use clap::{Parser, Subcommand};
use inquire::Text;
use serde::{Deserialize, Serialize};
use sqlx::{self, prelude::FromRow};
use tabled::{builder::Builder, settings::Style};
use utils::{get_db_pool_connection, unique_id};
use uuid::Uuid;

use crate::utils::{get_config, write_config};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    database_url: String,
    domain: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct AliasSchema {
    id: Uuid,
    alias: String,
    url: String,
}

#[derive(Debug, Parser)]
#[command(name = "shortenurl")]
#[command(about = "A personal url shortening CLI manager.")]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    // Configure Tsuro
    #[command(about = "Configure the database.")]
    Config,
    #[command(about = "Command to get/create/delete an alias.")]
    #[command(subcommand)]
    Alias(AliasArgs),
}

#[derive(Parser, Debug)]
enum AliasArgs {
    /// Get alias for the provided URL if exists.
    #[command(about)]
    Get { alias: String },

    /// Lists all the URLS with thier respective aliases.
    #[command(about)]
    GetALL,

    /// Creates an alias and stores the record in database if not existing.
    #[command(about)]
    Create { url: String },

    /// Removes the record from the database for the provided URL if exists.
    #[command(about)]
    RemoveAlias { alias: String },

    /// Flushes the database
    #[command(about)]
    Flush,
}

#[tokio::main]
async fn main() {
    let cli_args = CLI::parse();
    match cli_args.command {
        Commands::Config => configure().await,
        Commands::Alias(args) => match args {
            AliasArgs::Get { alias } => {
                get_url(&alias).await;
            }
            AliasArgs::GetALL => {
                get_all_record().await;
            }
            AliasArgs::Create { url } => {
                create_alias(&url).await;
            }
            AliasArgs::RemoveAlias { alias } => remove_record(&alias).await,
            AliasArgs::Flush => {
                flush_records().await;
            }
        },
    }
}

async fn configure() {
    let database_url = Text::new("Provide postgres connection string:").prompt();
    let domain = Text::new("Provide primary domain configured in Vercel(e.g. foo.com):").prompt();
    match (database_url, domain) {
        (Ok(database_url), Ok(domain)) => {
            println!("Sourcing configurations");
            let config = Config {
                database_url,
                domain,
            };
            write_config(&config);
            println!("Migration started");
            sleep(time::Duration::from_millis(2000));
            run_migrations().await
        }
        (Ok(_), Err(_)) => {
            println!("Please provide correct domain e.g. foo.com");
            std::process::exit(1);
        }
        (Err(_), Ok(_)) => {
            println!("Please provide correct postgres connection string");
            std::process::exit(1);
        }
        (Err(_), Err(_)) => {
            println!(
                "Something went wrong!! Either try again or create an issue in GitHub: some_url"
            );
            std::process::exit(1);
        }
    }
}

async fn run_migrations() {
    let pool = get_db_pool_connection().await;
    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => {
            println!("Migration complete.");
        }
        Err(err) => {
            println!("Migration failed: {}", err);
            std::process::exit(1);
        }
    };
}

async fn get_url(alias: &str) {
    let pool = get_db_pool_connection().await;
    match sqlx::query_as::<_, AliasSchema>(&format!(
        "SELECT * FROM aliases WHERE alias='{}'",
        alias
    ))
    .fetch_one(&pool)
    .await
    {
        Ok(record) => {
            println!("Long URL: {}", record.url);
            println!(
                "Short URL: https://{}/{}",
                &get_config().domain,
                record.alias
            )
        }
        Err(_) => {
            println!("Alias does not exists.");
        }
    };
}

async fn get_all_record() {
    let pool = get_db_pool_connection().await;
    match sqlx::query_as::<_, AliasSchema>(&format!("SELECT * FROM aliases"))
        .fetch_all(&pool)
        .await
    {
        Ok(records) => {
            //table of records
            let mut builder = Builder::new();
            let mut serial = 1;
            builder.push_record(["S.No.", "Short URL", "Long URL"]);
            for record in records {
                builder.push_record([
                    serial.to_string(),
                    format!(
                        "https://{}/{}",
                        &get_config().domain,
                        record.alias.to_string()
                    ),
                    record.url.to_string(),
                ]);
                serial = serial + 1;
            }

            let table = builder.build().with(Style::rounded()).to_string();
            println!("{table}")
        }
        Err(_) => {
            println!("No records found, please create one using command: shortenurl alias create <your url>.");
        }
    };
}

async fn create_alias(url: &str) {
    let pool = get_db_pool_connection().await;
    let alias_str = unique_id();

    match sqlx::query_as::<_, AliasSchema>(&format!(
        "INSERT INTO aliases (alias, url) VALUES ('{}', '{}') RETURNING *",
        alias_str, url
    ))
    .fetch_one(&pool)
    .await
    {
        Ok(record) => {
            println!("Alias created successfully!!");
            println!("Short URL: http://your.domain.com/{}", record.alias)
        }
        Err(err) => {
            println!("Failed to create alias, try again!!!, {err}");
        }
    };
}

async fn remove_record(alias: &str) {
    let pool = get_db_pool_connection().await;
    match sqlx::query_as::<_, AliasSchema>(&format!(
        "DELETE FROM aliases WHERE alias='{}' RETURNING *",
        alias
    ))
    .fetch_one(&pool)
    .await
    {
        Ok(_) => {
            println!("Record was removed.");
        }
        Err(err) => {
            println!("Alias cannot be found. {err}");
        }
    };
}

async fn flush_records() {
    let pool = get_db_pool_connection().await;
    match sqlx::query_as::<_, AliasSchema>("DELETE FROM aliases RETURNING *")
        .fetch_one(&pool)
        .await
    {
        Ok(_) => {
            println!("All records were removed.");
        }
        Err(err) => {
            println!("Something wen wrong, {err}");
        }
    };
}
