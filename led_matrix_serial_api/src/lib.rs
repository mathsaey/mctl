use serialport::{SerialPortType::UsbPort, UsbPortInfo};

mod error;
mod led_matrix;

pub use error::{Error, Result};
pub use led_matrix::LedMatrix;

const FRAMEWORK_VENDOR_ID: u16 = 12972;
const LED_MATRIX_PRODUCT_ID: u16 = 32;

pub fn get_device_names() -> Result<Vec<String>> {
    let ports = serialport::available_ports()?;

    let ports = ports
        .into_iter()
        .filter_map(|port| match port.port_type {
            UsbPort(UsbPortInfo {
                vid: FRAMEWORK_VENDOR_ID,
                pid: LED_MATRIX_PRODUCT_ID,
                ..
            }) => Some(port.port_name),
            _ => None,
        })
        .collect();

    Ok(ports)
}
