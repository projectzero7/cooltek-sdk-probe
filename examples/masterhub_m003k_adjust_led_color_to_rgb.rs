use ::cooltek_sdk_probe::{adjust_led_color_cm_m0_knob, connect_device_cm_m0_knob};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let m003k = connect_device_cm_m0_knob(None)?;

    adjust_led_color_cm_m0_knob(&m003k, 0x00, 0xff, 0x00, 0x00, false)?; // Knob 1 to red
    adjust_led_color_cm_m0_knob(&m003k, 0x01, 0x00, 0xff, 0x00, false)?; // Knob 2 to green
    adjust_led_color_cm_m0_knob(&m003k, 0x02, 0x00, 0x00, 0xff, false)?; // Knob 3 to blue

    Ok(())
}
