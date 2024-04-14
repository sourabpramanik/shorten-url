use sqlx::{postgres::PgPoolOptions, Pool};

use crate::Config;
use std::{
    fs,
    io::Write,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn get_config() -> Config {
    let config_dir = dirs::config_dir().expect("Missing config dir");
    let config_file_path = config_dir.join("shortenurl").join("shortenurl.toml");
    let content: String = fs::read_to_string(&config_file_path)
        .unwrap()
        .parse()
        .unwrap();
    let config: Config = toml::from_str(&content).expect("Failed to read the config file");
    config
}

pub fn write_config(new_config: &Config) {
    let config_dir = dirs::config_dir().expect("Missing config dir");
    let config_file_path = config_dir.join("shortenurl").join("shortenurl.toml");
    let content = toml::to_string(new_config).expect("Could not encode TOML value");
    let mut config_file = fs::File::create(&config_file_path)
        .expect("Failed to create configuration file shortenurl.toml");

    config_file
        .write_all(content.as_bytes())
        .expect("Could not write to file!");
}

pub async fn get_db_pool_connection() -> Pool<sqlx::Postgres> {
    let config = get_config();
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => pool,
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            println!("Try configuring againa using command: shortenurl config");
            std::process::exit(1);
        }
    };
    pool
}

pub fn unique_id() -> String {
    let now = SystemTime::now();
    let unix_tp = now
        .duration_since(UNIX_EPOCH)
        .expect("Something went wrong in generating alias UID!")
        .as_millis();
    let alias_str = base62::encode(unix_tp).to_string();
    return alias_str;
}
