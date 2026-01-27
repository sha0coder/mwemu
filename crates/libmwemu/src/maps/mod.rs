pub mod heap_allocation;
pub mod mem64;
pub mod tlb;

use crate::constants;
use crate::maps::mem64::Permission;
use crate::maps::tlb::LPF_OF;
use ahash::AHashMap;
use mem64::Mem64;
use serde::{Deserialize, Serialize};
use slab::Slab;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::str;
use tlb::TLB;

#[derive(Clone, Serialize, Deserialize)]
pub struct Maps {
    pub banzai: bool,
    // adding slab so that it is easier to manage memory, now every other place contain the
    // key to the memory slab
    pub mem_slab: Slab<Mem64>,
    pub maps: BTreeMap<u64, usize>,
    pub name_map: AHashMap<String, usize>,
    pub is_64bits: bool,
    tlb: RefCell<TLB>,
}

impl Default for Maps {
    fn default() -> Self {
        Maps {
            mem_slab: Slab::with_capacity(200),
            maps: BTreeMap::<u64, usize>::default(),
            name_map: AHashMap::<String, usize>::with_capacity(200),
            is_64bits: false,
            banzai: false,
            tlb: RefCell::new(TLB::new()),
        }
    }
}

impl Maps {
    const DEFAULT_ALIGNMENT: u64 = 0x1000; //16;

    pub fn new(
        mem_slab: Slab<Mem64>,
        maps: BTreeMap<u64, usize>,
        name_map: AHashMap<String, usize>,
        is_64bits: bool,
        banzai: bool,
        tlb: RefCell<TLB>,
    ) -> Maps {
        Maps {
            banzai,
            mem_slab,
            maps,
            name_map,
            is_64bits,
            tlb,
        }
    }

    pub fn set_banzai(&mut self, banzai: bool) {
        self.banzai = banzai;
    }

    pub fn clear(&mut self) {
        self.mem_slab.clear();
        self.maps.clear();
        self.name_map.clear();
    }

    pub fn get_base(&self) -> Option<u64> {
        self.mem_slab
            .iter()
            .find(|map| map.1.get_name().ends_with(".pe"))
            .map(|map| map.1.get_base())
    }

    pub fn overflow_predicted(&self, addr: u64, amount: u64) -> bool {
        self.maps
            .range(..=addr)
            .next_back()
            .and_then(|(start, region_key)| {
                let region = self.mem_slab.get(*region_key)?;
                let start = *start;
                let size = region.size() as u64;

                if addr >= start && addr < start + size {
                    if addr + amount < start + size {
                        Some(false)
                    } else {
                        Some(true)
                    }
                } else {
                    Some(true)
                }
            })
            .unwrap_or(true)
    }

    #[inline(always)]
    pub fn exists_mapname(&self, name: &str) -> bool {
        self.name_map.contains_key(name)
    }

    // slow, better hold the object
    pub fn get_map_by_name(&self, name: &str) -> Option<&Mem64> {
        self.name_map.get(name).and_then(|v| self.mem_slab.get(*v))
    }

    pub fn get_map_by_name_mut(&mut self, name: &str) -> Option<&mut Mem64> {
        let name_key = self.name_map.get(name)?;
        self.mem_slab.get_mut(*name_key)
    }

    pub fn get_mem_size(&self, addr: u64) -> Option<usize> {
        self.maps
            .range(..=addr)
            .next_back()
            .and_then(|(start, region_key)| {
                let region = self.mem_slab.get(*region_key)?;
                let start = *start;
                let size = region.size() as u64;
                if addr >= start && addr < start + size {
                    Some(region.size())
                } else {
                    None
                }
            })
    }

    pub fn create_map(
        &mut self,
        name: &str,
        base: u64,
        size: u64,
        permission: Permission,
    ) -> Result<&mut Mem64, String> {
        //if size == 0 {
        //    return Err(format!("map size cannot be 0"));
        //}

        if self.get_mem_by_addr_mut(base).is_some() {
            return Err(format!("this map address 0x{:x} already exists!", base));
        }

        if self.exists_mapname(name) {
            self.show_maps();
            return Err(format!("this map name {} already exists!", name));
        }

        let mut mem = Mem64::default();
        mem.set_name(name);
        mem.set_base(base);
        mem.set_size(size);
        mem.set_permission(permission);

        let base_key = self.mem_slab.insert(mem);
        self.name_map.insert(name.to_string(), base_key);
        self.maps.insert(base, base_key);
        Ok(self.mem_slab.get_mut(base_key).unwrap())
    }

    pub fn write_byte(&mut self, addr: u64, value: u8) -> bool {
        let banzai = self.banzai;
        match self.get_mem_by_addr_mut(addr) {
            Some(mem) if mem.inside(addr) => {
                mem.write_byte(addr, value);
                true
            }
            Some(_) => {
                if banzai {
                    log::warn!("Writing byte to unmapped region at 0x{:x}", addr);
                } else {
                    panic!("Writing byte to unmapped region at 0x{:x}", addr);
                }
                false
            }
            None => {
                if banzai {
                    log::warn!("Writing byte to unmapped region at 0x{:x}", addr);
                } else {
                    panic!("Writing byte to unmapped region at 0x{:x}", addr);
                }
                false
            }
        }
    }

    pub fn read_byte(&self, addr: u64) -> Option<u8> {
        let banzai = self.banzai;
        match self.get_mem_by_addr(addr) {
            Some(mem) => Some(mem.read_byte(addr)),
            None if banzai => {
                log::warn!("Reading byte from unmapped region at 0x{:x}", addr);
                None
            }
            _ => None,
        }
    }

    pub fn read_f64(&self, addr: u64) -> Option<f64> {
        self.read_qword(addr).map(|v| f64::from_bits(v))
    }

    pub fn read_f32(&self, addr: u64) -> Option<f32> {
        self.read_dword(addr).map(|v| f32::from_bits(v))
    }

    pub fn write_f64(&mut self, addr: u64, value: f64) -> bool {
        self.write_qword(addr, value.to_bits())
    }

    pub fn write_f32(&mut self, addr: u64, value: f32) -> bool {
        self.write_dword(addr, value.to_bits())
    }

    pub fn write_qword(&mut self, addr: u64, value: u64) -> bool {
        let end_addr = addr + 7;
        let banzai = self.banzai;
        match self.get_mem_by_addr_mut(addr) {
            Some(mem) if mem.inside(end_addr) => {
                mem.write_qword(addr, value);
                true
            }
            Some(_) => {
                if banzai {
                    log::warn!("Writing qword to unmapped region at 0x{:x}", addr);
                } else {
                    panic!("Writing qword to unmapped region at 0x{:x}", addr);
                }
                false
            }
            None => {
                if banzai {
                    log::warn!("Writing qword to unmapped region at 0x{:x}", addr);
                } else {
                    panic!("Writing qword to unmapped region at 0x{:x}", addr);
                }
                false
            }
        }
    }

    pub fn write_dword(&mut self, addr: u64, value: u32) -> bool {
        let end_addr = addr + 3;
        let banzai = self.banzai;
        match self.get_mem_by_addr_mut(addr) {
            Some(mem) if mem.inside(end_addr) => {
                mem.write_dword(addr, value);
                true
            }
            Some(_) => {
                if banzai {
                    log::warn!("Writing dword to unmapped region at 0x{:x}", addr);
                } else {
                    panic!("Writing dword to unmapped region at 0x{:x}", addr);
                }
                false
            }
            None => {
                if banzai {
                    log::warn!("Writing dword to unmapped region at 0x{:x}", addr);
                } else {
                    panic!("Writing dword to unmapped region at 0x{:x}", addr);
                }
                false
            }
        }
    }

    pub fn write_word(&mut self, addr: u64, value: u16) -> bool {
        let end_addr = addr + 1;
        let banzai = self.banzai;

        match self.get_mem_by_addr_mut(addr) {
            Some(mem) if mem.inside(end_addr) => {
                mem.write_word(addr, value);
                true
            }
            Some(_) => {
                if banzai {
                    log::warn!("Writing word to unmapped region at 0x{:x}", addr);
                } else {
                    panic!("Writing word to unmapped region at 0x{:x}", addr);
                }
                false
            }
            None => {
                if banzai {
                    log::warn!("Writing word to unmapped region at 0x{:x}", addr);
                } else {
                    panic!("Writing word to unmapped region at 0x{:x}", addr);
                }
                false
            }
        }
    }

    pub fn write_bytes_slice(&mut self, addr: u64, data: &[u8]) -> bool {
        if data.is_empty() {
            return true;
        }

        // Write byte by byte to handle any boundary issues
        for (i, &byte) in data.iter().enumerate() {
            if !self.write_byte(addr + i as u64, byte) {
                return false;
            }
        }
        true
    }

    pub fn write_bytes(&mut self, addr: u64, data: Vec<u8>) -> bool {
        if data.is_empty() {
            return true;
        }

        // Write byte by byte to handle any boundary issues
        for (i, &byte) in data.iter().enumerate() {
            if !self.write_byte(addr + i as u64, byte) {
                return false;
            }
        }

        true
    }

    pub fn write_128bits_le(&mut self, addr: u64, value: u128) -> bool {
        let bytes = value.to_le_bytes().to_vec();
        self.write_bytes(addr, bytes)
    }

    pub fn write_128bits_be(&mut self, addr: u64, value: u128) -> bool {
        let bytes = value.to_be_bytes().to_vec();
        self.write_bytes(addr, bytes)
    }

    pub fn read_128bits_be(&self, addr: u64) -> Option<u128> {
        let end_addr = addr + 15;
        let banzai = self.banzai;
        match self.get_mem_by_addr(addr) {
            Some(mem) if mem.inside(end_addr) => {
                let b = self
                    .read_bytes_option(addr, 16)
                    .expect("fail to read 128bits");
                Some(u128::from_le_bytes(b.to_vec().try_into().unwrap()))
            }
            None if banzai => {
                log::warn!("Reading word from unmapped region at 0x{:x}", addr);
                None
            }
            _ => {
                panic!("Reading oword to unmapped region at 0x{:x}", addr);
            }
        }
    }

    pub fn read_128bits_le(&self, addr: u64) -> Option<u128> {
        let end_addr = addr + 15;
        let banzai = self.banzai;
        match self.get_mem_by_addr(addr) {
            Some(mem) if mem.inside(end_addr) => Some(mem.read_oword(addr)),
            None if banzai => {
                log::warn!("Reading oword from unmapped region at 0x{:x}", addr);
                None
            }
            _ => {
                panic!("Reading oword to unmapped region at 0x{:x}", addr);
            }
        }
    }

    pub fn read_qword(&self, addr: u64) -> Option<u64> {
        let end_addr = addr + 7;
        let banzai = self.banzai;
        match self.get_mem_by_addr(addr) {
            Some(mem) if mem.inside(end_addr) => Some(mem.read_qword(addr)),
            None if banzai => {
                log::warn!("Reading qword from unmapped region at 0x{:x}", addr);
                None
            }
            _ => {
                panic!("Reading qword to unmapped region at 0x{:x}", addr);
            }
        }
    }

    pub fn read_dword(&self, addr: u64) -> Option<u32> {
        let end_addr = addr + 3;
        let banzai = self.banzai;
        match self.get_mem_by_addr(addr) {
            Some(mem) if mem.inside(end_addr) => Some(mem.read_dword(addr)),
            None if banzai => {
                log::warn!("Reading dword from unmapped region at 0x{:x}", addr);
                None
            }
            _ => {
                log::warn!("Reading dword to unmapped region at 0x{:x}", addr);
                None
            }
        }
    }

    pub fn read_word(&self, addr: u64) -> Option<u16> {
        let end_addr = addr + 1;
        let banzai = self.banzai;
        match self.get_mem_by_addr(addr) {
            Some(mem) if mem.inside(end_addr) => Some(mem.read_word(addr)),
            None if banzai => {
                log::warn!("Reading dword from unmapped region at 0x{:x}", addr);
                None
            }
            _ => {
                panic!("Reading dword to unmapped region at 0x{:x}", addr);
            }
        }
    }

    pub fn get_mem_ref(&self, name: &str) -> &Mem64 {
        self.get_map_by_name(name)
            .expect("incorrect memory map name")
    }

    // deprecated
    pub fn get_mem(&self, name: &str) -> &Mem64 {
        self.get_map_by_name(name)
            .expect("incorrect memory map name")
    }

    pub fn get_mem_mut(&mut self, name: &str) -> &mut Mem64 {
        self.get_map_by_name_mut(name)
            .expect("incorrect memory map name")
    }

    #[inline(always)]
    pub fn get_mem2(&mut self, name: &str) -> Option<&mut Mem64> {
        self.get_map_by_name_mut(name)
    }

    #[inline(always)]
    pub fn get_mem_by_addr_mut(&mut self, addr: u64) -> Option<&mut Mem64> {
        let tlb_entry_mut = self.tlb.get_mut().get_entry_of_mut(addr, 0);
        let mem_key = tlb_entry_mut.get_mem();
        match self.mem_slab.get(mem_key) {
            Some(mem) => {
                if mem.inside(addr) {
                    return self.mem_slab.get_mut(tlb_entry_mut.mem64); // Clone the &Mem64
                }
            }
            _ => {
                tlb_entry_mut.invalidate();
            } // Remove the tlb entry
        };

        // TLB miss now search in the maps
        let mem_key_option = self
            .maps
            .range(..=addr)
            .next_back()
            .map(|(_start_addr, &key)| key);

        let mem_key = mem_key_option?;
        let mem_ref_mut = self.mem_slab.get_mut(mem_key)?;
        if !mem_ref_mut.inside(addr) {
            return None;
        }

        // Update TLB
        tlb_entry_mut.lpf = LPF_OF(addr);
        tlb_entry_mut.mem64 = mem_key;

        // Return back the memref
        Some(mem_ref_mut)
    }

    #[inline(always)]
    pub fn get_mem_by_addr(&self, addr: u64) -> Option<&Mem64> {
        let mut binding = self.tlb.borrow_mut();
        let entry = binding.get_entry_of(addr, 0);

        let mem_key = entry.get_mem();
        match self.mem_slab.get(mem_key) {
            Some(mem) => {
                if mem.inside(addr) {
                    return Some(&mem); // Clone the &Mem64
                }
            }
            _ => (), // TLB miss now search in maps
        };

        let mem_key_option = self.maps.range(..=addr).next_back().map(|(_k, &v)| v);

        let mem_key = mem_key_option?; // Return None if not found

        let mem_ref = self.mem_slab.get(mem_key)?;
        if !mem_ref.inside(addr) {
            return None;
        }

        // --- Update TLB ---
        let tlb_entry_mut = binding.get_entry_of_mut(addr, 0);
        tlb_entry_mut.lpf = LPF_OF(addr);
        tlb_entry_mut.mem64 = mem_key;
        Some(mem_ref)
    }

    #[inline(always)]
    pub fn memset(&mut self, addr: u64, b: u8, amount: usize) {
        for i in 0..amount {
            self.write_byte(addr + i as u64, b);
        }
    }

    pub fn memcpy(&mut self, to: u64, from: u64, size: usize) -> bool {
        let b = match self.read_bytes_option(from, size) {
            None => return false,
            Some(b) => b.to_vec(),
        };
        self.write_bytes(to, b);
        true
    }

    pub fn sizeof_wide(&self, unicode_str_ptr: u64) -> usize {
        let MAX_STR_LEN: usize = 1_000_000;
        let mut counter: usize = 0;

        for i in (0..MAX_STR_LEN).step_by(2) {
            let b = self
                .read_word(unicode_str_ptr + i as u64)
                .expect("maps.sizeof_wide controlled overflow");
            if b == 0 {
                return counter;
            }
            counter += 1;
        }

        0
    }

    pub fn write_string(&mut self, to: u64, from: &str) {
        //log::debug!("write_string 0x{:x}: `{}`", to, from);
        let bs: Vec<u8> = from.bytes().collect();

        self.write_bytes(to, bs.clone());
        self.write_byte(to + bs.len() as u64, 0x00);
    }

    pub fn write_wide_string(&mut self, to: u64, from: &str) {
        //log::debug!("write_wide_string 0x{:x}: `{}`", to, from);
        self.get_mem_by_addr_mut(to)
            .expect(format!("Cannot write wide string: Memory {} doesn't exists", to).as_str())
            .write_wide_string(to, from);
    }

    #[inline(always)]
    pub fn write_buffer(&mut self, to: u64, from: &[u8]) {
        self.write_bytes_slice(to, from);
    }

    pub fn read_bytes_buff(&self, buff: &mut [u8], addr: u64) {
        let mem = match self.get_mem_by_addr(addr) {
            Some(v) => v,
            None => panic!("Cannot read bytes: Memory {} doesn't exists", addr),
        };
        let len = buff.len();
        buff.copy_from_slice(mem.read_bytes(addr, len));
    }

    #[inline(always)]
    pub fn read_buffer(&mut self, from: u64, sz: usize) -> Vec<u8> {
        self.read_bytes_option(from, sz)
            .expect(format!("Fail to read buffer: from address doesn't exists {}", from).as_str())
            .to_vec()
    }

    pub fn print_maps_keyword(&self, kw: &str) {
        log::info!("--- maps ---");
        for (mem_name, base) in self.name_map.iter() {
            let mem = self.get_map_by_name(mem_name).unwrap();
            let k = mem_name;

            let n = if k.len() < 20 { 20 - k.len() } else { 1 };
            let mut spcs: String = String::new();
            for i in 0..n {
                spcs.push(' ');
            }
            if k.contains(kw) {
                log::info!(
                    "{}{}0x{:x} - 0x{:x} ({})",
                    k,
                    spcs,
                    mem.get_base(),
                    mem.get_bottom(),
                    mem.size()
                );
            }
        }
        log::info!("memory usage: {} bytes", self.size());
        log::info!("---");
    }

    pub fn print_maps(&self) {
        println!("print_maps");
        log::info!("--- maps ---");
        for (mem_name, base) in self.name_map.iter() {
            let mem = self.get_map_by_name(mem_name).unwrap();
            let k = mem_name;

            let n = if k.len() < 20 { 20 - k.len() } else { 1 };
            let mut spcs: String = String::new();
            for i in 0..n {
                spcs.push(' ');
            }
            log::info!(
                "{}{}0x{:x} - 0x{:x} ({})",
                k,
                spcs,
                mem.get_base(),
                mem.get_bottom(),
                mem.size()
            );
        }
        log::info!("memory usage: {} bytes", self.size());
        log::info!("---");
    }

    #[inline(always)]
    pub fn get_addr_base(&self, addr: u64) -> Option<u64> {
        self.get_mem_by_addr(addr).map(|mem| mem.get_base())
    }

    #[inline(always)]
    pub fn is_mapped(&self, addr: u64) -> bool {
        self.get_mem_by_addr(addr).is_some()
    }

    #[inline(always)]
    pub fn is_allocated(&self, addr: u64) -> bool {
        self.get_mem_by_addr(addr).is_some()
    }

    #[inline(always)]
    pub fn is_valid_ptr(&self, addr: u64) -> bool {
        addr > 0 && self.get_mem_by_addr(addr).is_some()
    }

    #[inline(always)]
    pub fn show_addr_names(&self, addr: u64) {
        self.get_mem_by_addr(addr).map(|mem| mem.get_name());
    }

    #[inline(always)]
    pub fn get_addr_name(&self, addr: u64) -> Option<&str> {
        self.get_mem_by_addr(addr).map(|mem| mem.get_name())
    }

    #[inline(always)]
    pub fn get_addr_name_mut(&mut self, addr: u64) -> Option<&str> {
        self.get_mem_by_addr(addr).map(|mem| mem.get_name())
    }

    pub fn dump(&self, addr: u64) {
        let mut count = 0;
        for i in 0..8 {
            let mut bytes: Vec<u8> = Vec::new();
            print!("0x{:x}: ", addr + i * 16);
            for _ in 0..16 {
                let b = self.read_byte(addr + count).unwrap_or(0);
                bytes.push(b);
                count += 1;
                print!("{:02x} ", b);
            }

            let pritable_bytes = self.filter_replace_bytes(&bytes);
            let s: String = match str::from_utf8(&pritable_bytes) {
                Ok(v) => v.to_string(),
                Err(n) => " -utf8err- ".to_string(),
            };

            println!("    {}", s);
        }
    }

    pub fn dump_n(&self, addr: u64, amount: u64) {
        let mut count: u64 = 0;
        for i in 0..8 {
            let mut bytes: Vec<u8> = Vec::new();
            print!("0x{:x}: ", addr + i * 16);
            for _ in 0..16 {
                let b = self.read_byte(addr + count).unwrap_or(0);
                bytes.push(b);
                count += 1;
                print!("{:02x} ", b);
                if count >= amount {
                    println!();
                    return;
                }
            }

            let pritable_bytes = self.filter_replace_bytes(&bytes);
            let s: String = match str::from_utf8(&pritable_bytes) {
                Ok(v) => v.to_string(),
                Err(n) => " -utf8err- ".to_string(),
            };

            println!("    {}", s);
        }
    }

    #[deprecated]
    pub fn dump2(&self, addr: u64) {
        let mut count = 0;
        for _ in 0..8 {
            let mut bytes: Vec<u8> = Vec::new();
            print!("0x{:x}: ", addr + count * 4);
            for _ in 0..4 {
                let dw = match self.read_dword(addr + count * 4) {
                    Some(v) => v,
                    None => {
                        log::info!("bad address");
                        return;
                    }
                };
                count += 1;
                bytes.push((dw & 0xff) as u8);
                bytes.push(((dw & 0xff00) >> 8) as u8);
                bytes.push(((dw & 0xff0000) >> 16) as u8);
                bytes.push(((dw & 0xff000000) >> 24) as u8);
                print!(
                    "{:02x} {:02x} {:02x} {:02x}  ",
                    dw & 0xff,
                    (dw & 0xff00) >> 8,
                    (dw & 0xff0000) >> 16,
                    (dw & 0xff000000) >> 24
                );
            }

            let pritable_bytes = self.filter_replace_bytes(&bytes);
            let s: String = match str::from_utf8(&pritable_bytes) {
                Ok(v) => v.to_string(),
                Err(n) => " -utf8err- ".to_string(),
            };

            log::info!("{}", s);
        }
    }

    pub fn dump_qwords(&self, addr: u64, n: u64) {
        let mut value: u64;

        for i in 0..n {
            let a = addr + i * 8;
            value = match self.read_qword(a) {
                Some(v) => v,
                None => break,
            };

            let name = self.get_addr_name(value).unwrap_or_else(|| "");

            log::info!(
                "0x{:x}: 0x{:x} ({}) '{}'",
                a,
                value,
                name,
                self.filter_replace_string(&self.read_string(value))
            );
        }
    }

    pub fn dump_dwords(&self, addr: u64, n: u64) {
        let mut value: u32;

        for i in 0..n {
            let a = addr + i * 4;
            value = match self.read_dword(a) {
                Some(v) => v,
                None => break,
            };

            if !self.is_64bits {
                // only in 32bits make sense derreference dwords in memory
                let name = self.get_addr_name(value.into()).unwrap_or_else(|| "");

                let mut s = "".to_string();
                if !name.is_empty() {
                    s = self.read_string(value.into());
                }

                log::info!(
                    "0x{:x}: 0x{:x} ({}) '{}'",
                    a,
                    value,
                    name,
                    self.filter_replace_string(&s)
                );
            } else {
                log::info!("0x{:x}: 0x{:x}", a, value);
            }
        }
    }

    pub fn read_bytes(&self, addr: u64, sz: usize) -> &[u8] {
        let mem = match self.get_mem_by_addr(addr) {
            Some(v) => v,
            None => panic!("Cannot read bytes: Memory {} doesn't exists", addr),
        };
        mem.read_bytes(addr, sz)
    }

    pub fn read_bytes_option(&self, addr: u64, sz: usize) -> Option<&[u8]> {
        self.get_mem_by_addr(addr)
            .map(|mem| mem.read_bytes(addr, sz))
    }

    pub fn read_string_of_bytes(&mut self, addr: u64, sz: usize) -> String {
        let mut svec: Vec<String> = Vec::new();
        let bytes = self.read_bytes(addr, sz);
        for bs in bytes.iter() {
            svec.push(format!("{:02x} ", bs));
        }
        let s: String = svec.into_iter().collect();
        s
    }

    pub fn read_string(&self, addr: u64) -> String {
        if addr == 0 {
            return "".to_string();
        }

        let mut bytes: Vec<char> = Vec::new();
        let mut b: u8;
        let mut i: u64 = 0;

        loop {
            b = match self.read_byte(addr + i) {
                Some(v) => v,
                None => break,
            };

            if b == 0x00 {
                break;
            }

            i += 1;
            bytes.push(b as char);
        }

        let s: String = bytes.into_iter().collect();
        s
    }

    pub fn read_wide_string_nocrash(&self, addr: u64) -> String {
        if addr == 0 {
            return "".to_string();
        }
        let mem = match self.get_mem_by_addr(addr) {
            Some(m) => m,
            None => {
                return "".to_string();
            }
        };
        mem.read_wide_string(addr)
    }

    pub fn read_wide_string(&self, addr: u64) -> String {
        if addr == 0 {
            return "".to_string();
        }
        let mem = self
            .get_mem_by_addr(addr)
            .expect(format!("No memory map found at 0x{:x}", addr).as_str());
        mem.read_wide_string(addr)
    }

    pub fn read_wide_string_n(&self, addr: u64, max_chars: usize) -> String {
        if addr == 0 {
            return "".to_string();
        }

        let mem = self
            .get_mem_by_addr(addr)
            .expect(format!("No memory map found at 0x{:x}", addr).as_str());
        mem.read_wide_string_n(addr, max_chars)
    }

    pub fn search_string(&self, kw: &str, map_name: &str) -> Option<Vec<u64>> {
        /*
        TODO: We can use AVX2 instructions to speed up the comparision but I don't know how to do it in rust
        reference: https://github.com/0x1F9F1/mem/blob/master/include/mem/simd_scanner.h
        maybe using https://github.com/greaka/patterns
         */
        let map = self.get_map_by_name(map_name);
        if map.is_none() {
            log::info!("map not found");
            return None;
        }
        let mem = map.unwrap();
        let bkw = kw.as_bytes();

        let mut found: Vec<u64> = Vec::new();
        for addr in mem.get_base()..mem.get_bottom() {
            let mut c = 0;

            for (i, bkwi) in bkw.iter().enumerate() {
                let b = mem.read_byte(addr + (i as u64));

                if b == *bkwi {
                    c += 1;
                } else {
                    break;
                }
            }

            if c == kw.len() {
                found.push(addr);
            }
        }

        if !found.is_empty() {
            Some(found)
        } else {
            log::info!("map not found");
            None
        }
    }

    pub fn write_spaced_bytes(&mut self, addr: u64, sbs: &str) -> bool {
        let mut waddr = addr;
        let bs: Vec<&str> = sbs.split(' ').collect();
        for bsi in bs.iter() {
            let b = u8::from_str_radix(bsi, 16).expect("bad num conversion");
            if !self.write_byte(waddr, b) {
                return false;
            }
            waddr += 1;
        }
        true
    }

    pub fn spaced_bytes_to_bytes(&self, sbs: &str) -> Vec<u8> {
        let bs: Vec<&str> = sbs.split(' ').collect();
        let mut bytes: Vec<u8> = Vec::new();
        for bsi in bs.iter() {
            let b = match u8::from_str_radix(bsi, 16) {
                Ok(b) => b,
                Err(_) => {
                    log::info!("bad hex bytes");
                    return bytes;
                }
            };
            bytes.push(b);
        }
        bytes
    }

    #[inline]
    fn is_pattern_match_at(memory: &Mem64, address: u64, pattern: &Vec<u8>) -> bool {
        for (i, &pattern_byte) in pattern.iter().enumerate() {
            let current_addr = address + (i as u64);

            // If we reach the end of the memory region, the pattern doesn't match
            if current_addr >= memory.get_bottom() {
                return false;
            }

            // If the byte doesn't match, the pattern doesn't match
            if memory.read_byte(current_addr) != pattern_byte {
                return false;
            }
        }

        // All bytes matched
        true
    }

    // search only one occurence from specific address
    pub fn search_spaced_bytes_from(&self, sbs: &str, saddr: u64) -> u64 {
        let byte_pattern = self.spaced_bytes_to_bytes(sbs);

        // Find the memory region containing the start address
        for (_, memory) in self.mem_slab.iter() {
            // Skip memory regions that don't contain the start address
            if saddr < memory.get_base() || saddr >= memory.get_bottom() {
                continue;
            }

            // Search backwards from start_address to base address
            for current_addr in memory.get_base()..=saddr {
                if Maps::is_pattern_match_at(memory, current_addr, &byte_pattern) {
                    return current_addr;
                }
            }

            // If we searched the entire memory region and didn't find a match, return 0
            return 0;
        }

        // No matching memory region found
        0
    }

    // search only one occurence from specific address backward
    pub fn search_spaced_bytes_from_bw(&self, spaced_bytes: &str, start_address: u64) -> u64 {
        let byte_pattern = self.spaced_bytes_to_bytes(spaced_bytes);

        // Find the memory region containing the start address
        for (_, memory) in self.mem_slab.iter() {
            // Skip memory regions that don't contain the start address
            if start_address < memory.get_base() || start_address >= memory.get_bottom() {
                continue;
            }

            // Search backwards from start_address to base address
            for current_addr in (memory.get_base()..=start_address).rev() {
                if Maps::is_pattern_match_at(memory, current_addr, &byte_pattern) {
                    return current_addr;
                }
            }

            // If we searched the entire memory region and didn't find a match, return 0
            return 0;
        }

        // No matching memory region found
        0
    }

    pub fn search_spaced_bytes(&self, sbs: &str, map_name: &str) -> Vec<u64> {
        let bytes = self.spaced_bytes_to_bytes(sbs);
        self.search_bytes(bytes, map_name)
    }

    pub fn search_spaced_bytes_in_all(&self, sbs: &str) -> Vec<u64> {
        let bytes = self.spaced_bytes_to_bytes(sbs);
        let mut found: Vec<u64> = Vec::new();

        for (_, mem) in self.mem_slab.iter() {
            for addr in mem.get_base()..mem.get_bottom() {
                if addr < 0x70000000 {
                    let mut c = 0;
                    for (i, bi) in bytes.iter().enumerate() {
                        let addri = addr + (i as u64);
                        if !mem.inside(addri) {
                            break;
                        }

                        let b = mem.read_byte(addri);
                        if b == *bi {
                            c += 1;
                        } else {
                            break;
                        }
                    }

                    if c == bytes.len() {
                        found.push(addr);
                    }
                }
            }
        }

        found
    }

    //TODO: return a list with matches.
    pub fn search_string_in_all(&self, kw: String) {
        let mut found = false;
        for (_, mem) in self.mem_slab.iter() {
            if mem.get_base() >= 0x7000000 {
                continue;
            }

            let results = match self.search_string(&kw, &mem.get_name()) {
                Some(v) => v,
                None => {
                    continue;
                }
            };

            for addr in results.iter() {
                if self.is_64bits {
                    log::info!("found at 0x{:x} '{}'", addr, self.read_string(*addr));
                } else {
                    log::info!(
                        "found at 0x{:x} '{}'",
                        *addr as u32,
                        self.read_string(*addr)
                    );
                }
                found = true;
            }
        }

        if !found {
            log::info!("not found.");
        }
    }

    pub fn search_bytes(&self, bkw: Vec<u8>, map_name: &str) -> Vec<u64> {
        let mut addrs: Vec<u64> = Vec::new();

        for (_, mem) in self.mem_slab.iter() {
            if mem.get_name() == map_name {
                for addr in mem.get_base()..mem.get_bottom() {
                    let mut c = 0;

                    for (i, bkwn) in bkw.iter().enumerate() {
                        if addr + i as u64 >= mem.get_bottom() {
                            break;
                        }
                        let b = mem.read_byte(addr + (i as u64));
                        if b == *bkwn {
                            c += 1;
                        } else {
                            break;
                        }
                    }

                    if c == bkw.len() {
                        addrs.push(addr);
                    }
                }

                return addrs;
            }
        }
        addrs
    }

    pub fn size(&self) -> usize {
        let mut sz: usize = 0;
        for (_, mem) in self.mem_slab.iter() {
            sz += mem.size();
        }
        sz
    }

    pub fn overlaps(&self, addr: u64, sz: u64) -> bool {
        for a in addr..addr + sz {
            if self.is_mapped(a) {
                return true;
            }
        }
        false
    }

    pub fn show_allocs(&self) {
        for (_, mem) in self.mem_slab.iter() {
            let name = mem.get_name();
            if name.starts_with("alloc_") || name.starts_with("valloc_") {
                log::info!(
                    "{} 0x{:x} - 0x{:x} ({})",
                    name,
                    mem.get_base(),
                    mem.get_bottom(),
                    mem.size()
                );
            }
        }
    }

    pub fn show_maps(&self) {
        for (_, mem) in self.mem_slab.iter() {
            let name = mem.get_name();
            log::info!(
                "{} 0x{:x} - 0x{:x} ({})",
                name,
                mem.get_base(),
                mem.get_bottom(),
                mem.size()
            );
        }
    }

    pub fn free(&mut self, name: &str) {
        let id = self
            .name_map
            .get(name)
            .expect(format!("map name {} not found", name).as_str());
        let mem = self.mem_slab.get_mut(*id).unwrap();
        mem.clear();
        self.maps.remove(&mem.get_base());
        self.mem_slab.remove(*id);
        self.tlb.borrow_mut().flush();
        self.name_map.remove(name);
    }

    pub fn dealloc(&mut self, addr: u64) {
        let mem_key = match self.maps.get(&addr) {
            Some(key) => key,
            None => {
                log::info!("dealloc: non mapped address 0x{:x}", addr);
                return;
            }
        };
        let mem = self.mem_slab.get_mut(*mem_key).unwrap();
        self.name_map.remove(mem.get_name());
        mem.clear();
        self.mem_slab.remove(*mem_key);
        self.tlb.borrow_mut().flush();
        self.maps.remove(&addr);
    }

    pub fn map(&mut self, name: &str, sz: u64, permission: Permission) -> u64 {
        let addr = self.alloc(sz).expect("emu.maps.map(sz) cannot allocate");
        self.create_map(name, addr, sz, permission)
            .expect("emu.maps.map(sz) cannot create map");
        addr
    }

    pub fn map_lib(&mut self, name: &str, sz: u64, permission: Permission) -> u64 {
        let addr = self.alloc(sz).expect("emu.maps.map(sz) cannot allocate");
        if self.is_64bits {
            let addr = self
                .lib64_alloc(sz)
                .expect("emu.maps.map_lib(sz) cannot allocate");
        } else {
            let addr = self
                .lib32_alloc(sz)
                .expect("emu.maps.map_lib(sz) cannot allocate");
        }
        self.create_map(name, addr, sz, permission)
            .expect("emu.maps.map_lib(sz) cannot create map");
        addr
    }

    pub fn lib64_alloc(&self, sz: u64) -> Option<u64> {
        self._alloc(sz, constants::LIBS64_MIN, constants::LIBS64_MAX, true)
    }

    pub fn lib32_alloc(&self, sz: u64) -> Option<u64> {
        self._alloc(sz, constants::LIBS32_MIN, constants::LIBS32_MAX, true)
    }

    // this alloc return an address but you have to map it
    pub fn alloc(&self, sz: u64) -> Option<u64> {
        if self.is_64bits {
            self._alloc(sz, constants::ALLOC64_MIN, constants::ALLOC64_MAX, false)
        } else {
            self._alloc(sz, constants::ALLOC32_MIN, constants::ALLOC32_MAX, false)
        }
    }

    fn _alloc(&self, mut sz: u64, bottom: u64, top: u64, lib: bool) -> Option<u64> {
        /*
         *  params:
         *    sz: size to allocate, this number will be aligned.
         *    bottom: minimum address to allocate
         *    top: max address
         *    lib: allocating a library?
         *  vars:
         *    prev: is an aligned address, start with bottom and iterates every map bottom.
         *    base: base address of specific map.
         */

        let mut prev: u64 = self.align_up(bottom, Self::DEFAULT_ALIGNMENT);
        let debug = false;

        if sz > 0xffffff {
            sz = 0xffffff;
        }

        // Round up size to alignment
        sz = self.align_up(sz, Self::DEFAULT_ALIGNMENT);

        if debug {
            log::info!("allocating {} bytes from 0x{:x} to 0x{:x}", sz, bottom, top);
        }

        // Here we assume that we go from the bottom to the most
        for (_, mem_key) in self.maps.iter() {
            let mem = self.mem_slab.get(*mem_key).unwrap();
            let base = mem.get_base();

            if lib && base < bottom {
                if debug {
                    log::info!("skipping: 0x{:x}", base);
                }
                continue;
            }

            if debug {
                log::info!("base: 0x{:x} prev: 0x{:x} sz: 0x{:x}", base, prev, sz);
            }
            if prev > base {
                // we shouldn't care about this we just skip this memory region
                continue;
                // panic!("alloc error prev:0x{:x} > base:0x{:x}", prev, base);
            }
            if debug {
                log::info!("space: 0x{:x}", base - prev);
            }
            if (base - prev) > sz {
                if debug {
                    log::info!("space found: 0x{:x}", prev);
                }
                return Some(prev);
            }

            prev = self.align_up(mem.get_bottom(), Self::DEFAULT_ALIGNMENT);
        }

        if top < prev {
            prev = self.align_up(top, Self::DEFAULT_ALIGNMENT);
        }
        if top - prev > sz {
            if debug {
                log::info!("space found: 0x{:x} sz:{}", prev, sz);
            }
            return Some(prev);
        }

        log::info!("no space found");
        None
    }

    fn align_up(&self, addr: u64, align: u64) -> u64 {
        (addr + (align - 1)) & !(align - 1)
    }

    fn align_down(&self, addr: u64, align: u64) -> u64 {
        addr & !(align - 1)
    }

    pub fn save_all_allocs(&mut self, path: String) {
        for (_, mem) in self.mem_slab.iter() {
            if mem.get_name().to_string().starts_with("alloc_") {
                let mut ppath = path.clone();
                ppath.push('/');
                ppath.push_str(&mem.get_name());
                ppath.push_str(".bin");
                mem.save(mem.get_base(), mem.size(), ppath);
            }
        }
    }

    pub fn save_all(&self, path: String) {
        for (_, mem) in self.mem_slab.iter() {
            let mut ppath = path.clone();
            ppath.push('/');
            ppath.push_str(&format!("{:08x}-{}", mem.get_base(), mem.get_name()));
            ppath.push_str(".bin");
            mem.save(mem.get_base(), mem.size(), ppath);
        }
    }

    pub fn save(&mut self, addr: u64, size: u64, filename: String) {
        //TODO: return a boolean or option.
        match self.get_mem_by_addr_mut(addr) {
            Some(m) => {
                m.save(addr, size as usize, filename);
            }
            None => {
                log::info!("this address is not mapped.");
            }
        }
    }

    pub fn filter_string(&self, s: &mut String) {
        let valid = " 0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ \t\x00".as_bytes();
        s.retain(|c| valid.contains(&(c as u8)));
    }

    pub fn filter_replace_bytes(&self, s: &[u8]) -> Vec<u8> {
        let mut sanitized: Vec<u8> = Vec::new();
        let valid = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".as_bytes();
        let mut p;

        for si in s.iter() {
            p = false;
            for validj in valid.iter() {
                if validj == si {
                    sanitized.push(*si);
                    p = true;
                    break;
                }
            }
            if !p {
                sanitized.push(b'.');
            }
        }

        sanitized
    }

    pub fn filter_replace_string(&self, s: &str) -> String {
        let valid = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".as_bytes();
        let sb = s.as_bytes();
        let mut p;
        let mut dst: Vec<char> = Vec::new();

        for i in 0..s.len() {
            p = false;
            for j in 0..valid.len() {
                if sb[i] == valid[j] {
                    dst.push(sb[i] as char);
                    p = true;
                    break;
                }
            }
            if !p {
                dst.push('.');
            }
        }

        let sdst: String = dst.into_iter().collect();
        sdst
    }

    pub fn mem_test(&self) -> bool {
        for (_, mem1) in self.mem_slab.iter() {
            let name1 = mem1.get_name();

            for (_, mem2) in self.mem_slab.iter() {
                let name2 = mem2.get_name();

                if name1 != name2 {
                    for addr1 in mem1.get_base()..mem1.get_bottom() {
                        if mem2.inside(addr1) {
                            log::info!("/!\\ {} overlaps with {}", name1, name2);
                            log::info!(
                                "/!\\ 0x{:x}-0x{:x} vs 0x{:x}-0x{:x}",
                                mem1.get_base(),
                                mem1.get_bottom(),
                                mem2.get_base(),
                                mem2.get_bottom()
                            );
                            return false;
                        }
                    }
                }
            }

            if (mem1.get_base() + (mem1.size() as u64)) != mem1.get_bottom() {
                log::info!("/!\\ memory bottom dont match, mem: {}", name1);
                return false;
            }
        }

        true
    }
}
