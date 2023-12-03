use crate::registry_record::RegistryRecord;

#[derive(Clone, Copy)]
pub struct Registry {
    registry_pos: usize,
    registry_packet_len: usize,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            registry_pos: 0,
            registry_packet_len: 32,
        }
    }

    pub fn init(&self) {
        let mut registry_record = RegistryRecord::new();

        registry_record.rmb_mb_id = 0;
        registry_record.rmb_mb_packet_len = self.registry_packet_len as u8;
        registry_record.rmb_mb_position = self.registry_pos();
        registry_record.rmb_mb_len = self.registry_pos() + self.registry_packet_len;
        registry_record.rmb_mb_capacity = 65536;
        registry_record.rmb_packet_position = self.registry_pos();

        self.update(registry_record);
    }

    pub fn update(&self, registry_record: RegistryRecord) {
        let registry_record_ptr = registry_record.rmb_packet_position as *mut RegistryRecord;

        unsafe {
            *registry_record_ptr = registry_record;
        }
    }

    pub fn insert(&mut self, mut registry_record: RegistryRecord) {
        let registry_record_ptr = self.registry_pos() as *mut RegistryRecord;

        let new_record_position = unsafe {
            let current_rmb_mb_len = (*registry_record_ptr).rmb_mb_len;

            (*registry_record_ptr).rmb_mb_len = current_rmb_mb_len + self.registry_packet_len;

            current_rmb_mb_len
        };

        registry_record.rmb_packet_position = new_record_position;

        self.update(registry_record);
    }

    pub fn find_record_by_rmb_mb_id(&self, rmb_mb_id: u8) -> RegistryRecord {
        let result = RegistryRecord::new();
        let registry_record_ptr = self.registry_pos() as *mut RegistryRecord;

        let rmb_mb_len = unsafe {
            (*registry_record_ptr).rmb_mb_len
        };

        if self.registry_packet_len == 0 {
            return result;
        }

        for memory_block_pos in (self.registry_pos()..rmb_mb_len).step_by(self.registry_packet_len) {
            let search_registry_record_ptr = memory_block_pos as *mut RegistryRecord;

            if search_registry_record_ptr.is_null() {
                continue;
            }

            let search_rmb_mb_id = unsafe {
                (*search_registry_record_ptr).rmb_mb_id
            };

            if search_rmb_mb_id == rmb_mb_id {
                let mut search_registry_record = RegistryRecord::new();

                unsafe {
                    search_registry_record.rmb_mb_id = (*search_registry_record_ptr).rmb_mb_id;
                    search_registry_record.rmb_mb_position = (*search_registry_record_ptr).rmb_mb_position;
                    search_registry_record.rmb_mb_len = (*search_registry_record_ptr).rmb_mb_len;
                    search_registry_record.rmb_mb_capacity = (*search_registry_record_ptr).rmb_mb_capacity;
                    search_registry_record.rmb_mb_packet_len = (*search_registry_record_ptr).rmb_mb_packet_len;
                    search_registry_record.rmb_packet_position = (*search_registry_record_ptr).rmb_packet_position;
                };

                return search_registry_record;
            }
        }

        return result;
    }

    fn registry_pos(&self) -> usize {
        // skip zero-byte packet
        self.registry_pos + self.registry_packet_len
    }
}