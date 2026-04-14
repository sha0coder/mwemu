use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crate::emu;
use crate::loaders::pe::pe32::PE32;
use crate::windows::structures;

use crate::loaders::pe::readers::{
    read_c_string, read_u64_le as read_u64_le_shared,
};
use super::{
    DelayLoadDirectory, ImageDosHeader, ImageExportDirectory, ImageFileHeader,
    ImageImportDescriptor, ImageNtHeaders, ImageOptionalHeader64, ImageSectionHeader, PE64,
    TlsDirectory64, IMAGE_DIRECTORY_ENTRY_DELAY_LOAD, IMAGE_DIRECTORY_ENTRY_EXPORT,
    IMAGE_DIRECTORY_ENTRY_IAT, IMAGE_DIRECTORY_ENTRY_IMPORT, IMAGE_DIRECTORY_ENTRY_TLS,
    IMAGE_FILE_DLL, SECTION_HEADER_SZ,
};

macro_rules! read_u64_le {
    ($raw:expr, $off:expr) => {
        read_u64_le_shared(($raw).as_ref(), $off)
    };
}

impl PE64 {
    pub fn is_pe64(filename: &str) -> bool {
        let mut fd = File::open(filename).expect("file not found");
        let mut raw = vec![0u8; ImageDosHeader::size()];
        fd.read_exact(&mut raw).expect("couldnt read the file");
        let dos = ImageDosHeader::load(&raw, 0);

        if dos.e_magic != 0x5a4d {
            return false;
        }

        if dos.e_lfanew >= fd.metadata().unwrap().len() as u32 {
            return false;
        }

        true
    }

    pub fn load_from_raw(filename: &str, raw: &[u8]) -> PE64 {
        let dos = ImageDosHeader::load(raw, 0);
        let nt = ImageNtHeaders::load(raw, dos.e_lfanew as usize);
        let fh = ImageFileHeader::load(raw, dos.e_lfanew as usize + 4);
        let opt = ImageOptionalHeader64::load(raw, dos.e_lfanew as usize + 24);
        let mut sect: Vec<ImageSectionHeader> = Vec::new();

        let mut off = dos.e_lfanew as usize + 24 + fh.size_of_optional_header as usize;
        for _ in 0..fh.number_of_sections {
            let s = ImageSectionHeader::load(raw, off);
            sect.push(s);
            off += SECTION_HEADER_SZ;
        }

        let import_va = opt.data_directory[IMAGE_DIRECTORY_ENTRY_IMPORT].virtual_address;
        let export_va = opt.data_directory[IMAGE_DIRECTORY_ENTRY_EXPORT].virtual_address;
        let delay_load_va = opt.data_directory[IMAGE_DIRECTORY_ENTRY_DELAY_LOAD].virtual_address;
        let mut import_off: usize;
        let mut delay_load_off: usize;

        let mut image_import_descriptor: Vec<ImageImportDescriptor> = Vec::new();
        let mut delay_load_dir: Vec<DelayLoadDirectory> = Vec::new();

        if delay_load_va > 0 {
            delay_load_off = PE64::vaddr_to_off(&sect, delay_load_va) as usize;
            if delay_load_off > 0 {
                loop {
                    let mut delay_load = DelayLoadDirectory::load(raw, delay_load_off);
                    if delay_load.handle == 0 || delay_load.name_ptr == 0 {
                        break;
                    }

                    let off = PE64::vaddr_to_off(&sect, delay_load.name_ptr) as usize;
                    if off > raw.len() {
                        panic!("the delay_load.name of pe64 is out of buffer");
                    }
                    let libname = PE32::read_string(raw, off);
                    delay_load.name = libname.to_string();
                    delay_load_dir.push(delay_load);
                    delay_load_off += DelayLoadDirectory::size();
                }
            }
        }

        if import_va > 0 {
            import_off = PE64::vaddr_to_off(&sect, import_va) as usize;

            if import_off > 0 {
                loop {
                    let mut iid = ImageImportDescriptor::load(raw, import_off);
                    if iid.name_ptr == 0 {
                        break;
                    }
                    let off = PE64::vaddr_to_off(&sect, iid.name_ptr) as usize;
                    if off > raw.len() {
                        panic!("the name of pe64 iid is out of buffer");
                    }

                    let libname = PE32::read_string(raw, off);
                    iid.name = libname.to_string();

                    image_import_descriptor.push(iid);
                    import_off += ImageImportDescriptor::size();
                }
            }
        }

        let _exportd: Option<ImageExportDirectory> = if export_va > 0 { None } else { None };

        PE64 {
            filename: filename.to_string(),
            raw: raw.to_vec(),
            dos,
            fh,
            nt,
            opt,
            sect_hdr: sect,
            delay_load_dir,
            image_import_descriptor,
        }
    }

    pub fn load(filename: &str) -> PE64 {
        let mut fd = File::open(filename).expect("pe64 binary not found");
        let mut raw: Vec<u8> = Vec::new();
        fd.read_to_end(&mut raw)
            .expect("couldnt read the pe64 binary");
        PE64::load_from_raw(filename, &raw)
    }

    pub fn size(&self) -> u64 {
        self.raw.len() as u64
    }

    pub fn mem_size(&self) -> usize {
        let mut sz = 0;
        for sect in &self.sect_hdr {
            if sect.virtual_size > sect.size_of_raw_data {
                sz += sect.virtual_size as usize;
            } else {
                sz += sect.size_of_raw_data as usize;
            }
        }
        sz
    }

    pub fn is_dll(&self) -> bool {
        self.fh.characteristics & IMAGE_FILE_DLL != 0
    }

    pub fn get_raw(&self) -> &[u8] {
        &self.raw[0..self.raw.len()]
    }

    pub fn get_headers(&self) -> &[u8] {
        &self.raw[0..self.opt.size_of_headers as usize]
    }

    pub fn clear(&mut self) {
        self.raw.clear();
        self.sect_hdr.clear();
    }

    pub fn vaddr_to_off(sections: &Vec<ImageSectionHeader>, vaddr: u32) -> u32 {
        for sect in sections {
            if vaddr >= sect.virtual_address && vaddr < sect.virtual_address + sect.virtual_size {
                let offset_within_section = vaddr - sect.virtual_address;

                if offset_within_section >= sect.size_of_raw_data {
                    log::warn!(
                        "Virtual address 0x{:x} maps to uninitialized data in section '{}' (offset {} >= raw_size {})",
                        vaddr,
                        sect.get_name(),
                        offset_within_section,
                        sect.size_of_raw_data
                    );
                    return 0;
                }

                let file_offset = sect.pointer_to_raw_data + offset_within_section;
                return file_offset;
            }
        }

        0
    }

    pub fn read_string(raw: &[u8], off: usize) -> String {
        read_c_string(raw, off)
    }

    pub fn num_of_sections(&self) -> usize {
        self.sect_hdr.len()
    }

    pub fn get_section_ptr_by_name(&self, name: &str) -> Option<&[u8]> {
        for sect in &self.sect_hdr {
            if sect.get_name() == name {
                let off = sect.pointer_to_raw_data as usize;
                let sz = sect.virtual_size as usize;
                let section_ptr = &self.raw[off..off + sz];
                return Some(section_ptr);
            }
        }
        None
    }

    pub fn get_section(&self, id: usize) -> &ImageSectionHeader {
        &self.sect_hdr[id]
    }

    pub fn get_pe_off(&self) -> u32 {
        self.dos.e_lfanew
    }

    pub fn get_section_ptr(&self, id: usize) -> &[u8] {
        if id > self.sect_hdr.len() {
            panic!("/!\\ warning: invalid section id {}", id);
        }
        let off = self.sect_hdr[id].pointer_to_raw_data as usize;
        let sz = self.sect_hdr[id].size_of_raw_data as usize;
        if off + sz > self.raw.len() {
            log::trace!(
                "/!\\ warning: id:{} name:{} raw sz:{} off:{} sz:{}  off+sz:{}",
                id,
                self.sect_hdr[id].get_name(),
                self.raw.len(),
                off,
                sz,
                off + sz
            );
            if off > self.raw.len() {
                return &[];
            }
            return &self.raw[off..];
        }
        &self.raw[off..off + sz]
    }

    pub fn get_section_vaddr(&self, id: usize) -> u32 {
        self.sect_hdr[id].virtual_address
    }

    pub fn get_tls_callbacks(&self, _vaddr: u32) -> Vec<u64> {
        let mut callbacks: Vec<u64> = Vec::new();

        if self.opt.data_directory.len() < IMAGE_DIRECTORY_ENTRY_TLS {
            log::trace!("/!\\ alert there is .tls section but not tls directory entry");
            return callbacks;
        }

        let entry_tls = self.opt.data_directory[IMAGE_DIRECTORY_ENTRY_TLS].virtual_address;
        let _iat = self.opt.data_directory[IMAGE_DIRECTORY_ENTRY_IAT].virtual_address;
        let _align = self.opt.file_alignment;

        let tls_off = PE64::vaddr_to_off(&self.sect_hdr, entry_tls) as usize;

        let tls = TlsDirectory64::load(&self.raw, tls_off);
        tls.print();

        let mut cb_off = PE64::vaddr_to_off(&self.sect_hdr, (tls.tls_callbacks & 0xffff) as u32);
        loop {
            let callback: u64 = read_u64_le!(&self.raw, cb_off as usize);
            if callback == 0 {
                break;
            }
            log::trace!("0x{:x} TLS Callback: 0x{:x}", cb_off, callback);
            callbacks.push(callback);
            cb_off += 8;
        }

        callbacks
    }

    pub fn delay_load_binding(&mut self, emu: &mut emu::Emu, base_addr: u64) {
        self.pe64_delay_load_binding(emu, base_addr);
    }

    pub fn get_dependencies(&mut self, emu: &mut emu::Emu) -> Vec<String> {
        self.pe64_get_dependencies(emu)
    }

    pub fn iat_binding(&mut self, emu: &mut emu::Emu, base_addr: u64) {
        self.pe64_iat_binding(emu, base_addr);
    }

    pub fn iat_binding_alternative(
        &mut self,
        emu: &mut emu::Emu,
        base_addr: u64,
        first_thunk: u32,
        import_dll: &str,
        resolved_cache: &mut HashMap<String, u64>,
    ) {
        self.pe64_iat_binding_alternative(
            emu,
            base_addr,
            first_thunk,
            import_dll,
            resolved_cache,
        );
    }

    pub fn iat_binding_original(
        &mut self,
        emu: &mut emu::Emu,
        base_addr: u64,
        original_first_thunk: u32,
        first_thunk: u32,
        import_dll: &str,
        resolved_cache: &mut HashMap<String, u64>,
    ) {
        self.pe64_iat_binding_original(
            emu,
            base_addr,
            original_first_thunk,
            first_thunk,
            import_dll,
            resolved_cache,
        );
    }

    pub fn apply_relocations(&mut self, emu: &mut emu::Emu, base_addr: u64) {
        self.pe64_apply_relocations(emu, base_addr);
    }

    pub fn import_addr_to_name(&self, paddr: u64) -> String {
        self.pe64_import_addr_to_name(paddr)
    }

    pub fn import_addr_to_dll_and_name(&self, paddr: u64) -> String {
        self.pe64_import_addr_to_dll_and_name(paddr)
    }

    pub fn locate_resource_data_entry(
        &self,
        rsrc: &[u8],
        off: usize,
        level: u32,
        type_id: Option<u32>,
        name_id: Option<u32>,
        type_name: Option<&str>,
        name: Option<&str>,
    ) -> Option<structures::ImageResourceDataEntry64> {
        self.pe64_locate_resource_data_entry(rsrc, off, level, type_id, name_id, type_name, name)
    }

    pub fn read_resource_name_from_rsrc(&self, rsrc: &[u8], offset: usize) -> String {
        self.pe64_read_resource_name_from_rsrc(rsrc, offset)
    }

    pub fn get_resource(
        &self,
        type_id: Option<u32>,
        name_id: Option<u32>,
        type_name: Option<&str>,
        name: Option<&str>,
    ) -> Option<(u64, usize)> {
        self.pe64_get_resource(type_id, name_id, type_name, name)
    }

    pub fn get_resource_name(&self, entry: &structures::ImageResourceDirectoryEntry) -> String {
        self.pe64_get_resource_name(entry)
    }
}
