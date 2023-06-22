use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    process::Stdio,
};

use clap::{arg, Command, Parser};
use config::Client;
use dll_syringe::{process::OwnedProcess, Syringe};
use log::{error, info, warn};

use crate::config::app_directory;

mod config;
mod logger;
mod process;
mod utils;

#[derive(Parser)]
struct Cli {
    launch_type: Option<String>,
    path: std::path::PathBuf,
    username: String,
    option: Option<String>,
}

fn cli() -> Command {
    Command::new("instigator")
        .about("Open-source Fortnite launcher, built in Rust.")
        .author("jacksta <jacksta@pm.me>")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .subcommand(
            Command::new("start")
                .about("Launch game via a saved client")
                .args(&[arg!(name: "name of the client to launch"), arg!(-t --launch_type [TYPE] "client/server default: client")])
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("custom")
                .about("Launch a game via arguments")
                .args(
                    &[
                        arg!(path: "game path, make sure it includes FortniteGame and Engine folders."),
                        arg!(-u --username [USERNAME] "the username to launch the game with"),
                        arg!(-p --password [PASSWORD] "the password to launch the game with"),
                        arg!(-t --launch_type [TYPE] "client/server default: client"),
                    ]
                )
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("add")
                .about("Add a launch config")
                .args(
                    &[
                        arg!(name: "the name to save the client under"),
                        arg!(path:  "game path, make sure it includes FortniteGame and Engine folders."),
                        arg!(-u --username <USERNAME> "the username to launch the game with"),
                        arg!(-t --launch_type <TYPE> "client/server default: client"),
                    ]
                )
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("remove")
                .about("Remove a launch config")
                .args(&[arg!(name: "the name of the client to remove")])
        )
        .subcommand(Command::new("list").about("List all added clients"))
        .subcommand(Command::new("install").about("Prepare Instigator for use"))
}

#[allow(dead_code)]
fn push_args() -> Vec<clap::Arg> {
    vec![arg!(-m --message <MESSAGE>)]
}

fn main() {
    logger::logger_init().expect("Failed to initialize logger.");

    println!(
        r"
 _           _   _             _
(_)         | | (_)           | |
 _ _ __  ___| |_ _  __ _  __ _| |_ ___  _ __
| | '_ \/ __| __| |/ _` |/ _` | __/ _ \| '__|
| | | | \__ \ |_| | (_| | (_| | || (_) | |
|_|_| |_|___/\__|_|\__, |\__,_|\__\___/|_|
                    __/ |
                   |___/                      v{}
    ",
        env!("CARGO_PKG_VERSION")
    );

    config::app_directory();

    utils::run_bwc_checks();

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("start", args)) => {
            let name = args.get_one::<String>("name").expect("No name found.");

            let config = config::get();
            let client = config
                .iter()
                .find(|client| client.name == name.to_string())
                .expect("No client found with that name.");

            start(client);
        }

        Some(("custom", args)) => {
            info!("Launching via command-line arguments.");

            let username = match args.get_one::<String>("username") {
                Some(value) => value.to_string(),
                None => "Player".to_string(),
            };

            let password = args.get_one::<String>("password");

            let path = args.get_one::<String>("path").expect("Path is missing.");

            let mut launch_type = match args.get_one::<String>("launch_type") {
                Some(value) => value.to_string(),
                None => "client".to_string(),
            };

            if !["server", "client", "headless"].contains(&launch_type.as_str()) {
                launch_type = "client".to_string();
            }

            start(
                &(Client {
                    name: "custom".to_string(),
                    username: username.to_string(),
                    path: path.to_string().into(),
                    launch_type: Some(launch_type.to_string()),
                    password: password.cloned(),
                }),
            );
        }

        Some(("add", args)) => {
            let path = PathBuf::from(args.get_one::<String>("path").expect("No path found."));
            let name = args.get_one::<String>("name").expect("No name found.");

            let username = args
                .get_one::<String>("username")
                .expect("No username found.");

            let mut launch_type = match args.get_one::<String>("launch_type") {
                Some(value) => value.to_string(),
                None => "client".to_string(),
            };

            if !["server", "client", "headless"].contains(&launch_type.as_str()) {
                error!("Invalid launch type, defaulting to client.");
                launch_type = "client".to_string();
            }

            if !path.exists() {
                error!("Path does not exist.");
                std::process::exit(1);
            }

            if !["server", "client", "headless"].contains(&launch_type.as_str()) {
                launch_type = "client".to_string();
            }

            let mut clients = config::get();

            clients.iter_mut().for_each(|client| {
                if client.name == name.to_string() {
                    error!("Client name already exists.");
                    std::process::exit(1);
                }
            });

            let client = config::Client {
                path,
                launch_type: Some(launch_type.to_string()),
                username: username.to_string(),
                name: name.to_string(),
                password: None,
            };

            let mut clients = config::get();

            if clients.iter().any(|client| client.name == name.to_string()) {
                error!("Client name already exists.");
                std::process::exit(1);
            }

            clients.push(client);
            config::save(clients);

            info!("Added client: {}", name);
            info!(
                "You can start this by running `instigator.exe start {}`",
                name
            );
        }
        Some(("remove", args)) => {
            let name = args.get_one::<String>("name").expect("No name found.");
            let mut clients = config::get();
            clients.retain(|client| client.name != name.to_string());
            config::save(clients);

            info!("Removed client: {}", name);
        }

        Some(("install", _args)) => {
            std::process::Command::new("explorer")
                .arg(app_directory())
                .spawn()
                .expect("Could not open File Explorer. Please navigate to %appdata%\\instigator manually.");

            info!("Opened Instigator directory.\n");
            println!(
                "Please add your console, SSL bypass/redirect, and game server libraries. Make sure they are named console.dll, redirect.dll and server.dll respectively."
            );
            println!("Set-up complete. Please add a client by running `instigator.exe add`");
        }

        Some(("list", _args)) => {
            let clients = config::get();
            for client in clients {
                println!(
                    "{} - {} - {} - {}",
                    client.name,
                    client.path.to_str().unwrap(),
                    client.username,
                    client.launch_type.unwrap()
                );
            }
        }

        Some((ext, _args)) => {
            error!("Unknown subcommand: {}", ext);
        }
        _ => unreachable!(),
    }
}

fn inject(path: PathBuf, pid: u32) {
    let target_process = OwnedProcess::from_pid(pid);
    let syringe = Syringe::for_process(target_process.unwrap());

    syringe.inject(path.as_path()).unwrap();
}

fn start(client: &Client) {
    let mut game_path = PathBuf::from(&client.path);
    game_path.push("FortniteGame\\Binaries\\Win64\\FortniteClient-Win64-Shipping.exe");

    process::start_ac(&client.path);
    process::start_launcher(&client.path);

    let user_arg = &format!("-AUTH_LOGIN={}@localhost", client.username);

    let pass_arg = &format!(
        "-AUTH_PASSWORD={}",
        client.password.clone().unwrap_or("null".to_string())
    );

    let fort_args = vec![
        "-epicapp=Fortnite",
        "-epicenv=Prod",
        "-epiclocale=en-us",
        "-epicportal",
        "-skippatchcheck",
        "-fromfl=eac",
        "-nobe",
        "-fltoken=3c836951cd605a77bc8132f4",
        user_arg,
        pass_arg,
        "-AUTH_TYPE=epic", // TO-DO: add caldera.
    ];

    let mut cmd = std::process::Command::new(game_path)
        .args(&fort_args)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let pid: u32 = cmd.id();
    let stdout = cmd.stdout.as_mut().unwrap();
    let stdout_reader = BufReader::new(stdout);
    let stdout_lines = stdout_reader.lines();

    let redirect: PathBuf = [
        dirs::config_dir().unwrap().to_str().unwrap().to_owned(),
        "instigator".to_owned(),
        "redirect.dll".to_string(),
    ]
    .iter()
    .collect();

    if redirect.exists() {
        info!("Injecting redirect library.");
        inject(redirect, pid);
    } else {
        warn!("No redirect library not found. Make sure you are using Fiddler or similar.");
    }

    for line in stdout_lines {
        let line = line.unwrap();
        info!("{}", line);

        // We are looking for a safe state to unlock console.
        // If this is too early or too late for some versions let me know :).
        if line.contains("LogInit: Display: Starting Game.") {
            let console: PathBuf = [
                dirs::config_dir().unwrap().to_str().unwrap().to_owned(),
                "instigator".to_owned(),
                "console.dll".to_string(),
            ]
            .iter()
            .collect();

            let server: PathBuf = [
                dirs::config_dir().unwrap().to_str().unwrap().to_owned(),
                "instigator".to_owned(),
                "server.dll".to_string(),
            ]
            .iter()
            .collect();

            if client.launch_type.clone().unwrap() != "server" {
                if console.exists() {
                    info!("Injecting console unlock.");
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    inject(console, pid);
                } else {
                    warn!("No console library not found. You will have to inject this manually.");
                }
            } else {
                if server.exists() {
                    info!("Injecting headed server library.");
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    inject(server, pid);
                } else {
                    warn!(
                        "No game server library not found. You will have to inject this manually."
                    );
                }
            }
        }
    }
    cmd.wait().unwrap();
    process::kill_all();

    info!("Game closed, Instigator cleaning up. ");
}
