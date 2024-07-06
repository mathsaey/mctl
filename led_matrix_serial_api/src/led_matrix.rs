use core::time::Duration;
use serialport::SerialPort;
use std::fmt::{self, Display, Formatter};
use crate::error::Result;

const BAUD_RATE: u32 = 115_200;
const CMD_PREFIX: [u8; 2] = [0x32, 0xAC];
const RESPONSE_SIZE: usize = 32;
const DEFAULT_TIMEOUT: Duration = Duration::from_millis(20);

#[derive(Debug)]
pub enum LedMatrix {
    Device(Box<dyn SerialPort>),
    Collection(Vec<Box<dyn SerialPort>>),
}

impl Display for LedMatrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Device(serial) => {
                write!(
                    f,
                    "LED matrix {}",
                    serial.name().unwrap_or("unnamed".to_string())
                )
            }
            Self::Collection(vec) => {
                let devices = vec
                    .into_iter()
                    .map(|p| p.name().unwrap_or("unnamed".to_string()))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "LED matrices {}", devices)
            }
        }
    }
}

impl FromIterator<Box<dyn SerialPort>> for LedMatrix {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Box<dyn SerialPort>>,
    {
        let vec: Vec<Box<dyn SerialPort>> = Vec::from_iter(iter);
        Self::Collection(vec)
    }
}

impl LedMatrix {
    pub fn open(device: String) -> Result<Self> {
        let device = serialport::new(device, BAUD_RATE)
            .timeout(DEFAULT_TIMEOUT)
            .open()?;
        Ok(Self::Device(device))
    }

    pub fn open_many(devices: Vec<String>) -> Result<Self> {
        devices
            .into_iter()
            .map(|dev| {
                serialport::new(dev, BAUD_RATE)
                    .timeout(DEFAULT_TIMEOUT)
                    .open()
                    .map_err(From::from)
            })
            .collect()
    }

    pub fn open_all() -> Result<Self> {
        let devices = crate::get_device_names()?;
        Self::open_many(devices)
    }

    pub fn set_read_timeout(&mut self, timeout: Duration) {
        match self {
            Self::Device(port) => port.set_timeout(timeout).unwrap(),
            Self::Collection(vec) => vec
                .into_iter()
                .for_each(|p| p.set_timeout(timeout).unwrap()),
        }
    }

    fn write_buff(&mut self, buff: &[u8]) -> Result<()> {
        match self {
            Self::Device(port) => {
                port.write(&buff)?;
                port.flush().map_err(From::from)
            }
            Self::Collection(vec) => vec
                .into_iter()
                .map(|p| {
                    p.write(&buff)?;
                    p.flush().map_err(From::from)
                })
                .collect(),
        }
    }

    fn cmd(&mut self, cmd: &[u8]) -> Result<()> {
        let mut buff = Vec::with_capacity(2 + cmd.len());
        buff.extend_from_slice(&CMD_PREFIX);
        buff.extend_from_slice(cmd);

        self.write_buff(&buff)
    }

    fn read(&mut self) -> Result<Vec<u8>> {
        let mut buff: [u8; 32] = [0; RESPONSE_SIZE];

        match self {
            Self::Device(port) => {
                port.read_exact(&mut buff)?;
                Ok(vec![buff[0]])
            }
            Self::Collection(vec) => vec
                .into_iter()
                .map(|p| {
                    p.read_exact(&mut buff)?;
                    Ok(buff[0])
                })
                .collect(),
        }
    }

    fn set_cmd(&mut self, cmd: u8, bool: bool) -> Result<()> {
        self.cmd(&[cmd, bool as u8])
    }

    fn read_bool(&mut self) -> Result<Vec<bool>> {
        self.read().map(|v| v.into_iter().map(|i| i != 0).collect())
    }

    pub fn get_brightness(&mut self) -> Result<Vec<u8>> {
        self.cmd(&[0x00])?;
        self.read()
    }

    // Max 255
    pub fn set_brightness(&mut self, pct: u8) -> Result<()> {
        self.cmd(&[0x00, pct])
    }

    pub fn percent(&mut self, pct: u8) -> Result<()> {
        assert!(pct <= 100);
        self.cmd(&[0x01, 0x00, pct])
    }

    pub fn gradient(&mut self) -> Result<()> {
        self.cmd(&[0x01, 0x01])
    }

    pub fn double_gradient(&mut self) -> Result<()> {
        self.cmd(&[0x01, 0x02])
    }

    pub fn lotus_horizontal(&mut self) -> Result<()> {
        self.cmd(&[0x01, 0x03])
    }

    pub fn zigzag(&mut self) -> Result<()> {
        self.cmd(&[0x01, 0x04])
    }

    pub fn full_brightness(&mut self) -> Result<()> {
        self.cmd(&[0x01, 0x05])
    }

    pub fn panic(&mut self) -> Result<()> {
        self.cmd(&[0x01, 0x06])
    }

    pub fn lotus_vertical(&mut self) -> Result<()> {
        self.cmd(&[0x01, 0x07])
    }

    pub fn test(&mut self) -> Result<()> {
        self.cmd(&[0x01, 0x08])
    }

    pub fn get_sleep(&mut self) -> Result<Vec<bool>> {
        self.cmd(&[0x03])?;
        self.read_bool()
    }

    pub fn set_sleep(&mut self, sleep: bool) -> Result<()> {
        self.set_cmd(0x03, sleep)
    }

    pub fn get_animate(&mut self) -> Result<Vec<bool>> {
        self.cmd(&[0x04])?;
        self.read_bool()
    }

    pub fn set_animate(&mut self, animate: bool) -> Result<()> {
        self.set_cmd(0x04, animate)
    }

    pub fn draw_bw_buffer(&mut self, bytes: &[u8; 39]) -> Result<()> {
        let mut buff: [u8; 42] = [0; 42];
        buff[..2].copy_from_slice(&CMD_PREFIX);
        buff[2] = 0x06;
        buff[3..].copy_from_slice(bytes);
        self.write_buff(&buff)
    }

    pub fn stage_col(&mut self, col: u8, bytes: &[u8; 34]) -> Result<()> {
        let mut buff: [u8; 38] = [0; 38];
        buff[..2].copy_from_slice(&CMD_PREFIX);
        buff[2] = 0x07;
        buff[3] = col;
        buff[4..].copy_from_slice(bytes);
        self.write_buff(&buff)
    }

    pub fn flush_cols(&mut self) -> Result<()> {
        self.cmd(&[0x08])
    }

    // max 255
    pub fn draw_cols(&mut self, cols: &[[u8; 34]; 9]) -> Result<()> {
        for (i, col) in cols.iter().enumerate() {
            self.stage_col(i as u8, &col)?;
        }
        self.flush_cols()
    }
}
