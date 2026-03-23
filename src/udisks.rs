use zbus::{Connection, fdo::ObjectManagerProxy};

// Struct to hold our new data
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

    let mut devices = Vec::new();
    let objects = manager.get_managed_objects().await?;

    for (path, interfaces) in objects {
        if path.as_str().contains("zram") {
            continue;
        }
        if let Some(block_props) = interfaces.get("org.freedesktop.UDisks2.Block") {

            // Extract the size property (it returns a zvariant::Value which we convert to u64)
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
