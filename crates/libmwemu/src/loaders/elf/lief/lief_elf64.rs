use std::fs;
use std::io::Read;
use std::path::Path;

use crate::loaders::elf::elf64::{
    EI_NIDENT, ELFCLASS64, Elf64, Elf64DynamicInfo, Elf64Ehdr, Elf64Phdr, Elf64RelocationInfo,
    Elf64Shdr, Elf64Sym, ParserBackendKind,
};
use lief::elf;
use lief::generic::Binary as GenericBinary;
use lief::generic::Relocation as GenericRelocation;
use lief::generic::Section as GenericSection;
use lief::generic::Symbol as GenericSymbol;

use super::error::LiefElfError;

pub struct LiefElf64 {
    file_path: String,
    raw: Vec<u8>,
    binary: elf::Binary,
    _temp_file: Option<tempfile::NamedTempFile>,
}

fn file_type_to_u16(ft: elf::header::FileType) -> u16 {
    match ft {
        elf::header::FileType::NONE => 0,
        elf::header::FileType::REL => 1,
        elf::header::FileType::EXEC => 2,
        elf::header::FileType::DYN => 3,
        elf::header::FileType::CORE => 4,
        elf::header::FileType::UNKNOWN(v) => v as u16,
    }
}

fn arch_to_u16(a: elf::header::Arch) -> u16 {
    match a {
        elf::header::Arch::NONE => 0,
        elf::header::Arch::I386 => 3,
        elf::header::Arch::X86_64 => 0x3E,
        elf::header::Arch::ARM => 0x28,
        elf::header::Arch::AARCH64 => 0xB7,
        elf::header::Arch::UNKNOWN(v) => v as u16,
        _ => 0,
    }
}

fn class_to_u8(c: elf::header::Class) -> u8 {
    match c {
        elf::header::Class::NONE => 0,
        elf::header::Class::ELF32 => 1,
        elf::header::Class::ELF64 => 2,
        elf::header::Class::UNKNOWN(v) => v as u8,
    }
}

fn version_to_u32(v: elf::header::Version) -> u32 {
    match v {
        elf::header::Version::NONE => 0,
        elf::header::Version::CURRENT => 1,
        elf::header::Version::UNKNOWN(val) => val,
    }
}

fn osabi_to_u8(osabi: elf::header::OsAbi) -> u8 {
    match osabi {
        elf::header::OsAbi::SYSTEMV => 0,
        elf::header::OsAbi::LINUX => 3,
        elf::header::OsAbi::FREEBSD => 9,
        elf::header::OsAbi::UNKNOWN(v) => v as u8,
        _ => 0,
    }
}

fn segment_type_to_u32(pt: elf::segment::Type) -> u32 {
    match pt {
        elf::segment::Type::PT_NULL => 0,
        elf::segment::Type::LOAD => 1,
        elf::segment::Type::DYNAMIC => 2,
        elf::segment::Type::INTERP => 3,
        elf::segment::Type::NOTE => 4,
        elf::segment::Type::SHLIB => 5,
        elf::segment::Type::PHDR => 6,
        elf::segment::Type::TLS => 7,
        elf::segment::Type::GNU_EH_FRAME => 0x6474e550,
        elf::segment::Type::GNU_STACK => 0x6474e551,
        elf::segment::Type::GNU_RELRO => 0x6474e552,
        elf::segment::Type::GNU_PROPERTY => 0x6474e553,
        elf::segment::Type::PAX_FLAGS => 0x65041580,
        elf::segment::Type::UNKNOWN(v) => v as u32,
        _ => 0,
    }
}

fn section_type_to_u32(st: elf::section::Type) -> u32 {
    match st {
        elf::section::Type::SHT_NULL => 0,
        elf::section::Type::PROGBITS => 1,
        elf::section::Type::SYMTAB => 2,
        elf::section::Type::STRTAB => 3,
        elf::section::Type::RELA => 4,
        elf::section::Type::HASH => 5,
        elf::section::Type::DYNAMIC => 6,
        elf::section::Type::NOTE => 7,
        elf::section::Type::NOBITS => 8,
        elf::section::Type::REL => 9,
        elf::section::Type::SHLIB => 10,
        elf::section::Type::DYNSYM => 11,
        elf::section::Type::INIT_ARRAY => 14,
        elf::section::Type::FINI_ARRAY => 15,
        elf::section::Type::PREINIT_ARRAY => 16,
        elf::section::Type::GROUP => 17,
        elf::section::Type::SYMTAB_SHNDX => 18,
        elf::section::Type::RELR => 19,
        elf::section::Type::GNU_ATTRIBUTES => 0x6ffffff5,
        elf::section::Type::GNU_HASH => 0x6ffffff6,
        elf::section::Type::GNU_VERDEF => 0x6ffffffd,
        elf::section::Type::GNU_VERNEED => 0x6ffffffe,
        elf::section::Type::GNU_VERSYM => 0x6fffffff,
        elf::section::Type::UNKNOWN(v) => v as u32,
        _ => 0,
    }
}

fn validate_file_range(file_len: usize, offset: u64, size: u64) -> Result<(), LiefElfError> {
    let end = offset.checked_add(size).ok_or_else(|| {
        LiefElfError::ParseFailed(format!(
            "offset 0x{:x} + size 0x{:x} overflows u64",
            offset, size
        ))
    })?;
    if end > file_len as u64 {
        return Err(LiefElfError::ParseFailed(format!(
            "range 0x{:x}..0x{:x} exceeds file_len 0x{:x}",
            offset, end, file_len
        )));
    }
    Ok(())
}

impl LiefElf64 {
    pub fn load(path: &str) -> Result<Self, LiefElfError> {
        if !Path::new(path).exists() {
            return Err(LiefElfError::FileNotFound(path.to_string()));
        }

        let raw =
            fs::read(path).map_err(|e| LiefElfError::ReadFailed(format!("{}: {}", path, e)))?;

        let binary = elf::Binary::parse(path).ok_or_else(|| {
            LiefElfError::ParseFailed(format!("{}: LIEF parse returned None", path))
        })?;

        Self::from_binary(path, raw, binary, None)
    }

    pub fn load_from_raw(filename: &str, raw: &[u8]) -> Result<Self, LiefElfError> {
        let mut tmp = tempfile::NamedTempFile::new()
            .map_err(|e| LiefElfError::TempFileFailed(format!("{}", e)))?;
        std::io::Write::write_all(&mut tmp, raw)
            .map_err(|e| LiefElfError::TempFileFailed(format!("write: {}", e)))?;

        let binary = elf::Binary::parse(tmp.path()).ok_or_else(|| {
            LiefElfError::ParseFailed(format!("{}: LIEF parse returned None", filename))
        })?;

        Self::from_binary(filename, raw.to_vec(), binary, Some(tmp))
    }

    fn from_binary(
        filename: &str,
        raw: Vec<u8>,
        binary: elf::Binary,
        _temp_file: Option<tempfile::NamedTempFile>,
    ) -> Result<Self, LiefElfError> {
        let header = binary.header();
        if header.identity_class() != elf::header::Class::ELF64 {
            return Err(LiefElfError::UnsupportedClass(format!(
                "{:?}",
                header.identity_class()
            )));
        }

        match header.machine_type() {
            elf::header::Arch::X86_64 | elf::header::Arch::AARCH64 => {}
            ref machine => {
                return Err(LiefElfError::UnsupportedMachine {
                    machine: format!("{:?}", machine),
                });
            }
        }

        Ok(LiefElf64 {
            file_path: filename.to_string(),
            raw,
            binary,
            _temp_file,
        })
    }

    pub fn is_elf64_x64(path: &str) -> bool {
        Self::detect_with_machine(path, 0x3E)
    }

    pub fn is_elf64_aarch64(path: &str) -> bool {
        Self::detect_with_machine(path, 0xB7)
    }

    fn detect_with_machine(path: &str, expected: u16) -> bool {
        let mut fd = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(_) => return false,
        };
        let mut raw = vec![0u8; 20];
        if fd.read_exact(&mut raw).is_err() {
            return false;
        }
        if raw[0] != 0x7f
            || raw[1] != b'E'
            || raw[2] != b'L'
            || raw[3] != b'F'
            || raw[4] != ELFCLASS64
        {
            return false;
        }
        let e_machine = u16::from_le_bytes([raw[18], raw[19]]);
        e_machine == expected
    }

    pub fn to_legacy_model(&self) -> Result<Elf64, LiefElfError> {
        let header = self.binary.header();

        let mut e_ident = [0u8; EI_NIDENT];
        e_ident[0] = 0x7f;
        e_ident[1..4].copy_from_slice(b"ELF");
        e_ident[4] = class_to_u8(header.identity_class());
        match header.identity_data() {
            elf::header::ElfData::LSB => e_ident[5] = 1,
            elf::header::ElfData::MSB => e_ident[5] = 2,
            _ => e_ident[5] = 1,
        }
        e_ident[6] = version_to_u32(header.identity_version()) as u8;
        e_ident[7] = osabi_to_u8(header.identity_os_abi());
        if self.raw.len() >= EI_NIDENT {
            e_ident[8..EI_NIDENT].copy_from_slice(&self.raw[8..EI_NIDENT]);
        }

        let elf_hdr = Elf64Ehdr {
            e_ident,
            e_type: file_type_to_u16(header.file_type()),
            e_machine: arch_to_u16(header.machine_type()),
            e_version: version_to_u32(header.object_file_version()),
            e_entry: self.binary.entrypoint(),
            e_phoff: header.program_headers_offset(),
            e_shoff: header.section_headers_offset(),
            e_flags: header.processor_flag(),
            e_ehsize: header.header_size() as u16,
            e_phentsize: header.program_header_size() as u16,
            e_phnum: header.numberof_segments() as u16,
            e_shentsize: header.section_header_size() as u16,
            e_shnum: header.numberof_sections() as u16,
            e_shstrndx: header.section_name_table_idx() as u16,
        };

        let mut elf_phdr = Vec::new();
        for (i, seg) in self.binary.segments().enumerate() {
            let p_offset = seg.file_offset();
            let p_filesz = seg.physical_size();
            let p_memsz = seg.virtual_size();
            let is_load = seg.p_type() == elf::segment::Type::LOAD;

            if is_load && p_filesz > 0 {
                validate_file_range(self.raw.len(), p_offset, p_filesz).map_err(|_| {
                    LiefElfError::InvalidSegmentBounds {
                        index: i,
                        offset: p_offset,
                        size: p_filesz,
                        file_len: self.raw.len(),
                    }
                })?;
            }

            elf_phdr.push(Elf64Phdr {
                p_type: segment_type_to_u32(seg.p_type()),
                p_flags: seg.flags() as u32,
                p_offset,
                p_vaddr: seg.virtual_address(),
                p_paddr: seg.physical_address(),
                p_filesz,
                p_memsz,
                p_align: seg.alignment(),
            });
        }

        let mut section_names = Vec::new();
        let mut elf_shdr = Vec::new();
        for (i, sec) in self.binary.sections().enumerate() {
            let sh_offset = sec.offset();
            let sh_size = sec.size();

            let is_nobits = matches!(sec.get_type(), elf::section::Type::NOBITS);
            if !is_nobits && sh_size > 0 {
                validate_file_range(self.raw.len(), sh_offset, sh_size).map_err(|_| {
                    LiefElfError::InvalidSectionBounds {
                        index: i,
                        offset: sh_offset,
                        size: sh_size,
                        file_len: self.raw.len(),
                    }
                })?;
            }

            section_names.push(sec.name().to_string());
            elf_shdr.push(Elf64Shdr {
                sh_name: 0,
                sh_type: section_type_to_u32(sec.get_type()),
                sh_flags: sec.flags().bits() as u64,
                sh_addr: sec.virtual_address(),
                sh_offset,
                sh_size,
                sh_link: sec.link() as u32,
                sh_info: sec.information() as u32,
                sh_addralign: sec.alignment(),
                sh_entsize: sec.entry_size(),
            });
        }

        let (elf_strtab, name_offsets): (Vec<u8>, Vec<usize>) = {
            let mut strtab = vec![0u8];
            let mut offsets = Vec::new();
            for name in &section_names {
                offsets.push(strtab.len());
                strtab.extend_from_slice(name.as_bytes());
                strtab.push(0);
            }
            (strtab, offsets)
        };

        for (i, offset) in name_offsets.into_iter().enumerate() {
            if i < elf_shdr.len() {
                elf_shdr[i].sh_name = offset as u32;
            }
        }

        let mut needed_libs = Vec::new();
        for entry in self.binary.dynamic_entries() {
            use lief::elf::dynamic::DynamicEntry;
            if entry.tag() == elf::dynamic::Tag::NEEDED {
                use lief::elf::dynamic::Entries;
                if let Entries::Library(lib) = entry {
                    needed_libs.push(lib.name());
                }
            }
        }

        let lief_dynamic_info = Some(self.build_dynamic_info());

        let lief_relocations = self.parse_lief_relocations();

        let mut elf_dynsym = Vec::new();
        let mut sym_to_addr = std::collections::HashMap::new();
        let mut addr_to_symbol = std::collections::HashMap::new();

        for sym in self.binary.dynamic_symbols() {
            let name = sym.name();
            let st_value = sym.value();
            let st_size = sym.size();
            let st_info = sym.information();
            let st_other = sym.other();
            let st_shndx = sym.section_idx();

            elf_dynsym.push(Elf64Sym {
                st_dynstr_name: name.clone(),
                st_name: 0,
                st_info,
                st_other,
                st_shndx,
                st_value,
                st_size,
            });

            if !name.is_empty() && st_value > 0 {
                sym_to_addr.insert(name.clone(), st_value);
                addr_to_symbol.insert(st_value, name);
            }
        }

        let mut elf = Elf64 {
            base: 0,
            bin: self.raw.clone(),
            elf_hdr,
            elf_phdr,
            elf_shdr,
            elf_strtab,
            init: None,
            elf_dynsym,
            elf_dynstr_off: 0,
            elf_got_off: 0,
            needed_libs,
            sym_to_addr,
            addr_to_symbol,
            backend: ParserBackendKind::Lief,
            lief_dynamic_info,
            lief_relocations,
        };

        for shdr in &elf.elf_shdr {
            let sname = elf.get_section_name(shdr.sh_name as usize);
            if sname == ".dynstr" {
                elf.elf_dynstr_off = shdr.sh_offset;
            }
            if sname == ".got" {
                elf.elf_got_off = shdr.sh_offset;
            }
        }

        Ok(elf)
    }

    fn build_dynamic_info(&self) -> Elf64DynamicInfo {
        use lief::elf::dynamic::{DynamicEntry, Entries, Tag};

        let mut info = Elf64DynamicInfo::default();

        for entry in self.binary.dynamic_entries() {
            match entry.tag() {
                Tag::NEEDED => {
                    if let Entries::Library(lib) = entry {
                        info.needed.push(lib.name());
                    }
                }
                Tag::STRTAB => info.strtab_addr = entry.value(),
                Tag::SYMTAB => info.symtab_addr = entry.value(),
                Tag::SYMENT => info.syment = entry.value(),
                Tag::RELA => info.rela_addr = entry.value(),
                Tag::RELASZ => info.rela_size = entry.value(),
                Tag::RELAENT => info.rela_ent = entry.value(),
                Tag::JMPREL => info.jmprel_addr = entry.value(),
                Tag::PLTRELSZ => info.pltrelsz = entry.value(),
                _ => {}
            }
        }

        info
    }

    fn parse_lief_relocations(&self) -> Vec<Elf64RelocationInfo> {
        let mut relocs = Vec::new();

        let all_relocs = self
            .binary
            .dynamic_relocations()
            .into_iter()
            .chain(self.binary.pltgot_relocations().into_iter());

        for reloc in all_relocs {
            let symbol_name = reloc.symbol().map(|s| s.name());
            relocs.push(Elf64RelocationInfo {
                offset: reloc.address(),
                reloc_type: u32::from(reloc.get_type()),
                addend: reloc.addend(),
                symbol_name,
            });
        }

        relocs
    }
}
