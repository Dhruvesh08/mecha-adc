use crate::adc::{Adc, AdcDevice};

mod adc;
fn main() {
    let mut adc = Adc {
        channel_1_path: "/sys/bus/iio/devices/iio:device0/in_voltage0_raw",
        channel_2_path: "/sys/bus/iio/devices/iio:device0/in_voltage1_raw",
    };
    let channel1 = adc.read_channel1();
    let channel2 = adc.read_channel2();
    println!("Channel 1: {:?}", channel1);
    println!("Channel 2: {:?}", channel2);

    //wait for 5 seconds 
    std::thread::sleep(std::time::Duration::from_secs(5));

    //read again
    let channel1 = adc.read_channel1();
    let channel2 = adc.read_channel2();
    println!("Channel 1: {:?}", channel1);
    println!("Channel 2: {:?}", channel2);
}