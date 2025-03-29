use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use log::error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Client {
    pub(crate) path: PathBuf,
    pub(crate) name: String,
    pub(crate) username: String,
    pub(crate) email: Option<String>,
    #[serde(rename = "launchType")]
    pub(crate) launch_type: Option<String>,
    pub(crate) password: Option<String>,
}

pub fn get() -> Vec<Client> {
    let mut file = match File::open(path()) {
        Ok(file) => file,
        Err(e) => {
            error!("Could not open config.json: {}", e);
            std::process::exit(1);
        }
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        error!("Could not read config.json: {}", e);
        std::process::exit(1);
    }

    let clients: Vec<Client> = match serde_json::from_str(&contents) {
        Ok(clients) => clients,
        Err(e) => {
            error!("Could not parse config.json. Is it JSON? Error: {}", e);
            std::process::exit(1);
        }
    };

    return clients;
}

pub fn save(clients: Vec<Client>) {
    let mut file = match File::create(path()) {
        Ok(file) => file,
        Err(e) => {
            error!("Could not create config.json: {}", e);
            std::process::exit(1);
        }
    };

    let contents = match serde_json::to_string_pretty(&clients) {
        Ok(contents) => contents,
        Err(e) => {
            error!("Could not serialize clients: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = file.write_all(contents.as_bytes()) {
        error!("Could not write to config.json: {}", e);
        std::process::exit(1);
    }
}

pub fn path() -> PathBuf {
    let mut path = app_directory();

    path.push("config.json");

    if !Path::exists(&path) {
        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(e) => {
                error!("Config does not exist and could not be created: {}", e);
                std::process::exit(1);
            }
        };
        
        if let Err(e) = file.write_all("[]".as_bytes()) {
            error!("Failed to write starter config: {}", e);
            std::process::exit(1);
        }
    }

    return path;
}

pub fn app_directory() -> PathBuf {
    let config_dir = match dirs::config_dir() {
        Some(dir) => dir,
        None => {
            error!("Could not determine config directory");
            std::process::exit(1);
        }
    };
    
    let dir_str = match config_dir.to_str() {
        Some(s) => s.to_owned(),
        None => {
            error!("Config directory path contains invalid characters");
            std::process::exit(1);
        }
    };
    
    let mut path = PathBuf::from(dir_str);

    path.push("instigator");

    if !Path::exists(&path) {
        if let Err(e) = std::fs::create_dir(&path) {
            error!("Could not create instigator directory: {}", e);
            std::process::exit(1);
        }
    }

    return path;
}
