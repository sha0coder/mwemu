/*
    Little endian 64 bits and inferior bits memory.
*/
use crate::emu_context;
use bytemuck::cast_slice;
use md5;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Read;
use std::io::SeekFrom;
use std::io::Write;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Permission(u8);

// Bit flags for permissions
impl Permission {
    pub const NONE: Permission = Permission(0b000);
    pub const READ: Permission = Permission(0b001);
    pub const WRITE: Permission = Permission(0b010);
    pub const EXECUTE: Permission = Permission(0b100);

    // Common combinations
    pub const READ_WRITE: Permission = Permission(0b011);
    pub const READ_EXECUTE: Permission = Permission(0b101);
    pub const WRITE_EXECUTE: Permission = Permission(0b110);
    pub const READ_WRITE_EXECUTE: Permission = Permission(0b111);

    /// Create a new Permission from raw bits
    #[inline]
    pub const fn from_bits(bits: u8) -> Self {
        Permission(bits & 0b111) // Only use the lower 3 bits
    }

    /// Get the raw bits
    #[inline]
    pub const fn bits(&self) -> u8 {
        self.0
    }

    /// Create permission from individual flags
    #[inline]
    pub fn from_flags(read: bool, write: bool, execute: bool) -> Self {
        let mut bits = 0;
        if read {
            bits |= 0b001;
        }
        if write {
            bits |= 0b010;
        }
        if execute {
            bits |= 0b100;
        }
        Permission(bits)
    }

    /// Check if read access is allowed
    #[inline]
    pub const fn can_read(&self) -> bool {
        (self.0 & 0b001) != 0
    }

    /// Check if write access is allowed
    #[inline]
    pub const fn can_write(&self) -> bool {
        (self.0 & 0b010) != 0
    }

    /// Check if execute access is allowed
    #[inline]
    pub const fn can_execute(&self) -> bool {
        (self.0 & 0b100) != 0
    }

    /// Check if any access is allowed
    #[inline]
    pub const fn is_accessible(&self) -> bool {
        self.0 != 0
    }

    /// Add a permission (union)
    #[inline]
    pub const fn add(&self, other: Permission) -> Self {
        Permission(self.0 | other.0)
    }

    /// Remove a permission
    pub const fn remove(&self, other: Permission) -> Self {
        Permission(self.0 & !other.0)
    }

    /// Check if this permission contains another permission
    #[inline]
    pub const fn contains(&self, other: Permission) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Get individual permission flags
    #[inline]
    pub const fn to_flags(&self) -> (bool, bool, bool) {
        (self.can_read(), self.can_write(), self.can_execute())
    }
}

// Implement bitwise operations for Permission
impl std::ops::BitOr for Permission {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Permission(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for Permission {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Permission(self.0 & rhs.0)
    }
}

impl std::ops::BitXor for Permission {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        Permission(self.0 ^ rhs.0)
    }
}

impl std::ops::Not for Permission {
    type Output = Self;

    fn not(self) -> Self {
        Permission(!self.0 & 0b111)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Mem64 {
    mem_name: String,
    base_addr: u64,
    bottom_addr: u64,
    permission: Permission,
    mem: Vec<u8>,
}

impl Default for Mem64 {
    fn default() -> Self {
        Mem64 {
            mem_name: "".to_string(),
            base_addr: 0,
            bottom_addr: 0,
            permission: Permission::from_flags(true, true, false),
            mem: Vec::new(),
        }
    }
}

impl Mem64 {
    pub fn new(
        mem_name: String,
        base_addr: u64,
        bottom_addr: u64,
        mem: Vec<u8>,
        permission: Permission,
    ) -> Mem64 {
        Mem64 {
            mem_name,
            base_addr,
            bottom_addr,
            permission,
            mem,
        }
    }

    pub fn permission(&self) -> Permission {
        self.permission
    }

    /// Set new permissions
    pub fn set_permission(&mut self, permission: Permission) {
        self.permission = permission;
    }

    /// Add permissions
    pub fn add_permission(&mut self, permission: Permission) {
        self.permission = self.permission.add(permission);
    }

    /// Remove permissions
    pub fn remove_permission(&mut self, permission: Permission) {
        self.permission = self.permission.remove(permission);
    }

    /// Check if read access is allowed for this memory region
    pub fn can_read(&self) -> bool {
        self.permission.can_read()
    }

    /// Check if write access is allowed for this memory region
    pub fn can_write(&self) -> bool {
        self.permission.can_write()
    }

    /// Check if execute access is allowed for this memory region
    pub fn can_execute(&self) -> bool {
        self.permission.can_execute()
    }

    pub fn clear(&mut self) {
        self.mem_name = "".to_string();
        self.base_addr = 0;
        self.bottom_addr = 0;
        self.mem.clear();
    }

    #[inline(always)]
    pub fn get_name(&self) -> &str {
        self.mem_name.as_str()
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
        // TODO: log trace read write?
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
        (addr..addr + sz as u64).collect()
    }

    #[inline(always)]
    pub fn read_from(&self, addr: u64) -> &[u8] {
        if !self.can_read() {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "FAILED doesn't have permission: read_from: 0x{:x?}",
                        addr
                    );
                }
            })
            .unwrap();
            panic!("FAILED to read without permission: addr: 0x{:x?}", addr);
        }

        let idx = (addr - self.base_addr) as usize;
        let max_sz = (self.bottom_addr - self.base_addr) as usize;
        /*
        let mut sz = idx + 5;
        if sz > max_sz {
            sz = max_sz;
        }*/
        let r = self.mem.get(idx..max_sz).unwrap();
        if cfg!(feature = "log_mem_read") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem_trace: read_from: 0x{:x?} = {:x?}",
                        self.build_addresses(addr, max_sz),
                        r
                    );
                }
            })
            .unwrap();
        }
        r
    }

    #[inline(always)]
    pub fn read_bytes(&self, addr: u64, sz: usize) -> &[u8] {
        if !self.can_read() {
            panic!("FAILED to read without permission: addr: 0x{:x?}", addr);
        }

        if addr >= self.base_addr + self.mem.len() as u64 {
            // TODO: log trace?
            return &[0; 0];
        }
        if addr < self.base_addr {
            // TODO: log trace?
            return &[0; 0];
        }
        let idx = (addr - self.base_addr) as usize;
        let sz2 = idx + sz;
        if sz2 > self.mem.len() {
            // TODO: log trace?
            let addr = self.mem.get(idx..self.mem.len()).unwrap();
            return addr;
        }
        let r = self.mem.get(idx..sz2).unwrap();
        if cfg!(feature = "log_mem_read") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem_trace: read_bytes: 0x{:x?} = {:x?}",
                        self.build_addresses(addr, sz),
                        r
                    );
                }
            })
            .unwrap();
        }
        r
    }

    #[inline(always)]
    pub fn get_bytes(&self) -> &[u8] {
        let idx = 0;
        let sz2 = self.size();
        if sz2 > self.mem.len() {
            // TODO: log trace?
            let bytes = self.mem.get(idx..self.mem.len()).unwrap();
            return bytes;
        }
        let r = self.mem.get(idx..sz2).unwrap();
        if cfg!(feature = "log_mem_read") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem_trace: read_bytes: 0x{:x?} = {:x?}",
                        self.build_addresses(self.get_base(), self.size()),
                        r
                    );
                }
            })
            .unwrap();
        }
        r
    }

    #[inline(always)]
    pub fn read_byte(&self, addr: u64) -> u8 {
        if !self.can_read() {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "FAILED doesn't have permission: read_from: 0x{:x?}",
                        addr
                    );
                }
            })
            .unwrap();
            panic!("FAILED to read without permission: addr: 0x{:x?}", addr);
        }

        let idx = (addr - self.base_addr) as usize;
        let r = self.mem[idx];
        if cfg!(feature = "log_mem_read") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem_trace: read_byte: 0x{:x?} = 0x{:x}",
                        self.build_addresses(addr, 1),
                        r
                    );
                }
            })
            .unwrap();
        }
        r
    }

    #[inline(always)]
    pub fn read_word(&self, addr: u64) -> u16 {
        if !self.can_read() {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "FAILED doesn't have permission: read_from: 0x{:x?}",
                        addr
                    );
                }
            })
            .unwrap();
            panic!("FAILED to read without permission: addr: 0x{:x?}", addr);
        }

        let idx = (addr - self.base_addr) as usize;
        let r = (self.mem[idx] as u16) + ((self.mem[idx + 1] as u16) << 8);
        let r = u16::from_le_bytes(self.mem[idx..idx + 2].try_into().expect("incorrect length"));
        if cfg!(feature = "log_mem_read") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem_trace: read_word: 0x{:x?} = 0x{:x}",
                        self.build_addresses(addr, 2),
                        r
                    );
                }
            })
            .unwrap();
        }
        r
    }

    #[inline(always)]
    pub fn read_dword(&self, addr: u64) -> u32 {
        if !self.can_read() {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "FAILED doesn't have permission: read_from: 0x{:x?}",
                        addr
                    );
                }
            })
            .unwrap();
            panic!("FAILED to read without permission: addr: 0x{:x?}", addr);
        }

        let idx = (addr - self.base_addr) as usize;
        let r = u32::from_le_bytes(self.mem[idx..idx + 4].try_into().expect("incorrect length"));
        if cfg!(feature = "log_mem_read") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem_trace: read_dword: 0x{:x?} = 0x{:x}",
                        self.build_addresses(addr, 4),
                        r
                    );
                }
            })
            .unwrap();
        }
        r
    }

    #[inline(always)]
    pub fn read_qword(&self, addr: u64) -> u64 {
        if !self.can_read() {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "FAILED doesn't have permission: read_from: 0x{:x?}",
                        addr
                    );
                }
            })
            .unwrap();
            panic!("FAILED to read without permission: addr: 0x{:x?}", addr);
        }

        let idx = (addr - self.base_addr) as usize;
        let r = u64::from_le_bytes(self.mem[idx..idx + 8].try_into().expect("incorrect length"));
        if cfg!(feature = "log_mem_read") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem_trace: read_qword: 0x{:x?} = 0x{:x}",
                        self.build_addresses(addr, 8),
                        r
                    );
                }
            })
            .unwrap();
        }
        r
    }

    pub fn read_oword(&self, addr: u64) -> u128 {
        if !self.can_read() {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "FAILED doesn't have permission: read_from: 0x{:x?}",
                        addr
                    );
                }
            })
            .unwrap();
            panic!("FAILED to read without permission: addr: 0x{:x?}", addr);
        }

        let idx = (addr - self.base_addr) as usize;
        let r = u128::from_le_bytes(
            self.mem[idx..idx + 16]
                .try_into()
                .expect("incorrect length"),
        );
        if cfg!(feature = "log_mem_read") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem_trace: read_qword: 0x{:x?} = 0x{:x}",
                        self.build_addresses(addr, 8),
                        r
                    );
                }
            })
            .unwrap();
        }
        r
    }

    #[inline(always)]
    pub fn force_write_byte(&mut self, addr: u64, value: u8) {
        let idx = (addr - self.base_addr) as usize;
        self.mem[idx] = value;
        if cfg!(feature = "log_mem_write") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem: force_write_byte: 0x{:x?} = 0x{:x}",
                        self.build_addresses(addr, 1),
                        value
                    );
                }
            })
            .unwrap();
        }
    }

    #[inline(always)]
    pub fn force_write_bytes(&mut self, addr: u64, bs: &[u8]) {
        let idx = (addr - self.base_addr) as usize;
        self.mem[idx..(bs.len() + idx)].copy_from_slice(bs.as_ref());
        if cfg!(feature = "log_mem_write") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem: force_write_bytes: 0x{:x?} = {:?}",
                        self.build_addresses(addr, bs.len()),
                        bs
                    );
                }
            })
            .unwrap();
        }
    }

    #[inline(always)]
    pub fn force_write_word(&mut self, addr: u64, value: u16) {
        let idx = (addr - self.base_addr) as usize;
        self.mem[idx..idx + 2].copy_from_slice(value.to_le_bytes().to_vec().as_ref());

        if cfg!(feature = "log_mem_write") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem: force_write_word: 0x{:x?} = 0x{:x}",
                        self.build_addresses(addr, 2),
                        value
                    );
                }
            })
            .unwrap();
        }
    }

    #[inline(always)]
    pub fn force_write_dword(&mut self, addr: u64, value: u32) {
        let idx = (addr - self.base_addr) as usize;
        self.mem[idx..idx + 4].copy_from_slice(value.to_le_bytes().to_vec().as_ref());

        if cfg!(feature = "log_mem_write") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem: force_write_dword: 0x{:x?} = 0x{:x}",
                        self.build_addresses(addr, 4),
                        value
                    );
                }
            })
            .unwrap();
        }
    }

    #[inline(always)]
    pub fn force_write_qword(&mut self, addr: u64, value: u64) {
        let idx = (addr - self.base_addr) as usize;
        self.mem[idx..idx + 8].copy_from_slice(value.to_le_bytes().to_vec().as_ref());

        if cfg!(feature = "log_mem_write") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem: force_write_qword: 0x{:x?} = 0x{:x}",
                        self.build_addresses(addr, 8),
                        value
                    );
                }
            })
            .unwrap();
        }
    }

    #[inline(always)]
    pub fn force_write_oword(&mut self, addr: u64, value: u128) {
        let idx = (addr - self.base_addr) as usize;
        self.mem[idx..idx + 16].copy_from_slice(value.to_le_bytes().to_vec().as_ref());

        if cfg!(feature = "log_mem_write") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem: force_write_oword: 0x{:x?} = 0x{:x}",
                        self.build_addresses(addr, 16),
                        value
                    );
                }
            })
            .unwrap();
        }
    }

    #[inline(always)]
    pub fn force_write_string(&mut self, addr: u64, s: &str) {
        let mut v = s.as_bytes().to_vec();
        v.push(0);
        self.force_write_bytes(addr, &v);

        if cfg!(feature = "log_mem_write") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem: force_write_string: 0x{:x?} = {:?}",
                        self.build_addresses(addr, s.len() + 1),
                        s
                    );
                }
            })
            .unwrap();
        }
    }

    #[inline(always)]
    pub fn write_byte(&mut self, addr: u64, value: u8) {
        if !self.can_write() {
            panic!("FAILED to write without permission: addr: 0x{:x?}", addr);
        }

        let idx = (addr - self.base_addr) as usize;
        self.mem[idx] = value;
        if cfg!(feature = "log_mem_write") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem_trace: write_byte: 0x{:x?} = 0x{:x}",
                        self.build_addresses(addr, 1),
                        value
                    );
                }
            })
            .unwrap();
        }
    }

    #[inline(always)]
    pub fn write_bytes(&mut self, addr: u64, bs: &[u8]) {
        if !self.can_write() {
            panic!("FAILED to write without permission: addr: 0x{:x?}", addr);
        }

        let idx = (addr - self.base_addr) as usize;
        self.mem[idx..(bs.len() + idx)].copy_from_slice(bs.as_ref());
        if cfg!(feature = "log_mem_write") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem_trace: write_bytes: 0x{:x?} = {:x?}",
                        self.build_addresses(addr, bs.len()),
                        bs
                    );
                }
            })
            .unwrap();
        }
    }

    #[inline(always)]
    pub fn write_word(&mut self, addr: u64, value: u16) {
        if !self.can_write() {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "FAILED doesn't have permission: write_from: 0x{:x?}",
                        addr
                    );
                }
            })
            .unwrap();
            panic!("FAILED to write without permission: addr: 0x{:x?}", addr);
        }

        let idx = (addr - self.base_addr) as usize;
        self.mem[idx..idx + 2].copy_from_slice(value.to_le_bytes().to_vec().as_ref());

        if cfg!(feature = "log_mem_write") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem_trace: write_word: 0x{:x?} = 0x{:x}",
                        self.build_addresses(addr, 2),
                        value
                    );
                }
            })
            .unwrap();
        }
    }

    #[inline(always)]
    pub fn write_dword(&mut self, addr: u64, value: u32) {
        if !self.can_write() {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "FAILED doesn't have permission: write_from: 0x{:x?}",
                        addr
                    );
                }
            })
            .unwrap();
            panic!("FAILED to write without permission: addr: 0x{:x?}", addr);
        }

        let idx = (addr - self.base_addr) as usize;
        self.mem[idx..idx + 4].copy_from_slice(value.to_le_bytes().to_vec().as_ref());

        if cfg!(feature = "log_mem_write") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem_trace: write_dword: 0x{:x?} = 0x{:x}",
                        self.build_addresses(addr, 4),
                        value
                    );
                }
            })
            .unwrap();
        }
    }

    #[inline(always)]
    pub fn write_qword(&mut self, addr: u64, value: u64) {
        if !self.can_write() {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "FAILED doesn't have permission: write_from: 0x{:x?}",
                        addr
                    );
                }
            })
            .unwrap();
            panic!("FAILED to write without permission: addr: 0x{:x?}", addr);
        }

        let idx = (addr - self.base_addr) as usize;
        self.mem[idx..idx + 8].copy_from_slice(value.to_le_bytes().to_vec().as_ref());

        if cfg!(feature = "log_mem_write") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem_trace: write_qword: 0x{:x?} = 0x{:x}",
                        self.build_addresses(addr, 8),
                        value
                    );
                }
            })
            .unwrap();
        }
    }

    #[inline(always)]
    pub fn write_oword(&mut self, addr: u64, value: u128) {
        if !self.can_write() {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "FAILED doesn't have permission: write_from: 0x{:x?}",
                        addr
                    );
                }
            })
            .unwrap();
            panic!("FAILED to write without permission: addr: 0x{:x?}", addr);
        }

        let idx = (addr - self.base_addr) as usize;
        self.mem[idx..idx + 16].copy_from_slice(value.to_le_bytes().to_vec().as_ref());

        if cfg!(feature = "log_mem_write") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem_trace: write_qword: 0x{:x?} = 0x{:x}",
                        self.build_addresses(addr, 8),
                        value
                    );
                }
            })
            .unwrap();
        }
    }

    #[inline(always)]
    pub fn write_string(&mut self, addr: u64, s: &str) {
        if !self.can_write() {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "FAILED doesn't have permission: write_from: 0x{:x?}",
                        addr
                    );
                }
            })
            .unwrap();
            panic!("FAILED to write without permission: addr: 0x{:x?}", addr);
        }

        let mut v = s.as_bytes().to_vec();
        if v.last() != Some(&0) {
            v.push(0);
        }
        self.write_bytes(addr, &v);

        if cfg!(feature = "log_mem_write") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem_trace: write_string: 0x{:x?} = {} ({:x?})",
                        self.build_addresses(addr, v.len()),
                        s,
                        v
                    );
                }
            })
            .unwrap();
        }
    }

    #[inline(always)]
    pub fn read_string(&self, addr: u64) -> String {
        if !self.can_read() {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "FAILED doesn't have permission: read_from: 0x{:x?}",
                        addr
                    );
                }
            })
            .unwrap();
            panic!("FAILED to read without permission: addr: 0x{:x?}", addr);
        }

        let MAX_SIZE_STR = 1_000_000;
        let mut s: Vec<u8> = Vec::new();
        let mut idx = addr;
        while idx < addr + MAX_SIZE_STR {
            let b = self.read_byte(idx);
            if b == 0 {
                break;
            }
            s.push(b);
            idx += 1;
        }
        if cfg!(feature = "log_mem_read") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem_trace: read_string: 0x{:x?} = {:x?}",
                        self.build_addresses(addr, s.len() + 1),
                        s
                    );
                }
            })
            .unwrap();
        }
        String::from_utf8(s).expect("invalid utf-8")
    }

    #[inline(always)]
    pub fn write_wide_string(&mut self, addr: u64, s: &str) {
        if !self.can_write() {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "FAILED doesn't have permission: write_from: 0x{:x?}",
                        addr
                    );
                }
            })
            .unwrap();
            panic!("FAILED to write without permission: addr: 0x{:x?}", addr);
        }

        let mut wide_string: Vec<u16> = s.encode_utf16().collect();
        if wide_string.last() != Some(&0) {
            wide_string.push(0);
        }
        let wide_string_byte_slice: &[u8] = cast_slice(&wide_string);
        self.write_bytes(addr, &wide_string_byte_slice);

        if cfg!(feature = "log_mem_write") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem_trace: write_wide_string: 0x{:x?} = {} ({:x?})",
                        self.build_addresses(addr, wide_string_byte_slice.len()),
                        s,
                        wide_string_byte_slice
                    );
                }
            })
            .unwrap();
        }
    }

    #[inline]
    pub fn read_wide_string(&self, addr: u64) -> String {
        if !self.can_read() {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "FAILED doesn't have permission: read_from: 0x{:x?}",
                        addr
                    );
                }
            })
            .unwrap();
            panic!("FAILED to read without permission: addr: 0x{:x?}", addr);
        }

        let MAX_SIZE_STR = 1_000_000;
        let mut s: Vec<u16> = Vec::new();
        let mut idx = addr;
        while idx < addr + MAX_SIZE_STR {
            let b = self.read_word(idx);
            if b == 0 {
                break;
            }
            s.push(b);
            idx += 2;
        }
        if cfg!(feature = "log_mem_read") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem_trace: read_wide_string: 0x{:x?} = {:x?}",
                        self.build_addresses(addr, s.len() + 1),
                        s
                    );
                }
            })
            .unwrap();
        }

        match String::from_utf16(&s) {
            Ok(s) => {
                return s;
            }
            Err(_) => {
                return "".to_string();
            }
        }
    }

    pub fn read_wide_string_n(&self, addr: u64, max_chars: usize) -> String {
        if !self.can_read() {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "FAILED doesn't have permission: read_from: 0x{:x?}",
                        addr
                    );
                }
            })
            .unwrap();
            panic!("FAILED to read without permission: addr: 0x{:x?}", addr);
        }

        let mut s: Vec<u16> = Vec::new();
        let mut idx = addr;
        for _ in 0..max_chars {
            let b = self.read_word(idx);
            if b == 0 {
                break;
            }
            s.push(b);
            idx += 2;
        }
        if cfg!(feature = "log_mem_read") {
            emu_context::with_current_emu(|emu| {
                if emu.cfg.trace_mem {
                    log_red!(
                        emu,
                        "mem_trace: read_wide_string_n: 0x{:x?} = {:x?}",
                        self.build_addresses(addr, s.len() + 1),
                        s
                    );
                }
            })
            .unwrap();
        }
        String::from_utf16_lossy(&s)
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
        self.mem.clear();
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
            Ok(_) => log::debug!(
                "saved. addr: 0x{:x} size: {} filename: {}",
                addr,
                size,
                filename
            ),
            Err(_) => log::error!("couldn't save the file"),
        }

        f.sync_all().unwrap();
    }

    pub fn save_all(&self, filename: &str) -> bool {
        let mut f = match File::create(filename) {
            Ok(f) => f,
            Err(e) => {
                return false;
            }
        };

        let blob = self.mem.get(0..).unwrap();

        let res = match f.write_all(blob) {
            Ok(_) => true,
            Err(_) => false,
        };

        f.sync_all().unwrap();

        res
    }
}
