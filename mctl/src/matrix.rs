use led_matrix_serial_api::{LedMatrix, Error};
use std::{thread, time::Duration};

pub fn open(brightness: u8) -> Result<LedMatrix, Error> {
    let mut matrix = LedMatrix::open_all()?;
    matrix.set_brightness(brightness)?;
    Ok(matrix)
}

pub fn wait_and_reset(matrix: &mut LedMatrix, seconds: u64) -> Result<(), Error> {
    thread::sleep(Duration::from_secs(seconds));
    matrix.percent(0)
}
