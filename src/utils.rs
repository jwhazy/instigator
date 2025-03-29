use crate::config;
use log::{error, info};

pub fn run_bwc_checks() {
    let mut clients = config::get();

    let mut any_updated = false;
    clients.iter_mut().for_each(|client| {
        if !["server", "client", "headless"].contains(&client.name.as_str())
            && client.launch_type.is_none()
        {
            client.launch_type = Some("client".to_string());
            any_updated = true;
        }

        // Check if client has email if it doesnt set it to localhost
        if client.email.is_none() {
            client.email = Some("localhost".to_string());

            any_updated = true;
        }
    });

    if any_updated {
        config::save(clients);
    }
}

pub fn folder() {
    if let Err(e) = std::process::Command::new("explorer")
        .arg(crate::config::app_directory())
        .spawn() {
            error!("Could not open File Explorer: {}. Please navigate to %appdata%\\instigator manually.", e);
        } else {
            info!("Opened Instigator directory.\n");
        }
}
