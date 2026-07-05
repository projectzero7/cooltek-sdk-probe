use ::cooltek_sdk_probe::{connect_device_cm_m0_slider, get_slider_value_cm_m0};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let m005s = connect_device_cm_m0_slider(None)?;

    println!("{:?}", get_slider_value_cm_m0(&m005s)?);

    Ok(())
}
