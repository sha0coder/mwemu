/*
    Little endian 64 bits and inferior bits memory.
*/
use md5;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Read;
use std::io::SeekFrom;
use std::io::Write;

#[derive(Clone, Serialize, Deserialize)]
pub struct Mem64 {
    mem_name: String,
    base_addr: u64,
    bottom_addr: u64,
    pub mem: Vec<u8>,
}

impl Default for Mem64 {
    fn default() -> Self {
        Self::new()
    }
}

impl Mem64 {
    pub fn new() -> Mem64 {
        Mem64 {
            mem_name: "".to_string(),
            base_addr: 0,
            bottom_addr: 0,
            mem: Vec::new(),
        }
    }

    #[inline(always)]
    pub fn get_name(&self) -> String {
        self.mem_name.clone()
    }

    #[inline(always)]
    pub fn set_name(&mut self, name: &str) {
        self.mem_name = name.to_string();
    }

    #[inline(always)]
    pub fn get_mem(&self) -> Vec<u8> {
        self.mem.clone()
    }

    #[inline(always)]
    pub fn alloc(&mut self, amount: usize) {
        self.mem = vec![0; amount];
    }

    pub fn extend(&mut self, amount: usize) {
        for i in 0..amount {
            self.mem.push(0);
        }
        self.bottom_addr += amount as u64;
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        self.mem.len()
    }

    #[inline(always)]
    pub fn get_base(&self) -> u64 {
        self.base_addr
    }

    #[inline(always)]
    pub fn get_bottom(&self) -> u64 {
        self.bottom_addr
    }

    pub fn memcpy(&mut self, ptr: &[u8], sz: usize) {
        if self.mem.len() < sz {
            log::error!(
                "Try memcpy at mem but size bigger than allocated size: addr {}, size {}",
                self.base_addr,
                sz
            );
            panic!("memcpy: {} < {}", self.mem.len(), sz);
        }
        self.mem[..sz].copy_from_slice(&ptr[..sz]);
    }

    #[inline]
    pub fn inside(&self, addr: u64) -> bool {
        if addr >= self.base_addr && addr < self.bottom_addr {
            true
        } else {
            false
        }
    }

    #[inline(always)]
    pub fn set_base(&mut self, base_addr: u64) {
        self.base_addr = base_addr;
        self.bottom_addr = base_addr;
    }

    #[inline(always)]
    pub fn update_base(&mut self, base_addr: u64) {
        self.base_addr = base_addr;
    }

    pub fn set_bottom(&mut self, bottom_addr: u64) {
        self.bottom_addr = bottom_addr;
        let size = self.bottom_addr - self.base_addr;
        self.alloc(size as usize);
    }

    #[inline(always)]
    pub fn update_bottom(&mut self, bottom_addr: u64) {
        self.bottom_addr = bottom_addr;
    }

    pub fn set_size(&mut self, size: u64) {
        self.bottom_addr = self.base_addr + size;
        self.alloc(size as usize);
    }

    #[inline(always)]
    pub fn build_addresses(&self, addr: u64, sz: usize) -> Vec<u64> {
        vec![addr; addr as usize + sz]
    }

    #[inline(always)]
    pub fn read_from(&self, addr: u64) -> &[u8] {
        let idx = (addr - self.base_addr) as usize;
        let max_sz = (self.bottom_addr - self.base_addr) as usize;
        /*
        let mut sz = idx + 5;
        if sz > max_sz {
            sz = max_sz;
        }*/
        let r = self.mem.get(idx..max_sz).unwrap();
        if cfg!(feature = "log_mem") {
            log::trace!(
                "mem: read_from: 0x{:x?} = {:?}",
                self.build_addresses(addr, max_sz),
                r
            );
        }
        r
    }

    #[inline(always)]
    pub fn read_bytes(&self, addr: u64, sz: usize) -> &[u8] {
        if addr >= self.base_addr + self.mem.len() as u64 {
            return &[0; 0];
        }
        if addr < self.base_addr {
            return &[0; 0];
        }
        let idx = (addr - self.base_addr) as usize;
        let sz2 = idx + sz;
        if sz2 > self.mem.len() {
            return self.mem.get(idx..self.mem.len()).unwrap();
        }
        let r = self.mem.get(idx..sz2).unwrap();
        if cfg!(feature = "log_mem") {
            log::trace!(
                "mem: read_bytes: 0x{:x?} = {:?}",
                self.build_addresses(addr, sz),
                r
            );
        }
        r
    }



    #[inline(always)]
    pub fn read_byte(&self, addr: u64) -> u8 {
        let idx = (addr - self.base_addr) as usize;
        let r = self.mem[idx];

        if cfg!(feature = "log_mem") {
            log::trace!(
                "mem: read_byte: 0x{:x?} = 0x{:x}",
                self.build_addresses(addr, 1),
                r
            );
        }
        r
    }

    #[inline(always)]
    pub fn read_word(&self, addr: u64) -> u16 {
        let idx = (addr - self.base_addr) as usize;
        let r = (self.mem[idx] as u16) + ((self.mem[idx + 1] as u16) << 8);
        let r = u16::from_le_bytes(self.mem[idx..idx + 2].try_into().expect("incorrect length"));
        if cfg!(feature = "log_mem") {
            log::trace!(
                "mem: read_word: 0x{:x?} = 0x{:x}",
                self.build_addresses(addr, 2),
                r
            );
        }
        r
    }

    #[inline(always)]
    pub fn read_dword(&self, addr: u64) -> u32 {
        let idx = (addr - self.base_addr) as usize;
        let r = u32::from_le_bytes(self.mem[idx..idx + 4].try_into().expect("incorrect length"));
        if cfg!(feature = "log_mem") {
            log::trace!(
                "mem: read_dword: 0x{:x?} = 0x{:x}",
                self.build_addresses(addr, 4),
                r
            );
        }

        r
    }

    #[inline(always)]
    pub fn read_qword(&self, addr: u64) -> u64 {
        let idx = (addr - self.base_addr) as usize;
        let r = u64::from_le_bytes(self.mem[idx..idx + 8].try_into().expect("incorrect length"));

        if cfg!(feature = "log_mem") {
            log::trace!(
                "mem: read_qword: 0x{:x?} = 0x{:x}",
                self.build_addresses(addr, 8),
                r
            );
        }

        r
    }

    pub fn read_oword(&self, addr: u64) -> u128 {
        let idx = (addr - self.base_addr) as usize;
        let r = u128::from_le_bytes(
            self.mem[idx..idx + 16]
                .try_into()
                .expect("incorrect length"),
        );

        if cfg!(feature = "log_mem") {
            log::trace!(
                "mem: read_qword: 0x{:x?} = 0x{:x}",
                self.build_addresses(addr, 8),
                r
            );
        }

        r
    }

    #[inline(always)]
    pub fn write_byte(&mut self, addr: u64, value: u8) {
        let idx = (addr - self.base_addr) as usize;
        self.mem[idx] = value;

        if cfg!(feature = "log_mem") {
            log::trace!(
                "mem: write_byte: 0x{:x?} = 0x{:x}",
                self.build_addresses(addr, 1),
                value
            );
        }
    }

    #[inline(always)]
    pub fn write_bytes(&mut self, addr: u64, bs: &[u8]) {
        let idx = (addr - self.base_addr) as usize;
        self.mem[idx..(bs.len() + idx)].copy_from_slice(bs.as_ref());
        if cfg!(feature = "log_mem") {
            log::trace!(
                "mem: write_bytes: 0x{:x?} = {:?}",
                self.build_addresses(addr, bs.len()),
                bs
            );
        }
    }

    #[inline(always)]
    pub fn write_word(&mut self, addr: u64, value: u16) {
        let idx = (addr - self.base_addr) as usize;
        self.mem[idx..idx + 2].copy_from_slice(value.to_le_bytes().to_vec().as_ref());

        if cfg!(feature = "log_mem") {
            log::trace!(
                "mem: write_word: 0x{:x?} = 0x{:x}",
                self.build_addresses(addr, 2),
                value
            );
        }
    }

    #[inline(always)]
    pub fn write_dword(&mut self, addr: u64, value: u32) {
        let idx = (addr - self.base_addr) as usize;
        self.mem[idx..idx + 4].copy_from_slice(value.to_le_bytes().to_vec().as_ref());

        if cfg!(feature = "log_mem") {
            log::trace!(
                "mem: write_dword: 0x{:x?} = 0x{:x}",
                self.build_addresses(addr, 4),
                value
            );
        }
    }

    #[inline(always)]
    pub fn write_qword(&mut self, addr: u64, value: u64) {
        let idx = (addr - self.base_addr) as usize;
        self.mem[idx..idx + 8].copy_from_slice(value.to_le_bytes().to_vec().as_ref());

        if cfg!(feature = "log_mem") {
            log::trace!(
                "mem: write_qword: 0x{:x?} = 0x{:x}",
                self.build_addresses(addr, 8),
                value
            );
        }
    }

    #[inline(always)]
    pub fn write_string(&mut self, addr: u64, s: &str) {
        let mut v = s.as_bytes().to_vec();
        v.push(0);
        self.write_bytes(addr, &v);

        if cfg!(feature = "log_mem") {
            log::trace!(
                "mem: write_string: 0x{:x?} = {:?}",
                self.build_addresses(addr, s.len() + 1),
                s
            );
        }
    }

    #[inline(always)]
    pub fn write_wide_string(&mut self, addr: u64, s: &str) {
        let wide_string: Vec<u16> = s.encode_utf16().collect();
        /* TODO: maybe this is unsafe. It needed to assert that the the input is not empty
            and divided by 2.
        */
        let byte_slice: &[u8] = unsafe {
            std::slice::from_raw_parts(
                wide_string.as_ptr() as *const u8,
                wide_string.len() * std::mem::size_of::<u16>(),
            )
        };
        self.write_bytes(addr, &byte_slice);

        if cfg!(feature = "log_mem") {
            log::trace!(
                "mem: write_wide_string: 0x{:x?} = {:?}",
                self.build_addresses(addr, s.len() * 2 + 2),
                s
            );
        }
    }

    #[inline]
    pub fn read_wide_string(&self, addr: u64) -> String {
        let MAX_SIZE_STR = 1_000_000;
        let mut s: Vec<u16> = Vec::new();
        let mut idx = addr;
        while idx < MAX_SIZE_STR {
            let b = self.read_word(idx);
            if b == 0 {
                break;
            }
            s.push(b);
            idx += 2;
        }
        String::from_utf16(&s).expect("invalid utf-16")
    }
    pub fn print_bytes(&self) {
        log::info!("---mem---");
        for b in self.mem.iter() {
            print!("{}", b);
        }
        log::info!("---");
    }

    pub fn print_dwords(&self) {
        self.print_dwords_from_to(self.get_base(), self.get_bottom());
    }

    pub fn print_dwords_from_to(&self, from: u64, to: u64) {
        log::info!("---mem---");
        for addr in (from..to).step_by(4) {
            log::info!("0x{:x}", self.read_dword(addr))
        }

        log::info!("---");
    }

    pub fn md5(&self) -> md5::Digest {
        md5::compute(&self.mem)
    }

    pub fn load_at(&mut self, base_addr: u64) {
        self.set_base(base_addr);
        let mut name: String = String::from(&self.mem_name);
        name.push_str(".bin");
        self.load(name.as_str());
    }

    pub fn load_chunk(&mut self, filename: &str, off: u64, sz: usize) -> bool {
        // log::info!("loading chunk: {} {} {}", filename, off, sz);
        let mut f = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                return false;
            }
        };
        f.seek(SeekFrom::Start(off));
        let mut reader = BufReader::new(&f);
        self.mem.clear();
        for i in 0..sz {
            self.mem.push(0);
        }
        reader
            .read_exact(&mut self.mem)
            .expect("cannot load chunk of file");
        f.sync_all(); // thanks, Alberto Segura
        true
    }

    pub fn load(&mut self, filename: &str) -> bool {
        // log::info!("loading map: {}", filename);
        let f = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                return false;
            }
        };
        let len = f.metadata().unwrap().len();
        self.bottom_addr = self.base_addr + len;
        let mut reader = BufReader::new(&f);
        reader
            .read_to_end(&mut self.mem)
            .expect("cannot load map file");
        f.sync_all(); // thanks, Alberto Segura
        true
    }

    pub fn save(&self, addr: u64, size: usize, filename: String) {
        let idx = (addr - self.base_addr) as usize;
        let sz2 = idx + size;
        if sz2 > self.mem.len() {
            log::error!("size too big, map size is {}  sz2:{}", self.mem.len(), sz2);
            return;
        }

        let mut f = match File::create(&filename) {
            Ok(f) => f,
            Err(e) => {
                log::error!("cannot create the file {}", e);
                return;
            }
        };

        let blob = self.mem.get(idx..sz2).unwrap();

        match f.write_all(blob) {
            Ok(_) => log::info!(
                "saved. addr: 0x{:x} size: {} filename: {}",
                addr,
                size,
                filename
            ),
            Err(_) => log::error!("couldn't save the file"),
        }

        f.sync_all().unwrap();
    }
}
