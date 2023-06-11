use std::fs::File;
use std::io::{Read, Result};

pub struct Adc {
    voltage0_file: File,
    voltage1_file: File,
}

impl Adc {
    pub fn new() -> Result<Self> {
        let voltage0_file = File::open("/sys/bus/iio/devices/iio:device0/in_voltage0_raw")?;
        let voltage1_file = File::open("/sys/bus/iio/devices/iio:device0/in_voltage1_raw")?;
        Ok(Self {
            voltage0_file,
            voltage1_file,
        })
    }

    pub fn read_voltage0(&mut self) -> Result<u16> {
        let mut buffer = String::new();
        self.voltage0_file.read_to_string(&mut buffer)?;
        Ok(buffer.trim().parse::<u16>().unwrap())
    }

    pub fn read_voltage1(&mut self) -> Result<u16> {
        let mut buffer = String::new();
        self.voltage1_file.read_to_string(&mut buffer)?;
        Ok(buffer.trim().parse::<u16>().unwrap())
    }
}
