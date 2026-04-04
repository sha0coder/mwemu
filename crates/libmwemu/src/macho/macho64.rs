use crate::err::MwemuError;
use crate::maps::mem64::Permission;
use crate::maps::Maps;
use std::fs;
use std::io::Read;

const MH_MAGIC_64: u32 = 0xFEEDFACF;
const CPU_TYPE_ARM64: u32 = 0x0100000C;

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
}

fn prot_to_permission(prot: u32) -> Permission {
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
