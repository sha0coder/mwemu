use crate::{maps::Maps, structures::unicode_string64::UnicodeString64};

#[derive(Debug)]
pub struct RtlUserProcessParameters64 {
    pub reserved1: [u8; 16],
    pub reserved2: [u64; 10],
    pub image_path_name: UnicodeString64,
    pub command_line: UnicodeString64,
}

impl Default for RtlUserProcessParameters64 {
    fn default() -> Self {
        Self::new()
    }
}

impl RtlUserProcessParameters64 {
    pub fn new() -> Self {
        Self {
            reserved1: [0; 16],
            reserved2: [0; 10],
            image_path_name: UnicodeString64::new(),
            command_line: UnicodeString64::new(),
        }
    }

    pub fn size() -> usize {
        128
    }

    pub fn save(&mut self, addr: u64, maps: &mut Maps) {
        maps.write_bytes(addr, self.reserved1.to_vec());
        for (i, val) in self.reserved2.iter().enumerate() {
            maps.write_qword(addr + 16 + (i * 8) as u64, *val);
        }
        self.image_path_name.save(addr + 16 + 80, maps);
        self.command_line.save(addr + 16 + 80 + 16, maps)
    }
}
