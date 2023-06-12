use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write, Result};

pub struct Adc {
    voltage0_file: File,
    voltage1_file: File,
    sampling_frequency_file: File,
}

impl Adc {
    pub fn new() -> Result<Self> {
        let voltage0_file = File::open("/sys/bus/iio/devices/iio:device0/in_voltage0_raw")?;
        let voltage1_file = File::open("/sys/bus/iio/devices/iio:device0/in_voltage1_raw")?;
        let sampling_frequency_file = File::open("/sys/bus/iio/devices/iio:device0/in_voltage0_sampling_frequency")?;
        Ok(Self {
            voltage0_file,
            voltage1_file,
            sampling_frequency_file,
        })
    }

    pub fn read_voltage0(&mut self) -> Result<u16> {
        let mut buffer = String::new();
        self.voltage0_file.read_to_string(&mut buffer)?;
        print!("{}", buffer);
        Ok(buffer.trim().parse::<u16>().unwrap())
    }

    pub fn read_voltage1(&mut self) -> Result<u16> {
        let mut buffer = String::new();
        self.voltage1_file.read_to_string(&mut buffer)?;
        Ok(buffer.trim().parse::<u16>().unwrap())
    }

    pub fn read_sampling_frequency(&mut self) -> Result<u32> {
        let mut buffer = String::new();
        self.sampling_frequency_file.seek(SeekFrom::Start(0))?;
        self.sampling_frequency_file.read_to_string(&mut buffer)?;
        Ok(buffer.trim().parse::<u32>().unwrap())
    }

    pub fn set_sampling_frequency(&mut self, frequency: u32) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .open("/sys/bus/iio/devices/iio:device0/in_voltage0_sampling_frequency")?;
        file.write_all(frequency.to_string().as_bytes())?;
        Ok(())
    }
}
