use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Client {
    pub(crate) path: PathBuf,
    pub(crate) name: String,
    pub(crate) username: String,
    //#[serde(rename = "launchType")]
    //pub(crate) launch_type: String,
}

pub fn get() -> Vec<Client> {
    let mut file = File::open(path()).expect("Could not open config.json");

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let clients: Vec<Client> = serde_json::from_str(&contents).unwrap();

    clients
}

// save client to file, save to existing file if it exists else create it
pub fn save(clients: Vec<Client>) {
    let mut file = File::create(path()).expect("Could not create config.json");

    let contents = serde_json::to_string_pretty(&clients).unwrap();

    file.write_all(contents.as_bytes()).unwrap();
}

pub fn path() -> PathBuf {
    let mut path = app_directory();

    path.push("config.json");

    if !Path::exists(&path) {
        let mut file = File::create(&path).unwrap();
        file.write_all("[]".as_bytes()).unwrap();
    }

    path
}

pub fn app_directory() -> PathBuf {
    let mut path = PathBuf::from(dirs::config_dir().unwrap().to_str().unwrap().to_owned());

    path.push("instigator");

    if !Path::exists(&path) {
        std::fs::create_dir(&path).unwrap();
    }

    path
}
