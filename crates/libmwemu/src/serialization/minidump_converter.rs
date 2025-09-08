use ahash::AHashMap;
use minidump::*;
use slab::Slab;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::error::Error;
use std::ops::Deref;
use minidump::format::MemoryProtection;

use crate::maps::mem64::{Mem64, Permission};
use crate::maps::tlb::TLB;
use crate::maps::Maps;
use crate::regs64::Regs64;
use crate::serialization::emu::SerializableEmu;
use crate::serialization::maps::SerializableMaps;
use crate::serialization::pe32::SerializablePE32;
use crate::serialization::pe64::SerializablePE64;


pub struct MinidumpConverter;

impl MinidumpConverter {
    fn get_pe_offset(data: &[u8]) -> Option<usize> {
        if data.len() < 0x3C + 4 {
            return None;
        }
        let offset = u32::from_le_bytes([data[0x3C], data[0x3D], data[0x3E], data[0x3F]]) as usize;
        if offset < data.len() {
            Some(offset)
        } else {
            None
        }
    }

    fn is_pe64(data: &[u8], pe_offset: usize) -> bool {
        if pe_offset + 24 >= data.len() {
            return false;
        }
        // Check machine type in PE header - 0x8664 = x64
        let machine = u16::from_le_bytes([data[pe_offset + 4], data[pe_offset + 5]]);
        machine == 0x8664
    }

    fn extract_pe_modules<T: Deref<Target = [u8]>>(
        dump: &minidump::Minidump<'static, T>,
    ) -> Result<(Option<SerializablePE32>, Option<SerializablePE64>), Box<dyn Error>> {
        let mut pe32 = None;
        let mut pe64 = None;

        if let Ok(modules) = dump.get_stream::<MinidumpModuleList>() {
            let memory = dump.get_memory().unwrap_or_default();

            for module in modules.iter() {
                // Try to read the module from memory
                if let Some(mem_region) = memory.memory_at_address(module.base_address()) {
                    let raw_data = mem_region.bytes().to_vec();

                    // Basic PE detection - check for MZ header
                    if raw_data.len() > 0x40 && &raw_data[0..2] == b"MZ" {
                        // Read PE header to determine 32 vs 64 bit
                        if let Some(pe_offset) = Self::get_pe_offset(&raw_data) {
                            if Self::is_pe64(&raw_data, pe_offset) {
                                pe64 = Some(SerializablePE64 {
                                    filename: module.name.to_string(),
                                    raw: raw_data,
                                });
                            } else {
                                pe32 = Some(SerializablePE32 {
                                    filename: module.name.to_string(),
                                    raw: raw_data,
                                });
                            }
                        }
                    }
                }
            }
        }

        Ok((pe32, pe64))
    }

    fn extract_memory_maps<T: Deref<Target = [u8]>>(
        dump: &minidump::Minidump<'static, T>,
    ) -> Result<SerializableMaps, Box<dyn Error>> {
        let mut mem_slab = Slab::new();
        let mut maps = BTreeMap::new();
        let mut name_map = AHashMap::new();

        // Get memory regions from memory info list
        if let Ok(memory_info) = dump.get_stream::<MinidumpMemoryInfoList>() {
            let memory = dump.get_memory().unwrap_or_default();

            for info in memory_info.iter() {
                let base_addr = info.raw.base_address;
                let size = info.raw.region_size;
                let permission = match info.protection.bits() & MemoryProtection::ACCESS_MASK.bits() {
                    x if x == MemoryProtection::PAGE_NOACCESS.bits() => Permission::NONE,
                    x if x == MemoryProtection::PAGE_READONLY.bits() => Permission::READ,
                    x if x == MemoryProtection::PAGE_READWRITE.bits() => Permission::READ_WRITE,
                    x if x == MemoryProtection::PAGE_EXECUTE.bits() => Permission::EXECUTE,
                    x if x == MemoryProtection::PAGE_EXECUTE_READ.bits() => Permission::READ_EXECUTE,
                    x if x == MemoryProtection::PAGE_EXECUTE_READWRITE.bits() => Permission::READ_WRITE_EXECUTE,
                    _ => Permission::READ_WRITE_EXECUTE,
                };

                // Try to get the actual memory data for this region
                let mem_data = memory
                    .memory_at_address(base_addr)
                    .unwrap()
                    .bytes()
                    .to_vec();

                let mem_entry = Mem64::new(
                    format!("mem_0x{:016x}", base_addr), // name
                    base_addr,                           // base_addr
                    base_addr + size,                    // bottom_addr (base + size)
                    mem_data,                            // mem data
                    permission,
                );

                let slab_key = mem_slab.insert(mem_entry);
                maps.insert(base_addr, slab_key);
            }
        }

        // Also add mapped modules to name_map
        if let Ok(modules) = dump.get_stream::<MinidumpModuleList>() {
            for module in modules.iter() {
                let module_base = module.base_address();
                let module_size = module.size();

                // Find corresponding memory region that contains this module
                for (&addr, &slab_key) in &maps {
                    if addr <= module_base && module_base < addr + mem_slab[slab_key].size() as u64
                    {
                        name_map.insert(module.name.to_string(), slab_key);
                        break;
                    }
                }
            }
        }

        let system_info = dump.get_stream::<MinidumpSystemInfo>()?;
        let is_64bits = matches!(system_info.cpu, minidump::system_info::Cpu::X86_64);

        let banzai = false;
        let tlb = RefCell::new(TLB::new());

        Ok(SerializableMaps::new(Maps::new(
            mem_slab, maps, name_map, is_64bits, banzai, tlb,
        )))
    }

    pub fn from_minidump_file(path: &str) -> Result<SerializableEmu, Box<dyn Error>> {
        let dump = minidump::Minidump::read_path(path)?;

        // Get basic streams we need
        let system_info = dump.get_stream::<MinidumpSystemInfo>()?;
        let exception = dump.get_stream::<MinidumpException>()?;
        let threads = dump.get_stream::<MinidumpThreadList>()?;

        // Find crashed thread
        let crashed_thread = threads
            .threads
            .first()
            .ok_or("No threads found in minidump")?;

        // Extract PE modules
        let (pe32, pe64) = Self::extract_pe_modules(&dump)?;

        // Extract memory maps
        let maps = Self::extract_memory_maps(&dump)?;

        // Extract registers - just use defaults for now since context parsing is complex
        let regs = Regs64::default();

        // Basic serializable emu with minimal data
        let mut serializable_emu = SerializableEmu::default();
        serializable_emu.set_maps(maps);
        serializable_emu.set_regs(regs);
        serializable_emu.set_pe32(pe32);
        serializable_emu.set_pe64(pe64);

        Ok(serializable_emu)
    }
}
