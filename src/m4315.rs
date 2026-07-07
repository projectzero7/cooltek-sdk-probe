use std::ffi::CStr;

use ::hidapi::{HidApi, HidDevice};

use crate::{CoolTekSdkError, CoolTekSdkResult, DeviceHandle, RcvError};

pub fn adjust_backlight_cm_m4315(device: &HidDevice, brightness: u8) -> CoolTekSdkResult<()> {
    if brightness > 0x64 {
        return Err(CoolTekSdkError::InvalidParameter);
    }

    let mut raw = [0u8; 0x20];
    raw[0] = 0x03;
    raw[1] = 0x34;
    raw[2] = brightness;

    let _ = device
        .send_feature_report(&raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    Ok(())
}

pub fn check_rcv_data_cm_m4315(
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

pub fn connect_device_by_path_cm_m4315(path: &CStr) -> CoolTekSdkResult<HidDevice> {
    let context = HidApi::new().map_err(|_| CoolTekSdkError::FailedToAllocateMemory)?;

    Ok(context
        .open_path(path)
        .map_err(|_| CoolTekSdkError::DeviceFailedToOpen)?)
}

pub fn connect_device_cm_m4315(serial_number: Option<&str>) -> CoolTekSdkResult<HidDevice> {
    let context = HidApi::new().map_err(|_| CoolTekSdkError::FailedToAllocateMemory)?;

    Ok(match serial_number {
        Some(serial_number) => context.open_serial(0x2516, 0x01fe, serial_number),
        None => context.open(0x2516, 0x01fe),
    }
    .map_err(|_| CoolTekSdkError::DeviceFailedToOpen)?)
}

pub fn disconnect_device_cm_m4315(device: HidDevice) {
    // move the device handle and Drop should handle the rest
}

pub fn display_color_on_button_cm_m4315(
    device: &HidDevice,
    index: u8,
    r: u8,
    g: u8,
    b: u8,
) -> CoolTekSdkResult<()> {
    // NOTE: the source doesn't seem to error if index > 0x0e; this is probably a bug.

    let mut raw = [0u8; 0x20];
    raw[0] = 0x03;
    raw[1] = 0x37;
    raw[2] = index;
    raw[3] = r;
    raw[4] = g;
    raw[5] = b;

    let _ = device
        .send_feature_report(&mut raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    Ok(())
}

pub fn display_color_on_screen_cm_m4315(
    device: &HidDevice,
    r: u8,
    g: u8,
    b: u8,
) -> CoolTekSdkResult<()> {
    let mut raw = [0u8; 0x20];
    raw[0] = 0x03;
    raw[1] = 0x36;
    raw[2] = r;
    raw[3] = g;
    raw[4] = b;

    let _ = device
        .send_feature_report(&mut raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    Ok(())
}

pub fn display_picture_on_button_cm_m4315(
    device: &HidDevice,
    index: u8,
    data: &[u8],
) -> CoolTekSdkResult<()> {
    // NOTE: the source doesn't seem to error if index > 0x0e; this is probably a bug.

    const REPORT_LEN: usize = 320; // 64
    const CHUNK_MAX: usize = REPORT_LEN - 8; // 56

    let total_len = data.len();
    let mut remaining = total_len;
    let mut chunk_index: u16 = 0;

    while remaining > 0 {
        let chunk_len = remaining.min(CHUNK_MAX);
        let is_last = if remaining <= CHUNK_MAX { 1u8 } else { 0u8 };

        let mut report = [0u8; REPORT_LEN];
        report[0] = 0x02;
        report[1] = 0x36;
        report[2..4].copy_from_slice(&chunk_index.to_le_bytes());
        report[4..6].copy_from_slice(&(chunk_len as u16).to_le_bytes());
        report[6] = index;
        report[7] = is_last;

        let src_offset = total_len - remaining;
        report[8..8 + chunk_len].copy_from_slice(&data[src_offset..src_offset + chunk_len]);

        let written = device
            .write(&report)
            .map_err(|_| CoolTekSdkError::TransferFailed)?;
        if written != REPORT_LEN {
            return Err(CoolTekSdkError::TransferFailed);
        }

        chunk_index += 1;
        remaining -= chunk_len;
    }

    Ok(())
}

pub fn display_picture_on_screen_cm_m4315(device: &HidDevice, data: &[u8]) -> CoolTekSdkResult<()> {
    unimplemented!()
}

fn get_animation_infos(device: &HidDevice) -> CoolTekSdkResult<Vec<u8>> {
    let mut raw = [0u8; 0x21];
    raw[0] = b'=';

    let _ = device
        .get_feature_report(&mut raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    if raw[0] != b'=' {
        return Err(CoolTekSdkError::InvalidResponse);
    }

    let len = raw[1] as usize;

    Ok(raw[2..2 + len].to_vec())
}

pub fn get_backlight_cm_m4315(device: &HidDevice) -> CoolTekSdkResult<u8> {
    let mut raw = [0u8; 0x21];
    raw[0] = 0x3c;

    let _ = device
        .get_feature_report(&mut raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    Ok(raw[2])
}

pub fn get_config_data_cm_m4315(
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
        '\0' => 0x3a,
        _ => 0x39,
    };

    let _ = device
        .write(&mut raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    Ok(())
}

pub fn get_config_infos_cm_m4315(device: &HidDevice) -> CoolTekSdkResult<Vec<u8>> {
    let mut raw = [0u8; 0x21];
    raw[0] = 0x3e;

    let _ = device
        .get_feature_report(&mut raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    let len = raw[1] as usize;

    Ok(raw[2..2 + len].to_vec())
}

pub fn get_device_infos_cm_m4315(device: &HidDevice) -> CoolTekSdkResult<Vec<u8>> {
    let mut raw = [0u8; 0x21];
    raw[0] = b';';

    let _ = device
        .get_feature_report(&mut raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    if raw[0] != b';' {
        return Err(CoolTekSdkError::InvalidResponse);
    }

    let len = raw[1] as usize;

    Ok(raw[2..2 + len].to_vec())
}

pub fn get_fw_infos_by_type_cm_m4315(
    device: &HidDevice,
    component: u8,
) -> CoolTekSdkResult<(u32, String)> {
    if component > 0x02 {
        return Err(CoolTekSdkError::InvalidParameter);
    }

    let mut raw = [0u8; 0x21];
    raw[0] = match component {
        0x00 => 0x37,
        0x01 => 0x38,
        0x02 => 0x39,
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

pub fn get_fw_infos_cm_m4315(device: &HidDevice) -> CoolTekSdkResult<(u32, String)> {
    get_fw_infos_by_type_cm_m4315(device, 0x01)
}

pub fn get_serial_number_cm_m4315(device: &HidDevice) -> CoolTekSdkResult<Option<String>> {
    let mut raw = [0u8; 0x21];
    raw[0] = 0x3a;

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

pub fn save_factory_data_cm_m4315(device: &HidDevice) -> CoolTekSdkResult<()> {
    let mut raw = [0u8; 0x20];
    raw[0] = 0x03;
    raw[1] = 0x3c;

    let _ = device
        .send_feature_report(&raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    Ok(())
}

fn save_screen_picture_cm() {
    unimplemented!()
}

pub fn set_serial_number_cm_m4315(device: &HidDevice, serial_number: &str) -> CoolTekSdkResult<()> {
    if serial_number.len() > 0x1d {
        return Err(CoolTekSdkError::InvalidParameter);
    }

    let mut raw = [0u8; 0x20];
    raw[0] = 0x03;
    raw[1] = 0x35;
    raw[2] = serial_number.len() as u8;
    raw[3..serial_number.len()].copy_from_slice(serial_number.as_bytes());

    let _ = device
        .send_feature_report(&raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    Ok(())
}

pub fn set_sw_mode_cm_m4315(device: &HidDevice, data: u8) -> CoolTekSdkResult<()> {
    let mut raw = [0u8; 0x20];
    raw[0] = 0x03;
    raw[1] = 0x3a;
    raw[2] = data;

    let _ = device
        .send_feature_report(&raw)
        .map_err(|_| CoolTekSdkError::TransferFailed);

    Ok(())
}

pub fn test_animation_recovery_process_cm_m4315(device: &HidDevice) -> CoolTekSdkResult<()> {
    let mut raw = [0u8; 0x20];
    raw[0] = 0x03;
    raw[1] = 0x3e;

    let _ = device
        .send_feature_report(&raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    Ok(())
}

pub fn test_factory_reset_process_cm_m4315(device: &HidDevice) -> CoolTekSdkResult<()> {
    let mut raw = [0u8; 0x20];
    raw[0] = 0x03;
    raw[1] = 0x3d;

    let _ = device
        .send_feature_report(&raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    Ok(())
}

pub fn test_fw_recovery_process_cm_m4315(device: &HidDevice) -> CoolTekSdkResult<()> {
    let mut raw = [0u8; 0x20];
    raw[0] = 0x03;
    raw[1] = 0x39;

    let _ = device
        .send_feature_report(&raw)
        .map_err(|_| CoolTekSdkError::TransferFailed)?;

    Ok(())
}

pub fn update_fw_by_type_cm_m4315(
    device: &HidDevice,
    firmware: &[u8],
    fw_type: u32,
) -> CoolTekSdkResult<()> {
    unimplemented!()
}

pub fn update_fw_cm_m4315(device: &HidDevice, firmware: &[u8]) -> CoolTekSdkResult<()> {
    update_fw_by_type_cm_m4315(device, firmware, 0x01)
}

// -------------------------------

const CHANNEL_TAGS: [u8; crate::CHANNEL_COUNT] = [b'4', b'5', b'6', b'7', b'8'];

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
        b'4' => 0,
        b'5' => 1,
        b'6' => 2,
        b'7' => 3,
        b'8' => 4,
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
