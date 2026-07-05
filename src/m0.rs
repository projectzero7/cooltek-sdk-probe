use std::ffi::CStr;

use ::hidapi::{HidApi, HidDevice};

use crate::{CoolTekSdkError, CoolTekSdkResult, DeviceHandle, LightingMode, RcvError};

pub fn adjust_led_brightness_cm_m0(
    device: &HidDevice,
    module: u8,
    index: u8,
    brightness: u8,
) -> CoolTekSdkResult<()> {
    if module == 0 && index != 0 {
        return Err(CoolTekSdkError::InvalidParameter);
    }

    if module == 1 && (index > 4 && index != 0xff) {
        return Err(CoolTekSdkError::InvalidParameter);
    }

    if module == 2 && index > 2 && index != 0xff {
        return Err(CoolTekSdkError::InvalidParameter);
    }

    if module > 2 {
        return Err(CoolTekSdkError::InvalidParameter);
    }

    // NOTE: the source doesn't seem to error if brightness > 0x64; this is probably a bug.

    let mut raw = [0u8; 0x40];
    raw[0] = 0x02;
    raw[1] = 0x2e;
    raw[2] = module;
    raw[3] = index;
    raw[4] = brightness;

    let _ = device
        .write(&mut raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    Ok(())
}

pub fn adjust_led_color_cm_m0(
    device: &HidDevice,
    module: u8,
    index: u8,
    r: u8,
    g: u8,
    b: u8,
    save_fw: bool,
) -> CoolTekSdkResult<()> {
    if module == 0 && index != 0 {
        return Err(CoolTekSdkError::InvalidParameter);
    }

    if module == 1 && (index > 4 && index != 0xff) {
        return Err(CoolTekSdkError::InvalidParameter);
    }

    if module == 2 && index > 2 && index != 0xff {
        return Err(CoolTekSdkError::InvalidParameter);
    }

    if module > 2 {
        return Err(CoolTekSdkError::InvalidParameter);
    }

    let mut raw = [0u8; 0x40];
    raw[0] = 0x02;
    raw[1] = 0x2d;
    raw[2] = module;
    raw[3] = index;
    raw[4] = r;
    raw[5] = g;
    raw[6] = b;
    raw[7] = save_fw as u8;

    let _ = device
        .write(&mut raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    Ok(())
}

pub fn adjust_led_mode_cm_m0(
    device: &HidDevice,
    module: u8,
    index: u8,
    mode: LightingMode,
    save_fw: bool,
) -> CoolTekSdkResult<()> {
    if module == 0 && index != 0 {
        return Err(CoolTekSdkError::InvalidParameter);
    }

    if module == 1 && (index > 4 && index != 0xff) {
        return Err(CoolTekSdkError::InvalidParameter);
    }

    if module == 2 && index > 2 && index != 0xff {
        return Err(CoolTekSdkError::InvalidParameter);
    }

    if module > 2 {
        return Err(CoolTekSdkError::InvalidParameter);
    }

    let mut raw = [0u8; 0x40];
    raw[0] = 0x02;
    raw[1] = 0x2c;
    raw[2] = module;
    raw[3] = index;
    raw[4] = mode as u8;
    raw[5] = save_fw as u8;

    let _ = device
        .write(&mut raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    Ok(())
}

pub fn adjust_led_speed_cm_m0(device: &HidDevice, speed: u8) -> CoolTekSdkResult<()> {
    if speed > 0x64 {
        return Err(CoolTekSdkError::InvalidParameter);
    }

    let mut raw = [0u8; 0x40];
    raw[0] = 0x02;
    raw[1] = 0x30;
    raw[2] = speed;

    let _ = device
        .write(&mut raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    Ok(())
}

pub fn calibrate_direction_cm_m0(device: &HidDevice) -> CoolTekSdkResult<()> {
    let mut raw = [0u8; 0x40];
    raw[0] = 0x02;
    raw[1] = 0x2f;

    let _ = device
        .write(&mut raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    Ok(())
}

pub fn check_rcv_data_cm_m0(
    handle: &mut DeviceHandle,
    timeout_ms: u32,
    channel: i32,
) -> Result<Vec<u8>, RcvError> {
    fun_1800042b0(handle)?;

    if handle.ring_buffer.occupied_count[channel as usize] != 0 {
        return drain_channel_slot(handle, channel);
    }

    let bytes_read = {
        let DeviceHandle {
            device,
            read_buffer,
            ..
        } = &mut *handle;

        device
            .as_mut()
            .expect("device is Some: guaranteed by fun_1800042b0")
            .read_timeout(read_buffer, timeout_ms as i32)
            .map_err(|_| RcvError::HidReadError)?
    };

    if bytes_read <= 0 {
        return Err(RcvError::Timeout);
    }

    if handle.read_buffer[0] != 0x01 {
        return Err(RcvError::BadReportId);
    }

    if !check_channel_tag(&handle.read_buffer, channel) {
        return Err(route_to_ring_slot(handle));
    }

    if channel == 3 {
        let packet_size = handle.packet_size;

        Ok(handle.read_buffer[..packet_size].to_vec())
    } else {
        if channel == 4 && handle.ring_buffer.occupied_count[3] != 0 {
            return Err(route_to_ring_slot(handle));
        }

        let payload_len =
            u16::from_le_bytes([handle.read_buffer[2], handle.read_buffer[3]]) as usize;

        Ok(handle.read_buffer[4..4 + payload_len].to_vec())
    }
}

pub fn connect_device_by_path_cm_m0(path: &CStr) -> CoolTekSdkResult<HidDevice> {
    let context = HidApi::new().map_err(|_| CoolTekSdkError::FailedToAllocateMemory)?;

    Ok(context
        .open_path(path)
        .map_err(|_| CoolTekSdkError::DeviceFailedToOpen)?)
}

pub fn connect_device_cm_m0(
    vendor_id: u16,
    product_id: u16,
    serial_number: Option<&str>,
) -> CoolTekSdkResult<HidDevice> {
    let context = HidApi::new().map_err(|_| CoolTekSdkError::FailedToAllocateMemory)?;

    Ok(match serial_number {
        Some(serial_number) => context.open_serial(vendor_id, product_id, serial_number),
        None => context.open(vendor_id, product_id),
    }
    .map_err(|_| CoolTekSdkError::DeviceFailedToOpen)?)
}

pub fn disconnect_device_cm_m0(device: HidDevice) {
    // move the device handle and Drop should handle the rest
}

pub fn get_config_data_cm_m0(
    device: &HidDevice,
    param_2: char,
    param_3: char,
) -> CoolTekSdkResult<()> {
    if param_2 != '\0' && param_3 != '\0' {
        return Err(CoolTekSdkError::InvalidParameter);
    }

    let mut raw = [0u8; 0x40];
    raw[0] = 0x02;
    raw[1] = match param_2 {
        '\0' => 0x33,
        _ => 0x32,
    };

    let _ = device
        .write(&mut raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    Ok(())
}

pub fn get_config_info_cm_m0(device: &HidDevice) -> CoolTekSdkResult<Vec<u8>> {
    let mut raw = [0u8; 0x21];
    raw[0] = 0x36;

    let _ = device
        .get_feature_report(&mut raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    let len = raw[1] as usize;

    Ok(raw[2..2 + len].to_vec())
}

pub fn get_device_direction_cm_m0(device: &HidDevice) -> CoolTekSdkResult<u8> {
    let mut raw = [0u8; 0x21];
    raw[0] = 0x35;

    let _ = device
        .get_feature_report(&mut raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    Ok(raw[2])
}

pub fn get_device_infos_cm_m0(device: &HidDevice) -> CoolTekSdkResult<char> {
    let mut raw = [0u8; 0x21];
    raw[0] = b'1';

    let _ = device
        .get_feature_report(&mut raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    if raw[0] != b'1' {
        return Err(CoolTekSdkError::InvalidResponse);
    }

    Ok(raw[2] as char)
}

pub fn get_fw_infos_by_type_cm_m0(
    device: &HidDevice,
    component: u8,
) -> CoolTekSdkResult<(u32, String)> {
    if component > 0x02 {
        return Err(CoolTekSdkError::InvalidParameter);
    }

    let mut raw = [0u8; 0x21];
    raw[0] = match component {
        0x00 => 0x2d,
        0x01 => 0x2e,
        0x02 => 0x2f,
        _ => unreachable!(),
    };

    let _ = device
        .get_feature_report(&mut raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    let raw_version = u32::from_le_bytes(raw[2..(2 + 0x04)].try_into().unwrap());
    let display_version = String::from_utf8_lossy(&raw[6..(6 + 0x0b)])
        .trim_matches('\0')
        .to_string();

    Ok((raw_version, display_version))
}

pub fn get_fw_infos_cm_m0(device: &HidDevice) -> CoolTekSdkResult<(u32, String)> {
    get_fw_infos_by_type_cm_m0(device, 0x01)
}

pub fn get_led_brightness_cm_m0(device: &HidDevice) -> CoolTekSdkResult<[u8; 0x05]> {
    let mut raw = [0u8; 0x21];
    raw[0] = 0x32;

    let _ = device
        .get_feature_report(&mut raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    let len = raw[1] as usize;
    let mut output = [0u8; 0x05];
    output[0..len].copy_from_slice(&raw[2..2 + len]);

    Ok(output)
}

pub fn get_led_mode_cm_m0(device: &HidDevice) -> CoolTekSdkResult<[u8; 0x05]> {
    let mut raw = [0u8; 0x21];
    raw[0] = 0x34;

    let _ = device
        .get_feature_report(&mut raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    let len = raw[1] as usize;
    let mut output = [0u8; 0x05];
    output[0..len].copy_from_slice(&raw[2..2 + len]);

    Ok(output)
}

pub fn get_serial_number_cm_m0(device: &HidDevice) -> CoolTekSdkResult<Option<String>> {
    let mut raw = [0u8; 0x21];
    raw[0] = 0x30;

    let _ = device
        .get_feature_report(&mut raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    Ok(match raw[1] as usize {
        0 => None,
        n => {
            let slice = &raw[2..2 + n];
            let raw_serial_number = String::from_utf8_lossy(slice);
            let trimmed_serial_number = raw_serial_number.trim_matches('\0');

            match trimmed_serial_number.len() {
                0 => None,
                _ => Some(trimmed_serial_number.to_string()),
            }
        }
    })
}

pub fn get_slider_value_cm_m0(device: &HidDevice) -> CoolTekSdkResult<[u8; 0x05]> {
    let mut raw = [0u8; 0x21];
    raw[0] = 0x33;

    let _ = device
        .get_feature_report(&mut raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    let mut output = [0u8; 0x05];
    output[0..5].copy_from_slice(&raw[2..7]);

    Ok(output)
}

pub fn save_factory_data_cm_m0(device: &HidDevice) -> CoolTekSdkResult<()> {
    let mut raw = [0u8; 0x40];
    raw[0] = 0x02;
    raw[1] = 0x31;

    let _ = device
        .write(&mut raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    Ok(())
}

pub fn set_serial_number_cm_m0(device: &HidDevice, serial_number: &str) -> CoolTekSdkResult<()> {
    if serial_number.len() > 0x1d {
        return Err(CoolTekSdkError::InvalidParameter);
    }

    let mut raw = [0u8; 0x40];
    raw[0] = 0x02;
    raw[1] = 0x2b;
    raw[2] = serial_number.len() as u8;
    raw[3..serial_number.len()].copy_from_slice(serial_number.as_bytes());

    let _ = device
        .write(&mut raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    Ok(())
}

pub fn test_factory_reset_process_cm_m0(device: &HidDevice) -> CoolTekSdkResult<()> {
    let mut raw = [0u8; 0x40];
    raw[0] = 0x02;
    raw[1] = 0x4a;

    let _ = device
        .write(&mut raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    Ok(())
}

pub fn test_fw_recovery_process_cm_m0(device: &HidDevice) -> CoolTekSdkResult<()> {
    let mut raw = [0u8; 0x40];
    raw[0] = 0x02;
    raw[1] = 0x49;

    let _ = device
        .write(&mut raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    Ok(())
}

pub fn update_fw_by_type_cm_m0(
    device: &HidDevice,
    firmware: &[u8],
    fw_type: u32,
) -> CoolTekSdkResult<()> {
    const REPORT_LEN: usize = 64;
    const CHUNK_MAX: usize = REPORT_LEN - 0x10;
    const PAGE_SIZE: usize = 0x1000;

    let fw_len = firmware.len() as u32;

    if firmware.is_empty() {
        return Err(CoolTekSdkError::InvalidParameter);
    }
    if fw_type == 0 && fw_len > 0x0fff {
        return Err(CoolTekSdkError::InvalidParameter);
    }
    if fw_type >= 1 && fw_len >= 0xa000 {
        return Err(CoolTekSdkError::InvalidParameter);
    }

    let mut raw = [0u8; REPORT_LEN];
    raw[0] = 0x02;
    raw[1] = 0x2a;

    let num_full_pages = fw_len as usize / PAGE_SIZE;
    let last_page_size = fw_len as usize % PAGE_SIZE;
    let num_pages = num_full_pages + if last_page_size > 0 { 1 } else { 0 };

    for page in 0..num_pages {
        let is_last_page = page == num_pages - 1;
        let page_data_size = if is_last_page && last_page_size > 0 {
            last_page_size
        } else {
            PAGE_SIZE
        };

        let mut remaining = page_data_size;
        let mut chunk_index: u16 = 0;

        // Reset per-page flags
        raw[2] = page as u8;
        raw[3] = 0; // last chunk in page
        raw[8] = 0; // last page
        raw[9] = fw_type as u8;

        while remaining > 0 {
            let chunk_len = remaining.min(CHUNK_MAX);
            let is_last_chunk = remaining <= CHUNK_MAX;

            if is_last_chunk {
                raw[3] = 1; // last chunk in page

                if is_last_page {
                    raw[8] = 1; // last page
                }
            }

            raw[4..6].copy_from_slice(&chunk_index.to_le_bytes());
            raw[6..8].copy_from_slice(&(chunk_len as u16).to_le_bytes());
            raw[10..16].fill(0);
            raw[16..].fill(0);

            let src_offset = page * PAGE_SIZE + (page_data_size - remaining);
            raw[16..16 + chunk_len].copy_from_slice(&firmware[src_offset..src_offset + chunk_len]);

            let written = device
                .write(&raw)
                .map_err(|_| CoolTekSdkError::TransferFailed)?;
            if written != REPORT_LEN {
                return Err(CoolTekSdkError::TransferFailed);
            }

            chunk_index += 1;
            remaining -= chunk_len;
        }
    }

    Ok(())
}

pub fn update_fw_cm_m0(device: &HidDevice, firmware: &[u8], fw_type: u8) -> CoolTekSdkResult<()> {
    update_fw_by_type_cm_m0(device, firmware, fw_type as u32)
}

// -------------------------------

/// Adjusts the brightness of the LEDs on a MasterHUB Base module.
///
/// # Parameters:
///
/// * `device` - The [`HidDevice`] handle to communicate with the physical device.
///
/// * `brightness` - The brightness to set the LEDs to. Should be in the range `[0x00, 0x64]` (`[0, 100]`).
pub fn adjust_led_brightness_cm_m0_base(
    device: &HidDevice,
    brightness: u8,
) -> CoolTekSdkResult<()> {
    adjust_led_brightness_cm_m0(device, 0x00, 0x00, brightness)
}

pub fn adjust_led_brightness_cm_m0_knob(
    device: &HidDevice,
    index: u8,
    brightness: u8,
) -> CoolTekSdkResult<()> {
    adjust_led_brightness_cm_m0(device, 0x02, index, brightness)
}

pub fn adjust_led_brightness_cm_m0_slider(
    device: &HidDevice,
    index: u8,
    brightness: u8,
) -> CoolTekSdkResult<()> {
    adjust_led_brightness_cm_m0(device, 0x01, index, brightness)
}

pub fn adjust_led_color_cm_m0_base(
    device: &HidDevice,
    r: u8,
    g: u8,
    b: u8,
    save_fw: bool,
) -> CoolTekSdkResult<()> {
    adjust_led_color_cm_m0(device, 0x00, 0x00, r, g, b, save_fw)
}

pub fn adjust_led_color_cm_m0_knob(
    device: &HidDevice,
    index: u8,
    r: u8,
    g: u8,
    b: u8,
    save_fw: bool,
) -> CoolTekSdkResult<()> {
    adjust_led_color_cm_m0(device, 0x02, index, r, g, b, save_fw)
}

pub fn adjust_led_color_cm_m0_slider(
    device: &HidDevice,
    index: u8,
    r: u8,
    g: u8,
    b: u8,
    save_fw: bool,
) -> CoolTekSdkResult<()> {
    adjust_led_color_cm_m0(device, 0x01, index, r, g, b, save_fw)
}

pub fn adjust_led_mode_cm_m0_base(
    device: &HidDevice,
    mode: LightingMode,
    save_fw: bool,
) -> CoolTekSdkResult<()> {
    adjust_led_mode_cm_m0(device, 0x00, 0x00, mode, save_fw)
}

pub fn adjust_led_mode_cm_m0_knob(
    device: &HidDevice,
    index: u8,
    mode: LightingMode,
    save_fw: bool,
) -> CoolTekSdkResult<()> {
    adjust_led_mode_cm_m0(device, 0x02, index, mode, save_fw)
}

pub fn adjust_led_mode_cm_m0_slider(
    device: &HidDevice,
    index: u8,
    mode: LightingMode,
    save_fw: bool,
) -> CoolTekSdkResult<()> {
    adjust_led_mode_cm_m0(device, 0x01, index, mode, save_fw)
}

pub fn connect_device_cm_m0_base(serial_number: Option<&str>) -> CoolTekSdkResult<HidDevice> {
    connect_device_cm_m0(0x2516, 0x01f6, serial_number)
}

pub fn connect_device_cm_m0_knob(serial_number: Option<&str>) -> CoolTekSdkResult<HidDevice> {
    connect_device_cm_m0(0x2516, 0x0202, serial_number)
}

pub fn connect_device_cm_m0_slider(serial_number: Option<&str>) -> CoolTekSdkResult<HidDevice> {
    connect_device_cm_m0(0x2516, 0x0200, serial_number)
}

pub fn connect_device_cm_m0_wheel(serial_number: Option<&str>) -> CoolTekSdkResult<HidDevice> {
    connect_device_cm_m0(0x2516, 0x0204, serial_number)
}

pub fn get_fw_infos_cm_m0_base(device: &HidDevice) -> CoolTekSdkResult<String> {
    let (_, display_version) = get_fw_infos_cm_m0(device)?;

    Ok(display_version)
}

pub fn get_fw_infos_cm_m0_knob(device: &HidDevice) -> CoolTekSdkResult<String> {
    let (_, display_version) = get_fw_infos_cm_m0(device)?;

    Ok(display_version)
}

pub fn get_fw_infos_cm_m0_slider(device: &HidDevice) -> CoolTekSdkResult<String> {
    let (_, display_version) = get_fw_infos_cm_m0(device)?;

    Ok(display_version)
}

pub fn get_fw_infos_cm_m0_wheel(device: &HidDevice) -> CoolTekSdkResult<String> {
    let (_, display_version) = get_fw_infos_cm_m0(device)?;

    Ok(display_version)
}

pub fn update_fw_cm_m0_base(device: &HidDevice, firmware: &[u8]) -> CoolTekSdkResult<()> {
    update_fw_cm_m0(device, firmware, 0x01)
}

pub fn update_fw_cm_m0_knob(device: &HidDevice, firmware: &[u8]) -> CoolTekSdkResult<()> {
    update_fw_cm_m0(device, firmware, 0x01)
}

pub fn update_fw_cm_m0_slider(device: &HidDevice, firmware: &[u8]) -> CoolTekSdkResult<()> {
    update_fw_cm_m0(device, firmware, 0x01)
}

pub fn update_fw_cm_m0_wheel(device: &HidDevice, firmware: &[u8]) -> CoolTekSdkResult<()> {
    update_fw_cm_m0(device, firmware, 0x01)
}

// -------------------------------

const CHANNEL_TAGS: [u8; crate::CHANNEL_COUNT] = [b'*', b'+', b',', b'-', b'.'];

fn fun_1800042b0(handle: &DeviceHandle) -> Result<(), RcvError> {
    if handle.device.is_none() {
        Err(RcvError::DeviceNotConnected)
    } else {
        Ok(())
    }
}

fn check_channel_tag(buffer: &[u8], channel: i32) -> bool {
    match channel {
        0..=4 => buffer[1] == CHANNEL_TAGS[channel as usize],
        _ => false,
    }
}

#[inline]
fn increment_slot_index(index: u8) -> u8 {
    index.wrapping_add(1) & 0x3F
}

#[inline]
fn decrement_occupied_count(count: u8) -> u8 {
    count.saturating_sub(1)
}

fn route_to_ring_slot(handle: &mut DeviceHandle) -> RcvError {
    let channel: usize = match handle.read_buffer[1] {
        b'*' => 0,
        b'+' => 1,
        b',' => 2,
        b'-' => 3,
        b'.' => 4,
        _ => return RcvError::UnknownTag,
    };

    let packet_size = handle.packet_size;
    let write_idx = handle.ring_buffer.write_head[channel] as usize;

    let DeviceHandle {
        ring_buffer,
        read_buffer,
        ..
    } = handle;

    ring_buffer.slots[write_idx][channel][..packet_size]
        .copy_from_slice(&read_buffer[..packet_size]);

    ring_buffer.occupied_count[channel] = increment_slot_index(ring_buffer.occupied_count[channel]);
    ring_buffer.write_head[channel] = increment_slot_index(ring_buffer.write_head[channel]);

    RcvError::Routed
}

fn drain_channel_slot(handle: &mut DeviceHandle, channel: i32) -> Result<Vec<u8>, RcvError> {
    let channel = channel as usize;
    let packet_size = handle.packet_size;

    let read_idx = handle.ring_buffer.read_tail[channel] as usize;

    let output = if channel == 3 {
        handle.ring_buffer.slots[read_idx][channel][..packet_size].to_vec()
    } else {
        if channel == 4 && handle.ring_buffer.occupied_count[3] != 0 {
            return Err(RcvError::DrainError);
        }

        let payload_len = u16::from_le_bytes([
            handle.ring_buffer.slots[read_idx][channel][2],
            handle.ring_buffer.slots[read_idx][channel][3],
        ]) as usize;
        handle.ring_buffer.slots[read_idx][channel][4..4 + payload_len].to_vec()
    };

    handle.ring_buffer.slots[read_idx][channel][..packet_size].fill(0);

    handle.ring_buffer.occupied_count[channel] =
        decrement_occupied_count(handle.ring_buffer.occupied_count[channel]);
    handle.ring_buffer.read_tail[channel] =
        increment_slot_index(handle.ring_buffer.read_tail[channel]);

    Ok(output)
}
