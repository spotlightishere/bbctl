mod device_error;
mod usb_transport;

pub fn enumerate() {
    usb_transport::open_device().expect("failed to open device")
}
