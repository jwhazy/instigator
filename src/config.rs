use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Client {
    pub(crate) path: PathBuf,
    pub(crate) name: String,
    pub(crate) username: String,
    #[serde(rename = "launchType")]
    pub(crate) launch_type: Option<String>,
}

pub fn get() -> Vec<Client> {
    let mut file = File::open(path()).expect("Could not open config.json");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read config.json.");

    let clients: Vec<Client> =
        serde_json::from_str(&contents).expect("Could not parse config.json. Is it JSON?");

    clients
}

pub fn save(clients: Vec<Client>) {
    let mut file = File::create(path()).expect("Could not create config.json");

    let contents = serde_json::to_string_pretty(&clients).expect("Could not serialize clients.");

    file.write_all(contents.as_bytes())
        .expect("Could not write to config.json");
}

pub fn path() -> PathBuf {
    let mut path = app_directory();

    path.push("config.json");

    if !Path::exists(&path) {
        let mut file = File::create(&path).expect("Config does not exist.");
        file.write_all("[]".as_bytes())
            .expect("Failed to write starter config.");
    }

    path
}

pub fn app_directory() -> PathBuf {
    let mut path = PathBuf::from(dirs::config_dir().unwrap().to_str().unwrap().to_owned());

    path.push("instigator");

    if !Path::exists(&path) {
        std::fs::create_dir(&path).expect("Could not create instigator directory.");
    }

    path
}
