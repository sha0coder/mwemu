use crate::windows::structures;

use super::PE64;
use crate::loaders::pe::readers::{
    read_u16_le as read_u16_le_shared, read_u32_le as read_u32_le_shared,
};

macro_rules! read_u16_le {
    ($raw:expr, $off:expr) => {
        read_u16_le_shared(($raw).as_ref(), $off)
    };
}

macro_rules! read_u32_le {
    ($raw:expr, $off:expr) => {
        read_u32_le_shared(($raw).as_ref(), $off)
    };
}

impl PE64 {
    pub(crate) fn pe64_locate_resource_data_entry(
        &self,
        rsrc: &[u8],
        off: usize,
        level: u32,
        type_id: Option<u32>,
        name_id: Option<u32>,
        type_name: Option<&str>,
        name: Option<&str>,
    ) -> Option<structures::ImageResourceDataEntry64> {
        if level >= 10 {
            log::warn!("Resource directory recursion limit reached");
            return None;
        }

        if off + 16 > rsrc.len() {
            log::warn!(
                "Resource directory at offset {} is out of bounds (rsrc size: {})",
                off,
                rsrc.len()
            );
            return None;
        }

        let mut dir = structures::ImageResourceDirectory::new();
        dir.characteristics = read_u32_le!(rsrc, off);
        dir.time_date_stamp = read_u32_le!(rsrc, off + 4);
        dir.major_version = read_u16_le!(rsrc, off + 8);
        dir.minor_version = read_u16_le!(rsrc, off + 10);
        dir.number_of_named_entries = read_u16_le!(rsrc, off + 12);
        dir.number_of_id_entries = read_u16_le!(rsrc, off + 14);

        let entries = dir.number_of_named_entries + dir.number_of_id_entries;

        for i in 0..entries {
            let entry_off = off + (i as usize * 8) + 16;
            if entry_off + 8 > rsrc.len() {
                log::warn!(
                    "Resource directory entry {} at offset {} is out of bounds",
                    i,
                    entry_off
                );
                continue;
            }

            let mut entry = structures::ImageResourceDirectoryEntry::new();
            entry.name_or_id = read_u32_le!(rsrc, entry_off);
            entry.data_or_directory = read_u32_le!(rsrc, entry_off + 4);

            let matched: bool;

            if entry.is_id() {
                let entry_id = entry.get_name_or_id();
                if level == 0 && type_id.is_some() && type_id.unwrap() == entry_id {
                    matched = true;
                } else if level == 1 && name_id.is_some() && name_id.unwrap() == entry_id {
                    matched = true;
                } else if level == 2 {
                    matched = true;
                } else {
                    matched = false;
                }
            } else {
                let name_offset = (entry.get_name_or_id() & 0x7FFFFFFF) as usize;
                let rsrc_section = self.get_section_ptr_by_name(".rsrc");
                if rsrc_section.is_none() {
                    continue;
                }

                if name_offset >= rsrc.len() {
                    continue;
                }

                let resource_name = self.pe64_read_resource_name_from_rsrc(rsrc, name_offset);
                if level == 0 && type_name.is_some() && type_name.unwrap() == resource_name {
                    matched = true;
                } else if level == 1 && name.is_some() && name.unwrap() == resource_name {
                    matched = true;
                } else {
                    matched = false;
                }
            }

            if matched {
                if entry.is_directory() {
                    let next_dir_offset = entry.get_offset() & 0x7FFFFFFF;
                    return self.pe64_locate_resource_data_entry(
                        rsrc,
                        next_dir_offset as usize,
                        level + 1,
                        type_id,
                        name_id,
                        type_name,
                        name,
                    );
                } else {
                    let data_entry_offset = entry.get_offset();
                    if data_entry_offset as usize + 16 > rsrc.len() {
                        return None;
                    }

                    let mut data_entry = structures::ImageResourceDataEntry64::new();
                    data_entry.offset_to_data =
                        read_u32_le!(rsrc, data_entry_offset as usize) as u64;
                    data_entry.size = read_u32_le!(rsrc, data_entry_offset as usize + 4) as u64;
                    data_entry.code_page =
                        read_u32_le!(rsrc, data_entry_offset as usize + 8) as u64;
                    data_entry.reserved =
                        read_u32_le!(rsrc, data_entry_offset as usize + 12) as u64;

                    return Some(data_entry);
                }
            }
        }

        None
    }

    pub(crate) fn pe64_read_resource_name_from_rsrc(
        &self,
        rsrc: &[u8],
        offset: usize,
    ) -> String {
        if offset + 1 >= rsrc.len() {
            return String::new();
        }

        let length = u16::from_le_bytes([rsrc[offset], rsrc[offset + 1]]) as usize;
        let string_start = offset + 2;

        let required_bytes = string_start + (length * 2);
        if required_bytes > rsrc.len() {
            return String::new();
        }

        let utf16_data: Vec<u16> = (0..length)
            .map(|i| {
                let idx = string_start + i * 2;
                u16::from_le_bytes([rsrc[idx], rsrc[idx + 1]])
            })
            .collect();

        String::from_utf16_lossy(&utf16_data)
    }

    pub(crate) fn pe64_get_resource(
        &self,
        type_id: Option<u32>,
        name_id: Option<u32>,
        type_name: Option<&str>,
        name: Option<&str>,
    ) -> Option<(u64, usize)> {
        let rsrc = self.get_section_ptr_by_name(".rsrc")?;
        let data_entry =
            self.pe64_locate_resource_data_entry(rsrc, 0, 0, type_id, name_id, type_name, name)?;
        let data_off = PE64::vaddr_to_off(&self.sect_hdr, data_entry.offset_to_data as u32)
            as usize
            - self.opt.image_base as usize;
        Some((data_off as u64, data_entry.size as usize))
    }

    pub(crate) fn pe64_get_resource_name(
        &self,
        entry: &structures::ImageResourceDirectoryEntry,
    ) -> String {
        let rsrc = match self.get_section_ptr_by_name(".rsrc") {
            Some(rsrc) => rsrc,
            None => return String::new(),
        };

        let name_offset = (entry.get_name_or_id() & 0x7FFFFFFF) as usize;
        self.pe64_read_resource_name_from_rsrc(rsrc, name_offset)
    }
}
