use led_matrix_serial_api::{Error, LedMatrix};
use std::{thread, time::Duration};

mod patterns;
pub use patterns::*;

pub fn open(brightness: u8) -> Result<LedMatrix, Error> {
    let mut matrix = LedMatrix::open_all()?;
    matrix.set_brightness(brightness)?;
    Ok(matrix)
}

pub fn wait_and_reset(matrix: &mut LedMatrix, seconds: u64) -> Result<(), Error> {
    thread::sleep(Duration::from_secs(seconds));
    matrix.percent(0)
}
