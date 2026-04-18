use zbus::{Connection, fdo::ObjectManagerProxy};
use adw::gio::Settings;
use adw::glib;
use adw::prelude::SettingsExtManual;

pub struct DriveInfo {
    pub path: String,
    pub size_bytes: u64,
}

pub async fn list_block_devices() -> zbus::Result<Vec<DriveInfo>> {
    let connection = Connection::system().await?;
    let manager = ObjectManagerProxy::builder(&connection)
        .destination("org.freedesktop.UDisks2")?
        .path("/org/freedesktop/UDisks2")?
        .build()
        .await?;

    //Load settings and fetch ignored devices here
    let settings = Settings::new("io.github.vani_tty1.sector");
    let ignored_devices: Vec<String> = settings
        .strv("ignored-devices")
        .into_iter()
        .map(|s: glib::GString| s.to_string())
        .collect();

    let mut devices = Vec::new();
    let objects = manager.get_managed_objects().await?;

    for (path, interfaces) in objects {
        //Check the path against the dynamic list instead of just zram
        if ignored_devices.iter().any(|ignored| path.as_str().contains(ignored)) {
            continue;
        }

        if let Some(block_props) = interfaces.get("org.freedesktop.UDisks2.Block") {
            let size_bytes: u64 = if let Some(size_val) = block_props.get("Size") {
                size_val.try_into().unwrap_or(0)
            } else {
                0
            };

            devices.push(DriveInfo {
                path: path.to_string(),
                size_bytes,
            });
        }
    }

    Ok(devices)
}
