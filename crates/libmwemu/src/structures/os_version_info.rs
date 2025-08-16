use crate::maps::Maps;

#[derive(Debug)]
pub struct OsVersionInfo {
    version_info_size: u32,
    major_version: u32,
    minor_version: u32,
    build_number: u32,
    platform_id: u32,
    version: [u8; 128],
}

impl Default for OsVersionInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl OsVersionInfo {
    pub fn new() -> OsVersionInfo {
        let mut ovi = OsVersionInfo {
            version_info_size: 284,
            major_version: 10,
            minor_version: 0,
            build_number: 19042,
            platform_id: 2,
            version: [0; 128],
        };

        "Service Pack 0"
            .as_bytes()
            .iter()
            .enumerate()
            .for_each(|(i, &byte)| {
                ovi.version[i] = byte;
            });

        ovi
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.version_info_size);
        maps.write_dword(addr + 4, self.major_version);
        maps.write_dword(addr + 8, self.minor_version);
        maps.write_dword(addr + 12, self.build_number);
        maps.write_dword(addr + 16, self.platform_id);
        maps.write_buffer(addr + 20, &self.version);
    }
}
