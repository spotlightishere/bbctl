use rusb::{DeviceList, Result};

pub fn enumerate() {
    scan_devices().expect("failed to scan devices")
}

fn scan_devices() -> Result<()> {
    for device in DeviceList::new()?.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        println!(
            "Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id()
        );
    }

    Ok(())
}
