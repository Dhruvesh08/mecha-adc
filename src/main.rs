use std::io::Result;

mod adc;

fn main() -> Result<()> {
    let mut adc = adc::Adc::new()?;
    let voltage0 = adc.read_voltage0()?;
    let voltage1 = adc.read_voltage1()?;
    println!("Voltage 0: {}", voltage0);
    println!("Voltage 1: {}", voltage1);
    let sampling_frequency = adc.read_sampling_frequency()?;
    println!("Sampling Frequency: {}", sampling_frequency);
    adc.set_sampling_frequency(adc::SamplingFrequency::F16)?;
    println!("Sampling frequency set to {}",adc.read_sampling_frequency()?);
    println!("updated Sampling Frequency: {}", adc.read_sampling_frequency()?);
    Ok(())
}
