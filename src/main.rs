use crate::adc::{Adc, AdcDevice};

mod adc;

fn main() {
    let mut adc = Adc {
        channel_1_path: "/sys/bus/iio/devices/iio:device0/in_voltage0_raw",
        channel_2_path: "/sys/bus/iio/devices/iio:device0/in_voltage1_raw",
        sampling_frequency: "/sys/bus/iio/devices/iio:device0/in_voltage0_sampling_frequency",
    };
    
    let channel1 = adc.read_channel_1();
    let channel2 = adc.read_channel_2();
    let sampling_frequency = adc.get_sampling_frequency();
    
    println!("Channel 1: {:?}", channel1);
    println!("Channel 2: {:?}", channel2);
    println!("Sampling Frequency: {:?}", sampling_frequency);
    //wait for 5 seconds 
    std::thread::sleep(std::time::Duration::from_secs(5));
    //set sampling frequency
    let _set_sampling_frequency = adc.set_sampling_frequency("32");
    //read again
    let channel1 = adc.read_channel_1();
    let channel2 = adc.read_channel_2();
    let sampling_frequency = adc.get_sampling_frequency();

    
    println!("Channel 1: {:?}", channel1);
    println!("Channel 2: {:?}", channel2);
    println!("Sampling Frequency: {:?}", sampling_frequency)
}