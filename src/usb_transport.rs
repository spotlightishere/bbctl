use rusb::{Context, DeviceHandle, UsbContext};

use crate::device_error::DeviceError;

const RIM_VENDOR: u16 = 0x0FCA;
// TODO: There may be further IDs beyond these.
const RIM_PRODUCT_IDS: &'static [u16] = &[0x8004, 0x8007];

// TODO(spotlightishere): Is it safe for us to hardcode these endpoint addresses?
// We may want to probe to determine, instead of hardcoding addresses.
const ENDPOINT_IN: u8 = 0x82;
const ENDPOINT_OUT: u8 = 0x02;

pub fn open_device() -> Result<(), DeviceError> {
    // TODO(spotlightishere): This function only handles a single device.
    let mut context = Context::new()?;

    // Let's first open the device, and ensure it has .
    let mut handle = find_usb_device(&mut context)?;
    let interface_num = ensure_interface(&mut handle)?;
    println!("Obtained endpoints! Interface number: {}", interface_num);

    Ok(())
}

/// Finds a single device with the proper vendor and a potential product ID.
fn find_usb_device<T: UsbContext>(context: &mut T) -> Result<DeviceHandle<T>, DeviceError> {
    // TODO(spotlightishere): This only handles a single device.
    let devices = context.devices()?;
    let possible_device = devices.iter().find(|potential_device| {
        let Ok(device_desc) = potential_device.device_descriptor() else {
            return false;
        };

        let device_vendor = device_desc.vendor_id();
        let device_product = device_desc.product_id();
        return device_vendor == RIM_VENDOR && RIM_PRODUCT_IDS.contains(&device_product);
    });

    // If possible, get our device.
    let Some(device) = possible_device else {
        return Err(DeviceError::NoDeviceFound);
    };

    // Attempt to open it.
    let handle = device.open()?;
    return Ok(handle);
}

/// Ensures the BlackBerry control interface is available on this device,
/// and has the necessary endpoints for operation.
fn ensure_interface<T: UsbContext>(handle: &mut DeviceHandle<T>) -> Result<u8, DeviceError> {
    // An interface with a class code and protocol of 0xFF must be available.
    // (For reference, its iInterface is "BlackBerry".)
    let device_descriptor = handle.device().active_config_descriptor()?;
    let blackberry_interface_desc = device_descriptor.interfaces().find_map(|interface| {
        // Get the interface's descriptors and check.
        for descriptor in interface.descriptors() {
            if descriptor.class_code() == 0xFF && descriptor.protocol_code() == 0xFF {
                return Some(descriptor);
            }
        }

        return None;
    });
    // TODO(spotlightishere): Is there ever a situation where a BlackBerry device
    // would be available, but intentionally have its control interface disabled?
    // We'll return a protocol error for now.
    let Some(control_interface) = blackberry_interface_desc else {
        return Err(DeviceError::ProtocolError);
    };

    // Lastly, ensure we have valid endpoints.
    let mut control_descriptors = control_interface.endpoint_descriptors();
    control_descriptors
        .find(|d| d.address() == ENDPOINT_IN)
        .ok_or(DeviceError::ProtocolError)?;
    control_descriptors
        .find(|d| d.address() == ENDPOINT_OUT)
        .ok_or(DeviceError::ProtocolError)?;

    Ok(control_interface.interface_number())
}
