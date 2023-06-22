use std::fs::File;
use std::io::{Read, Result};

pub trait AdcDevice<'a> {
    fn set_device(&mut self, channel_1_path: &'a str, channel_2_path: &'a str) -> Result<()>;
    fn get_device(&self) -> Option<(&str, &str)>;
    fn read_channel_1(&mut self) -> Result<i16>;
    fn read_channel_2(&mut self) -> Result<i16>;
}

pub struct Adc<'a> {
    pub channel_1_path: &'a str,
    pub channel_2_path: &'a str,
}

impl<'a> AdcDevice<'a> for Adc<'a> {
    fn set_device(&mut self, channel_1_path: &'a str, channel_2_path: &'a str) -> Result<()> {
        self.channel_1_path = channel_1_path;
        self.channel_2_path = channel_2_path;
        Ok(())
    }

    fn get_device(&self) -> Option<(&str, &str)> {
        Some((self.channel_1_path, self.channel_2_path))
    }

    fn read_channel_1(&mut self) -> Result<i16> {
        let mut buffer = String::new();
        File::open(self.channel_1_path)?.read_to_string(&mut buffer)?;
        Ok(buffer.trim().parse::<i16>().unwrap())
    }

    fn read_channel_2(&mut self) -> Result<i16> {
        let mut buffer = String::new();
        File::open(self.channel_2_path)?.read_to_string(&mut buffer)?;
        Ok(buffer.trim().parse::<i16>().unwrap())
    }
}


