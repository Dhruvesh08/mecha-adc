use std::io::Result;

mod adc;

fn main() -> Result<()> {
    let mut adc = adc::Adc::new()?;
    let voltage0 = adc.read_voltage0()?;
    let voltage1 = adc.read_voltage1()?;
    println!("Voltage 0: {}", voltage0);
    println!("Voltage 1: {}", voltage1);
    Ok(())
}
