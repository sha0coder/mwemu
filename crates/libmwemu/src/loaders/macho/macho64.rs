use crate::err::MwemuError;
use crate::maps::mem64::Permission;
use crate::maps::Maps;
use goblin::mach::load_command::CommandVariant;
use std::fs;
use std::io::Read;

const MH_MAGIC_64: u32 = 0xFEEDFACF;
const CPU_TYPE_ARM64: u32 = 0x0100000C;

// Chained fixup pointer format constants
const DYLD_CHAINED_PTR_64_OFFSET: u16 = 6;

#[derive(Debug)]
pub struct Macho64Segment {
    pub name: String,
    pub vmaddr: u64,
    pub vmsize: u64,
    pub data: Vec<u8>,
    pub initprot: u32, // VM_PROT_READ=1, VM_PROT_WRITE=2, VM_PROT_EXECUTE=4
}

#[derive(Debug)]
pub struct Macho64 {
    pub bin: Vec<u8>,
    pub entry: u64,
    pub segments: Vec<Macho64Segment>,
}

/// A resolved chained fixup bind entry: GOT address -> import ordinal
#[derive(Debug)]
pub struct ChainedBind {
    pub got_vmaddr: u64,
    pub import_ordinal: u32,
}

/// An import from the chained fixups import table
#[derive(Debug)]
pub struct ChainedImport {
    pub name: String,
    pub lib_ordinal: i8,
    pub weak: bool,
}

impl Macho64 {
    /// Detect a 64-bit AArch64 Mach-O file by reading the first 8 bytes.
    pub fn is_macho64_aarch64(filename: &str) -> bool {
        let mut f = match std::fs::File::open(filename) {
            Ok(f) => f,
            Err(_) => return false,
        };
        let mut buf = [0u8; 8];
        if f.read_exact(&mut buf).is_err() {
            return false;
        }
        let magic = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
        let cputype = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]);
        magic == MH_MAGIC_64 && cputype == CPU_TYPE_ARM64
    }

    /// Parse a Mach-O 64-bit AArch64 binary using goblin.
    pub fn parse(filename: &str) -> Result<Macho64, MwemuError> {
        let bin = fs::read(filename)
            .map_err(|e| MwemuError::new(&format!("cannot read macho binary: {}", e)))?;

        let macho = goblin::mach::MachO::parse(&bin, 0)
            .map_err(|e| MwemuError::new(&format!("cannot parse macho binary: {}", e)))?;

        if !macho.is_64 {
            return Err(MwemuError::new("not a 64-bit Mach-O"));
        }

        let entry = macho.entry;

        let mut segments = Vec::new();
        for segment in &macho.segments {
            let name = segment
                .name()
                .map_err(|e| MwemuError::new(&format!("cannot read segment name: {}", e)))?
                .to_string();

            // __PAGEZERO is a guard page with no file data — skip it
            if name == "__PAGEZERO" {
                continue;
            }

            segments.push(Macho64Segment {
                name,
                vmaddr: segment.vmaddr,
                vmsize: segment.vmsize,
                data: segment.data.to_vec(),
                initprot: segment.initprot,
            });
        }

        log::info!(
            "macho64: {} segments, entry=0x{:x}",
            segments.len(),
            entry
        );

        Ok(Macho64 {
            bin,
            entry,
            segments,
        })
    }

    /// Load parsed segments into emulator memory maps.
    pub fn load(&self, maps: &mut Maps) {
        for seg in &self.segments {
            if seg.vmsize == 0 {
                continue;
            }

            let perm = prot_to_permission(seg.initprot);

            log::info!(
                "macho64: mapping segment '{}' at 0x{:x} size 0x{:x} prot=0x{:x}",
                seg.name,
                seg.vmaddr,
                seg.vmsize,
                seg.initprot
            );

            let mem = maps
                .create_map(&seg.name, seg.vmaddr, seg.vmsize, perm)
                .expect(&format!(
                    "cannot create map for segment '{}' at 0x{:x}",
                    seg.name, seg.vmaddr
                ));

            if !seg.data.is_empty() {
                mem.force_write_bytes(seg.vmaddr, &seg.data);
            }
        }
    }

    /// Get the list of dependent dylib paths from load commands.
    pub fn get_libs(&self) -> Vec<String> {
        let macho = goblin::mach::MachO::parse(&self.bin, 0).expect("re-parse for libs");
        macho
            .libs
            .iter()
            .filter(|l| **l != "self")
            .map(|l| l.to_string())
            .collect()
    }

    /// Get exported symbols with their offsets (relative to binary load address).
    pub fn get_exports(&self) -> Vec<(String, u64)> {
        let macho = goblin::mach::MachO::parse(&self.bin, 0).expect("re-parse for exports");
        match macho.exports() {
            Ok(exports) => exports
                .iter()
                .map(|e| (e.name.clone(), e.offset))
                .collect(),
            Err(e) => {
                log::warn!("macho64: cannot read exports: {}", e);
                Vec::new()
            }
        }
    }

    /// Parse chained fixups to extract imports and their GOT bind locations.
    /// Returns (imports_table, bind_entries) where each bind entry references
    /// an import by ordinal and specifies the GOT vmaddr to patch.
    pub fn parse_chained_fixups(&self) -> (Vec<ChainedImport>, Vec<ChainedBind>) {
        let macho = goblin::mach::MachO::parse(&self.bin, 0).expect("re-parse for fixups");

        let mut imports = Vec::new();
        let mut binds = Vec::new();

        for lc in &macho.load_commands {
            if let CommandVariant::DyldChainedFixups(cmd) = &lc.command {
                let base = cmd.dataoff as usize;
                let data = &self.bin[base..base + cmd.datasize as usize];

                // dyld_chained_fixups_header
                let starts_offset = u32::from_le_bytes(data[4..8].try_into().unwrap()) as usize;
                let imports_offset =
                    u32::from_le_bytes(data[8..12].try_into().unwrap()) as usize;
                let symbols_offset =
                    u32::from_le_bytes(data[12..16].try_into().unwrap()) as usize;
                let imports_count =
                    u32::from_le_bytes(data[16..20].try_into().unwrap()) as u32;
                let imports_format =
                    u32::from_le_bytes(data[20..24].try_into().unwrap());

                // Parse import table
                for i in 0..imports_count {
                    let imp = match imports_format {
                        1 => {
                            // DYLD_CHAINED_IMPORT: lib_ordinal:8, weak_import:1, name_offset:23
                            let off = imports_offset + (i as usize * 4);
                            let raw =
                                u32::from_le_bytes(data[off..off + 4].try_into().unwrap());
                            let lib_ordinal = (raw & 0xFF) as i8;
                            let weak = (raw >> 8) & 1 != 0;
                            let name_offset = (raw >> 9) as usize;
                            let name_start = symbols_offset + name_offset;
                            let name_end = data[name_start..]
                                .iter()
                                .position(|b| *b == 0)
                                .unwrap_or(0)
                                + name_start;
                            let name = std::str::from_utf8(&data[name_start..name_end])
                                .unwrap_or("")
                                .to_string();
                            ChainedImport {
                                name,
                                lib_ordinal,
                                weak,
                            }
                        }
                        _ => {
                            log::warn!(
                                "macho64: unsupported chained import format {}",
                                imports_format
                            );
                            break;
                        }
                    };
                    imports.push(imp);
                }

                // Parse starts_in_image to walk chains and find BIND entries
                let seg_count = u32::from_le_bytes(
                    data[starts_offset..starts_offset + 4].try_into().unwrap(),
                ) as usize;

                for seg_idx in 0..seg_count {
                    let seg_info_off = u32::from_le_bytes(
                        data[starts_offset + 4 + seg_idx * 4
                            ..starts_offset + 8 + seg_idx * 4]
                            .try_into()
                            .unwrap(),
                    );
                    if seg_info_off == 0 {
                        continue;
                    }
                    let si = starts_offset + seg_info_off as usize;

                    let page_size =
                        u16::from_le_bytes(data[si + 4..si + 6].try_into().unwrap()) as usize;
                    let pointer_format =
                        u16::from_le_bytes(data[si + 6..si + 8].try_into().unwrap());
                    let page_count =
                        u16::from_le_bytes(data[si + 20..si + 22].try_into().unwrap())
                            as usize;

                    // Get segment vmaddr and fileoff from goblin's segment list
                    let (seg_vmaddr, seg_fileoff) = macho
                        .segments
                        .get(seg_idx)
                        .map(|s| (s.vmaddr, s.fileoff as usize))
                        .unwrap_or((0, 0));

                    for p in 0..page_count {
                        let page_start = u16::from_le_bytes(
                            data[si + 22 + p * 2..si + 24 + p * 2].try_into().unwrap(),
                        );
                        if page_start == 0xFFFF {
                            continue;
                        }

                        let chain_file_base =
                            seg_fileoff + p * page_size + page_start as usize;
                        let chain_vm_base =
                            seg_vmaddr + (p as u64 * page_size as u64) + page_start as u64;

                        self.walk_chain(
                            pointer_format,
                            chain_file_base,
                            chain_vm_base,
                            &mut binds,
                        );
                    }
                }
            }
        }

        log::info!(
            "macho64: parsed {} imports, {} bind entries from chained fixups",
            imports.len(),
            binds.len()
        );

        (imports, binds)
    }

    /// Walk a chain of fixup pointers starting at file_off/vmaddr.
    fn walk_chain(
        &self,
        pointer_format: u16,
        mut file_off: usize,
        mut vmaddr: u64,
        binds: &mut Vec<ChainedBind>,
    ) {
        loop {
            if file_off + 8 > self.bin.len() {
                break;
            }
            let raw =
                u64::from_le_bytes(self.bin[file_off..file_off + 8].try_into().unwrap());

            let (bind, next, ordinal) = match pointer_format {
                DYLD_CHAINED_PTR_64_OFFSET | 2 => {
                    // DYLD_CHAINED_PTR_64 / DYLD_CHAINED_PTR_64_OFFSET
                    // bind:1 (bit 63), next:12 (bits 51-62), ordinal:24 (bits 0-23)
                    let bind = raw >> 63;
                    let next = ((raw >> 51) & 0xFFF) as usize;
                    let ordinal = (raw & 0xFFFFFF) as u32;
                    (bind, next, ordinal)
                }
                _ => {
                    log::warn!(
                        "macho64: unsupported chained pointer format {} at 0x{:x}",
                        pointer_format,
                        vmaddr
                    );
                    break;
                }
            };

            if bind == 1 {
                log::trace!(
                    "macho64: chained BIND at 0x{:x} ordinal={}",
                    vmaddr,
                    ordinal
                );
                binds.push(ChainedBind {
                    got_vmaddr: vmaddr,
                    import_ordinal: ordinal,
                });
            }

            if next == 0 {
                break;
            }
            file_off += next * 4;
            vmaddr += (next * 4) as u64;
        }
    }
}

pub(crate) fn prot_to_permission(prot: u32) -> Permission {
    let r = prot & 1 != 0; // VM_PROT_READ
    let w = prot & 2 != 0; // VM_PROT_WRITE
    let x = prot & 4 != 0; // VM_PROT_EXECUTE

    match (r, w, x) {
        (true, true, true) => Permission::READ_WRITE_EXECUTE,
        (true, true, false) => Permission::READ_WRITE,
        (true, false, true) => Permission::READ_EXECUTE,
        (true, false, false) => Permission::READ,
        (false, true, false) => Permission::WRITE,
        (false, false, true) => Permission::EXECUTE,
        _ => Permission::READ_WRITE, // fallback
    }
}
