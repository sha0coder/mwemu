use iced_x86::Register;

use crate::arch::Arch;
use crate::emu::Emu;
use crate::loaders::elf::elf32::Elf32;
use crate::loaders::elf::elf64::Elf64;
use crate::loaders::macho::macho64::Macho64;
use crate::loaders::pe::{
    IMAGE_FILE_MACHINE_AMD64, IMAGE_FILE_MACHINE_ARM64, IMAGE_FILE_MACHINE_I386, pe_machine_type,
};
use crate::maps::mem64::Permission;
use crate::winapi::winapi64;
use crate::windows::constants;
use crate::windows::peb::{peb32, peb64};

mod elf;
mod macho;
mod pe;

impl Emu {
    /// Load a sample. It can be PE32, PE64, ELF32, ELF64 or shellcode.
    /// If its a shellcode cannot be known if is for windows or linux, it triggers also init() to
    /// setup windows simulator.
    /// For now mwemu also don't know if shellcode is for 32bits or 64bits, in commandline -6 has
    /// to be selected for indicating 64bits, and from python or rust the emu32() or emu64()
    /// construtor dtermines the engine.
    pub fn load_code(&mut self, filename: &str) {
        self.filename = filename.to_string();
        self.cfg.filename = self.filename.clone();

        // ELF32
        if Elf32::is_elf32(filename) && !self.cfg.shellcode {
            self.os = crate::arch::OperatingSystem::Linux;
            self.cfg.arch = Arch::X86;

            log::trace!("elf32 detected.");
            let mut elf32 = Elf32::parse(filename).unwrap();
            elf32.load(&mut self.maps);
            self.regs_mut().rip = (elf32.elf_hdr.e_entry as u64) + elf32.base();
            let stack_sz = 0x30000;
            let stack = self.alloc("stack", stack_sz, Permission::READ_WRITE);
            self.regs_mut().rsp = stack + (stack_sz / 2);
            self.elf32 = Some(elf32);

        // ELF64 AArch64
        } else if Elf64::is_elf64_aarch64(filename) && !self.cfg.shellcode {
            self.os = crate::arch::OperatingSystem::Linux;
            self.cfg.arch = Arch::Aarch64;
            self.maps.is_64bits = true;
            self.maps.clear();

            log::trace!("elf64 aarch64 detected.");
            // load_elf64 handles thread conversion (via init_linux64_aarch64)
            // and sets PC via set_pc()
            self.load_elf64(filename);

        // Mach-O AArch64
        } else if Macho64::is_macho64_aarch64(filename) && !self.cfg.shellcode {
            self.cfg.arch = Arch::Aarch64;
            self.maps.is_64bits = true;
            self.maps.clear();

            // Set maps folder for macOS dylibs (try repo root, then relative from crate)
            if self.cfg.maps_folder.is_empty() {
                if std::path::Path::new("maps/macos/aarch64").exists() {
                    self.cfg.maps_folder = "maps/macos/aarch64/".to_string();
                } else if std::path::Path::new("../../maps/macos/aarch64").exists() {
                    self.cfg.maps_folder = "../../maps/macos/aarch64/".to_string();
                }
            }

            log::trace!("macho64 aarch64 detected.");
            self.load_macho64(filename);

        // Mach-O x86_64
        } else if Macho64::is_macho64_x64(filename) && !self.cfg.shellcode {
            self.cfg.arch = Arch::X86_64;
            self.maps.is_64bits = true;
            self.maps.clear();

            // Set maps folder for macOS dylibs (try repo root, then relative from crate)
            if self.cfg.maps_folder.is_empty() {
                if std::path::Path::new("maps/macos/x86_64").exists() {
                    self.cfg.maps_folder = "maps/macos/x86_64/".to_string();
                } else if std::path::Path::new("../../maps/macos/x86_64").exists() {
                    self.cfg.maps_folder = "../../maps/macos/x86_64/".to_string();
                }
            }

            log::trace!("macho64 x86_64 detected.");
            self.load_macho64(filename);

        // ELF64 x86_64
        } else if Elf64::is_elf64_x64(filename) && !self.cfg.shellcode {
            self.os = crate::arch::OperatingSystem::Linux;
            self.cfg.arch = Arch::X86_64;
            self.maps.clear();

            log::trace!("elf64 x86_64 detected.");
            self.load_elf64(filename);

        // PE: use COFF Machine field to distinguish x86 / x86_64 / ARM64
        } else if !self.cfg.shellcode && pe_machine_type(filename) == Some(IMAGE_FILE_MACHINE_I386)
        {
            log::trace!(
                "PE32 x86 header detected (Machine=0x{:04x}).",
                IMAGE_FILE_MACHINE_I386
            );
            let clear_registers = false; // TODO: this needs to be more dynamic, like if we have a register set via args or not
            let clear_flags = false; // TODO: this needs to be more dynamic, like if we have a flag set via args or not
            self.cfg.arch = Arch::X86;
            self.os = crate::arch::OperatingSystem::Windows;

            // Set maps folder for Windows DLLs (try repo root, then relative from crate)
            if self.cfg.maps_folder.is_empty() {
                if std::path::Path::new("maps/windows/x86").exists() {
                    self.cfg.maps_folder = "maps/windows/x86/".to_string();
                } else if std::path::Path::new("../../maps/windows/x86").exists() {
                    self.cfg.maps_folder = "../../maps/windows/x86/".to_string();
                }
            }

            self.init_win32(clear_registers, clear_flags);
            let (base, _pe_off) = self.load_pe32(filename, true, 0);
            let ep = self.regs().rip;
            // emulating tls callbacks

            /*
            for i in 0..self.tls_callbacks.len() {
                self.regs_mut().rip = self.tls_callbacks[i];
                log::trace!("emulating tls_callback {} at 0x{:x}", i + 1, self.regs().rip);
                self.stack_push32(base);
                self.run(Some(base as u64));
            }*/

            self.regs_mut().rip = ep;

        // PE64 ARM64
        } else if !self.cfg.shellcode && pe_machine_type(filename) == Some(IMAGE_FILE_MACHINE_ARM64)
        {
            log::trace!(
                "PE64 ARM64 header detected (Machine=0x{:04x}). Windows AArch64 PE recognized.",
                IMAGE_FILE_MACHINE_ARM64
            );
            self.cfg.arch = Arch::Aarch64;
            self.os = crate::arch::OperatingSystem::Windows;
            self.maps.is_64bits = true;

            // Set maps folder for Windows ARM64 DLLs
            if self.cfg.maps_folder.is_empty() {
                if std::path::Path::new("maps/windows/aarch64").exists() {
                    self.cfg.maps_folder = "maps/windows/aarch64/".to_string();
                } else if std::path::Path::new("../../maps/windows/aarch64").exists() {
                    self.cfg.maps_folder = "../../maps/windows/aarch64/".to_string();
                }
            }

            let clear_registers = false;
            let clear_flags = false;
            self.init_win32(clear_registers, clear_flags);
            let (base, _pe_off) = self.load_pe64(filename, true, 0);
            let ep = self.pc();

            match self.pe64 {
                Some(ref pe64) => {
                    if pe64.is_dll() {
                        let regs = self.regs_aarch64_mut();
                        regs.x[0] = base; // hinstDLL
                        regs.x[1] = 1; // fdwReason = DLL_PROCESS_ATTACH
                        regs.x[2] = 0; // lpvReserved
                    }
                }
                _ => {
                    log::error!("No Pe64 found inside self");
                }
            }

            self.set_pc(ep);

        // PE64 x86_64
        } else if !self.cfg.shellcode && pe_machine_type(filename) == Some(IMAGE_FILE_MACHINE_AMD64)
        {
            log::trace!(
                "PE64 x86_64 header detected (Machine=0x{:04x}).",
                IMAGE_FILE_MACHINE_AMD64
            );
            let clear_registers = false; // TODO: this needs to be more dynamic, like if we have a register set via args or not
            let clear_flags = false; // TODO: this needs to be more dynamic, like if we have a flag set via args or not
            self.cfg.arch = Arch::X86_64;
            self.os = crate::arch::OperatingSystem::Windows;

            // Set maps folder for Windows DLLs (try repo root, then relative from crate)
            if self.cfg.maps_folder.is_empty() {
                if std::path::Path::new("maps/windows/x86_64").exists() {
                    self.cfg.maps_folder = "maps/windows/x86_64/".to_string();
                } else if std::path::Path::new("../../maps/windows/x86_64").exists() {
                    self.cfg.maps_folder = "../../maps/windows/x86_64/".to_string();
                }
            }

            self.init_win32(clear_registers, clear_flags);
            let (base, _pe_off) = self.load_pe64(filename, true, 0);
            let ep = self.regs().rip;

            match self.pe64 {
                Some(ref pe64) => {
                    // start loading dll
                    if pe64.is_dll() {
                        self.regs_mut().set_reg(Register::RCX, base);
                        self.regs_mut().set_reg(Register::RDX, 1);
                        self.regs_mut().set_reg(Register::R8L, 0);
                    }
                }
                _ => {
                    log::error!("No Pe64 found inside self");
                }
            }
            // Optional SSDT loader bootstrap: call ntdll!LdrInitializeThunk to perform loader init.
            if self.cfg.emulate_winapi {
                let ldr_init = winapi64::kernel32::resolve_api_name_in_module(
                    self,
                    "ntdll.dll",
                    "LdrInitializeThunk",
                );
                if ldr_init != 0 {
                    // Arrange return to entrypoint so execution continues normally.
                    self.regs_mut().rip = ep;
                    log::trace!("Initializing win32 64bits emulating ntdll!LdrInitializeThunk");
                    // LdrInitializeThunk(PCONTEXT Context, PVOID NtdllBase, PVOID Unused)
                    // The second argument must be ntdll's image base so the loader
                    // can parse its own PE headers during init.
                    let ntdll_base = self.maps.get_mem("ntdll.pe").get_base();

                    // Build a minimal x64 CONTEXT structure so LdrInitializeThunk does not
                    // null-deref when it reads CONTEXT.Rip to find the process entry point.
                    // x64 CONTEXT size is 0x4D0 bytes; key offsets:
                    //   +0x30  ContextFlags   (DWORD)
                    //   +0x98  Rsp            (QWORD)
                    //   +0xF8  Rip            (QWORD)
                    const CTX_SIZE: u64 = 0x4D0;
                    const CONTEXT_FULL: u32 = 0x10_007F;
                    let ctx_addr = self
                        .maps
                        .lib64_alloc(CTX_SIZE)
                        .expect("cannot alloc CONTEXT for LdrInitializeThunk");
                    self.maps
                        .create_map("ldr_context", ctx_addr, CTX_SIZE, Permission::READ_WRITE)
                        .expect("cannot create ldr_context map");
                    // ContextFlags
                    let _ = self.maps.write_dword(ctx_addr + 0x30, CONTEXT_FULL);
                    // Rsp: current stack pointer
                    let _ = self.maps.write_qword(ctx_addr + 0x98, self.regs().rsp);
                    // Rip: entry point — NtContinue will redirect execution here
                    let _ = self.maps.write_qword(ctx_addr + 0xF8, ep);

                    let _ = self.call64(ldr_init, &[ctx_addr, ntdll_base, 0]);
                    // Switch API dispatch to virtual stubs so PE code uses Rust
                    // implementations rather than executing real DLL machine code.
                    // Real DLL code writes non-volatile registers to shadow space,
                    // which can overlap with PE frame locals and corrupt saved RSP.
                    self.ldr_init_done = true;
                    log::trace!("ntdll!LdrInitializeThunk emulated completely.");
                    // Guest loader often clears `PEB+0x90`; restore before walking modules / main EP.
                    peb64::ensure_peb_system_dependent_07(self);

                    // LdrInitializeThunk was expected to bind the main image IAT but
                    // our emulation does not run LdrpProcessInitializationComplete fully.
                    // Bind it now so imported functions resolve to real stubs.
                    // Must run BEFORE rebuild_ldr_lists so that kernelbase/kernel32 .pe
                    // maps exist when the list is built.
                    let exe_base = self.base;
                    if let Some(mut pe) = self.pe64.take() {
                        pe.iat_binding(self, exe_base);
                        pe.delay_load_binding(self, exe_base);
                        self.pe64 = Some(pe);
                    }

                    // Ensure essential Windows DLLs are loaded with valid PE content so
                    // the PEB module list is usable by manual walkers (e.g. MinGW CRT).
                    // LdrInitializeThunk maps these via NtMapViewOfSection which only creates
                    // empty placeholder regions; we load the real PE files here.
                    for essential in &["kernelbase.dll", "kernel32.dll", "user32.dll"] {
                        winapi64::kernel32::load_library(self, essential);
                    }

                    // LdrInitializeThunk reinitializes the Ldr lists; if it didn't
                    // complete successfully the lists may be empty. Rebuild them
                    // from the actually-loaded PE images so user code can walk them.
                    peb64::rebuild_ldr_lists(self);
                } else if self.cfg.verbose >= 1 {
                    log::trace!("ssdt: could not resolve ntdll!LdrInitializeThunk");
                }
            }
            // emulating tls callbacks
            /*
            for i in 0..self.tls_callbacks.len() {
                self.regs_mut().rip = self.tls_callbacks[i];
                log::trace!("emulating tls_callback {} at 0x{:x}", i + 1, self.regs().rip);
                self.stack_push64(base);
                self.run(Some(base));
            }*/

            self.regs_mut().rip = ep;

        // Shellcode
        } else {
            log::trace!("shellcode detected.");
            let clear_registers = false; // TODO: this needs to be more dynamic, like if we have a register set via args or not
            let clear_flags = false; // TODO: this needs to be more dynamic, like if we have a flag set via args or not
            self.init_win32(clear_registers, clear_flags);
            let exe_name = self.cfg.exe_name.clone();
            if self.cfg.is_x64() {
                let (base, _pe_off) =
                    self.load_pe64(&format!("{}/{}", self.cfg.maps_folder, exe_name), false, 0);
                peb64::update_ldr_entry_base(&exe_name, base, self);
            } else {
                let (base, _pe_off) =
                    self.load_pe32(&format!("{}/{}", self.cfg.maps_folder, exe_name), false, 0);
                peb32::update_ldr_entry_base(&exe_name, base as u64, self);
            }

            if !self
                .maps
                .create_map(
                    "code",
                    self.cfg.code_base_addr,
                    0,
                    Permission::READ_WRITE_EXECUTE,
                )
                .expect("cannot create code map")
                .load(filename)
            {
                log::trace!("shellcode not found, select the file with -f");
                return;
            }
            let code = self.maps.get_mem_mut("code");
            code.extend(0xffff); // this could overlap an existing map
        }

        if self.cfg.entry_point != constants::CFG_DEFAULT_BASE {
            self.regs_mut().rip = self.cfg.entry_point;
        }

        /*if self.cfg.code_base_addr != constants::CFG_DEFAULT_BASE {
            let code = self.maps.get_mem("code");
            code.update_base(self.cfg.code_base_addr);
            code.update_bottom(self.cfg.code_base_addr + code.size() as u64);
        }*/
    }

    /// Load a shellcode from a variable.
    /// This assumes that there is no headers like PE/ELF and it's direclty code.
    /// Any OS simulation is triggered, but init() could be called by the user
    pub fn load_code_bytes(&mut self, bytes: &[u8]) {
        if self.cfg.verbose >= 1 {
            log::trace!("Loading shellcode from bytes");
        }

        self.init_cpu();

        let code = self
            .maps
            .create_map(
                "code",
                self.cfg.code_base_addr,
                bytes.len() as u64,
                Permission::READ_WRITE_EXECUTE,
            )
            .expect("cannot create code map");
        let base = code.get_base();
        code.write_bytes(base, bytes);
        self.set_pc(base);
    }
}
