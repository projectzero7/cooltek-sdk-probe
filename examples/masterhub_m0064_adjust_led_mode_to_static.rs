use ::cooltek_sdk_probe::{LightingMode, adjust_led_mode_cm_m0_base, connect_device_cm_m0_base};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let m0064 = connect_device_cm_m0_base(None)?;
    adjust_led_mode_cm_m0_base(&m0064, LightingMode::Static, false)?;

    Ok(())
}
