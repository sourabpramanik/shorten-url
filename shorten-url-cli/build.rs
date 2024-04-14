use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Seek, SeekFrom, Write};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    database_url: String,
}

fn main() {
    let config_dir = match dirs::config_dir() {
        Some(dir) => dir.join("shortenurl"),
        None => {
            let home = dirs::home_dir().expect("Failed to get home directory");
            home.join(".config/shortenurl")
        }
    };

    fs::create_dir_all(&config_dir)
        .expect("Failed to create directory for shortenurl in config directory");

    let config_file_path = config_dir.join("shortenurl.toml");
    let mut config_file = fs::File::create(&config_file_path)
        .expect("Failed to create configuration file shortenurl.toml");
    if config_file.seek(SeekFrom::End(0)).unwrap() == 0 {
        let config = Config {
            database_url: String::new(),
        };
        let config_str = toml::to_string(&config).expect("Could not encode TOML value");
        config_file
            .write_all(config_str.as_bytes())
            .expect("Could not write to file!");
    }

    println!("Configuration file created at: {:?}", config_file_path);
}
