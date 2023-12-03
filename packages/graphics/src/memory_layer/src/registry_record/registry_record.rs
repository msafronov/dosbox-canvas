#[repr(C)]
pub struct RegistryRecord {
    pub rmb_mb_id: u8,
    pub rmb_mb_packet_len: u8,
    pub reserved: u16,
    pub rmb_mb_position: usize,
    pub rmb_mb_len: usize,
    pub rmb_mb_capacity: usize,
    pub rmb_packet_position: usize,
}

impl RegistryRecord {
    pub fn new() -> Self {
        RegistryRecord {
            rmb_mb_id: 0,
            rmb_mb_packet_len: 0,
            reserved: 0,
            rmb_mb_position: 0,
            rmb_mb_len: 0,
            rmb_mb_capacity: 0,
            rmb_packet_position: 0,
        }
    }
}