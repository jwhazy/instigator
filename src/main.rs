use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    process::Stdio,
    sync::atomic::AtomicBool
};

use clap::{arg, Command, Parser};
use config::Client;
use ctrlc;
use dll_syringe::{process::OwnedProcess, Syringe};
use log::{error, info, warn};

use crate::process::{kill_all, kill_fortnite};

mod config;
mod logger;
mod process;
mod utils;

static CTRL_C_PRESSED: AtomicBool = AtomicBool::new(false);

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
        .author("jacksta <me@jacksta.dev>")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .subcommand(
            Command::new("start")
                .about("Launch game via a saved client")
                .args(&[arg!(name: "name of the client to launch")])
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("custom")
                .about("Launch a game via arguments")
                .args(
                    &[
                        arg!(path: "game path, make sure it includes FortniteGame and Engine folders."),
                        arg!(-u --username [USERNAME] "the username to launch the game with"),
                        arg!(-e --email [EMAIL] "the email domain to launch the game with e.g. gmail.com or localhost"),
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
                        arg!(-e --email [EMAIL] "the email domain to launch the game with e.g. gmail.com or localhost"),
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
        .subcommand(Command::new("folder").about("Open the Instigator directory"))
        .subcommand(
            Command::new("install")
                .about("Prepare Instigator for use (deprecated, use 'folder' instead)")
                .hide(true)
        )
}

#[allow(dead_code)]
fn push_args() -> Vec<clap::Arg> {
    vec![arg!(-m --message <MESSAGE>)]
}

fn main() {
    logger::logger_init().expect("Failed to initialize logger.");

    ctrlc::set_handler(move || {
        if !CTRL_C_PRESSED.load(std::sync::atomic::Ordering::SeqCst) {
            info!("Received Ctrl+C, cleaning up processes...");
            kill_fortnite();
            kill_all();
            CTRL_C_PRESSED.store(true, std::sync::atomic::Ordering::SeqCst);
        }
    })
    .expect("Error setting Ctrl+C handler");

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
            let name = match args.get_one::<String>("name") {
                Some(name) => name,
                None => {
                    error!("No name found.");
                    std::process::exit(1);
                }
            };

            let config = config::get();
            let client = match config.iter().find(|client| client.name == name.to_string()) {
                Some(client) => client,
                None => {
                    error!("No client found with that name.");
                    std::process::exit(1);
                }
            };

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

            let email = match args.get_one::<String>("email") {
                Some(value) => value.to_string(),
                None => "localhost".to_string(),
            };

            if !["server", "client", "headless"].contains(&launch_type.as_str()) {
                launch_type = "client".to_string();
            }

            start(
                &(Client {
                    name: "custom".to_string(),
                    username: username.to_string(),
                    path: path.to_string().into(),
                    email: email.into(),
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

            let email = match args.get_one::<String>("email") {
                Some(value) => value.to_string(),
                None => "localhost".to_string(),
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
                email: Some(email.to_string()),
                name: name.to_string(),
                password: None,
            };

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
        Some(("kill", _)) => kill_all(),

        Some(("folder", _)) => utils::folder(),

        Some(("install", _)) => {
            warn!("The 'install' command is deprecated. Please use 'folder' instead.");
            utils::folder()
        }

        Some(("list", _args)) => {
            println!("name - path - username - email - launch type");
            let clients = config::get();
            for client in clients {
                println!(
                    "{} - {} - {} - {} - {}",
                    client.name,
                    client.path.to_str().unwrap_or("<invalid path>"),
                    client.username,
                    client.email.unwrap_or_else(|| "unknown".to_string()),
                    client.launch_type.unwrap_or_else(|| "unknown".to_string())
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
    let target_process = match OwnedProcess::from_pid(pid) {
        Ok(process) => process,
        Err(_) => {
            error!("Could not find process with PID: {}", pid);
            return;
        }
    };

    let syringe = Syringe::for_process(target_process);

    match syringe.inject(path.as_path()) {
        Ok(_) => info!("Successfully injected DLL"),
        Err(e) => error!("Failed to inject DLL: {}", e),
    }
}

fn start(client: &Client) {
    let mut game_path = PathBuf::from(&client.path);
    game_path.push("FortniteGame\\Binaries\\Win64\\FortniteClient-Win64-Shipping.exe");
    
    if !game_path.exists() {
        error!("Game path does not exist.");
        std::process::exit(1);
    }

    process::start_ac(&client.path);
    process::start_launcher(&client.path);

    let default_email = "localhost".to_string();
    let email = client.email.as_ref().unwrap_or(&default_email);
    
    let user_arg = format!("-AUTH_LOGIN={}@{}", client.username, email);
    let pass_arg = format!(
        "-AUTH_PASSWORD={}",
        client.password.as_deref().unwrap_or("null")
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
        &user_arg,
        &pass_arg,
        "-AUTH_TYPE=epic", // TO-DO: add caldera.
    ];

    let mut fortnite = match std::process::Command::new(&game_path)
        .args(&fort_args)
        .stdout(Stdio::piped())
        .spawn() {
            Ok(process) => process,
            Err(e) => {
                error!("Failed to start Fortnite process: {}", e);
                std::process::exit(1);
            }
        };

    let pid = fortnite.id();

    let stdout = match fortnite.stdout.take() {
        Some(stdout) => stdout,
        None => {
            error!("Failed to capture stdout");
            std::process::exit(1);
        }
    };

    let stdout_reader = BufReader::new(stdout);
    let stdout_lines = stdout_reader.lines();

    let config_dir = match dirs::config_dir() {
        Some(dir) => dir,
        None => {
            error!("Could not determine config directory");
            std::process::exit(1);
        }
    };

    let redirect: PathBuf = [
        config_dir.to_str().unwrap_or_default(),
        "instigator",
        "redirect.dll",
    ].iter().collect();

    if redirect.exists() {
        info!("Injecting redirect library.");
        inject(redirect, pid);
    } else {
        warn!("No redirect library found. You can ignore this message if you are using Fiddler or similar.");
    }

    for line in stdout_lines {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                error!("Failed to read stdout: {}", e);
                info!("Reading console failed. This can happen when a DLL unlocks the console.");
                continue;
            }
        };
        info!("{}", line);

        // We are looking for a safe state to unlock console.
        // If this is too early or too late for some versions let me know :).
        if line.contains("LogInit: Display: Starting Game.") {
            let console: PathBuf = [
                config_dir.to_str().unwrap_or_default(),
                "instigator",
                "console.dll",
            ].iter().collect();

            let server: PathBuf = [
                config_dir.to_str().unwrap_or_default(), 
                "instigator",
                "server.dll",
            ].iter().collect();

            let launch_type = client.launch_type.as_deref().unwrap_or("client");

            if launch_type != "server" {
                if console.exists() {
                    info!("Injecting console unlock.");
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    inject(console, pid);
                } else {
                    warn!("No console library found. You will have to inject this manually.");
                }
            } else {
                if server.exists() {
                    info!("Injecting headed server library.");
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    inject(server, pid);
                } else {
                    warn!("No game server library found. You will have to inject this manually.");
                }
            }
        }
    }

    if let Err(e) = fortnite.wait() {
        error!("Error waiting for Fortnite process to exit: {}", e);
    }
    
    process::kill_all();
    info!("Game closed, Instigator cleaning up.");
}
