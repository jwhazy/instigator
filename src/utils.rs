use crate::config;

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
    });

    if any_updated {
        config::save(clients);
    }
}
