use hidapi::HidDevice;

#[derive(Debug)]
pub struct DeviceHandle {
    pub device: Option<HidDevice>,
    pub packet_size: usize,
    pub read_buffer: Vec<u8>,
    pub ring_buffer: RingBuffer,
}

impl DeviceHandle {
    pub fn new(device: HidDevice, packet_size: usize) -> Self {
        Self {
            device: Some(device),
            packet_size,
            read_buffer: vec![0u8; packet_size],
            ring_buffer: RingBuffer::new(packet_size),
        }
    }
}

// -------------------------------

pub const RING_SIZE: usize = 64;
pub const CHANNEL_COUNT: usize = 5;

#[derive(Debug)]
pub struct RingBuffer {
    pub slots: [[Vec<u8>; CHANNEL_COUNT]; RING_SIZE],
    pub occupied_count: [u8; CHANNEL_COUNT],
    pub write_head: [u8; CHANNEL_COUNT],
    pub read_tail: [u8; CHANNEL_COUNT],
}

impl RingBuffer {
    pub fn new(packet_size: usize) -> Self {
        Self {
            slots: std::array::from_fn(|_| std::array::from_fn(|_| vec![0u8; packet_size])),
            occupied_count: [0; CHANNEL_COUNT],
            write_head: [0; CHANNEL_COUNT],
            read_tail: [0; CHANNEL_COUNT],
        }
    }
}
