use crate::{
    console::Console, windows::constants, emu::Emu, exception::types::ExceptionType,
    winapi::winapi32, winapi::winapi64,
};

impl Emu {
    ///TODO: reimplement set_eip and set_rip
    /// Redirect execution flow on 64bits.
    /// If the target address is a winapi, triggers it's implementation.
    pub fn set_rip(&mut self, addr: u64, is_branch: bool) -> bool {
        self.force_reload = true;

        if addr == constants::RETURN_THREAD as u64 {
            log::trace!("/!\\ Thread returned, continuing the main thread");
            self.regs_mut().rip = self.main_thread_cont;
            Console::spawn_console(self);
            self.force_break = true;
            return true;
        }

        let name = match self.maps.get_addr_name(addr) {
            Some(n) => n,
            None => {
                if self.os.is_linux() {
                    return false;
                }
                let import = self.pe64.as_ref().unwrap().import_addr_to_dll_and_name(addr);
                if !import.is_empty() {
                    let (dll, api) = import.split_once('!').unwrap_or(("", ""));

                    // In SSDT mode (`emulate_winapi`), we usually execute the real mapped DLL code.
                    // But api-set CRT imports are virtual; if they weren't bound, the IAT entry can
                    // still point into `.idata` (RVA), which is not executable. Handle them
                    // virtually by name instead of jumping to `addr`.
                    if self.cfg.emulate_winapi {
                        self.gateway_return = self.stack_pop64(false).unwrap_or(0);
                        self.regs_mut().rip = self.gateway_return;
                        winapi64::gateway_by_import(dll, api, self);
                        self.force_break = true;
                        self.is_api_run = true;
                        return true;
                    }

                    self.gateway_return = self.stack_pop64(false).unwrap_or(0);
                    self.regs_mut().rip = self.gateway_return;
                    winapi64::gateway(addr, "not_loaded", self);
                    self.force_break = true;
                    self.is_api_run = true;
                    return true;
                } else {
                    log::error!(
                        "/!\\ set_rip setting rip to non mapped addr 0x{:x} {}",
                        addr,
                        self.filename
                    );
                    self.exception(ExceptionType::SettingRipToNonMappedAddr);
                    return false;
                }
            }
        };

        let map_name = self.filename.as_str();

        /*
        if addr < constants::LIBS64_MIN
            || name == "code"
            || (!map_name.is_empty() && name.starts_with(&map_name))
            || name == "loader.text"*/
        if addr < constants::LIBS64_MIN {
            if self.cfg.verbose > 0 {
                let rip = self.regs().rip;
                let prev = self.maps.get_addr_name(rip).unwrap_or("??");
                if prev != name {
                    log::trace!("{}:0x{:x} map change  {} -> {}", self.pos, rip, prev, name);
                }
            }

            self.regs_mut().rip = addr;
        } else if self.os.is_linux() {
            self.regs_mut().rip = addr; // in linux libs are no implemented are emulated
        } else {
            if self.cfg.verbose >= 1 && !self.cfg.emulate_winapi {
                log::trace!("/!\\ changing RIP to {} ", name);
            }

            // emulate winapi
            if self.cfg.emulate_winapi {
                let api_name = winapi64::kernel32::guess_api_name(self, addr);
                if !api_name.is_empty() {
                    log_red!(self, "emulating {}", api_name);
                }
                self.regs_mut().rip = addr;
                return true;
            }

            if self.skip_apicall {
                self.its_apicall = Some(addr);
                return false;
            }

            self.gateway_return = self.stack_pop64(false).unwrap_or(0);
            self.regs_mut().rip = self.gateway_return;

            let handle_winapi: bool =
                if let Some(mut hook_fn) = self.hooks.hook_on_winapi_call.take() {
                    let rip = self.regs().rip;
                    let result = hook_fn(self, rip, addr);
                    self.hooks.hook_on_winapi_call = Some(hook_fn);
                    result
                } else {
                    true
                };

            if handle_winapi {
                let name = self
                    .maps
                    .get_addr_name(addr)
                    .expect("/!\\ changing RIP to non mapped addr 0x");
                winapi64::gateway(addr, name.to_string().as_str(), self);
            }
            self.force_break = true;
        }

        true
    }

    /// Redirect execution flow for AArch64 branches.
    /// If the target address is in a loaded library (dylib/so), intercept and
    /// dispatch to the appropriate API handler. Mirrors set_rip() for Windows.
    pub fn set_pc_aarch64(&mut self, addr: u64) -> bool {
        // If in user code range, just set PC
        if addr < constants::LIBS64_MIN {
            self.regs_aarch64_mut().pc = addr;
            return true;
        }

        // Execution is entering a library — intercept
        let name = match self.maps.get_addr_name(addr) {
            Some(n) => n.to_string(),
            None => {
                log::error!("set_pc_aarch64: addr 0x{:x} not in any mapped region", addr);
                self.regs_aarch64_mut().pc = addr;
                return false;
            }
        };

        // Look up symbol name from address
        let symbol = self
            .macho64
            .as_ref()
            .and_then(|m| m.addr_to_symbol.get(&addr).cloned())
            .unwrap_or_else(|| format!("unknown_0x{:x}", addr));

        log::info!(
            "{}** {} API call: {} (in {}) at 0x{:x} {}",
            self.colors.light_red,
            self.pos,
            symbol,
            name,
            addr,
            self.colors.nc
        );

        // Set PC to return address (already in LR from BL/BLR)
        self.regs_aarch64_mut().pc = self.regs_aarch64().x[30];

        // Dispatch to appropriate platform API handler
        if self.os.is_macos() {
            crate::macosapi::gateway(addr, &name, &symbol, self);
        } else if self.os.is_linux() {
            crate::linuxapi::gateway(addr, &name, &symbol, self);
        }

        self.force_break = true;
        true
    }

    /// Redirect execution flow on 32bits.
    /// If the target address is a winapi, triggers it's implementation.
    pub fn set_eip(&mut self, addr: u64, is_branch: bool) -> bool {
        self.force_reload = true;

        if addr == constants::RETURN_THREAD as u64 {
            log::trace!("/!\\ Thread returned, continuing the main thread");
            self.regs_mut().rip = self.main_thread_cont;
            Console::spawn_console(self);
            self.force_break = true;
            return true;
        }

        let name = match self.maps.get_addr_name(addr) {
            Some(n) => n,
            None => {
                if self.os.is_linux() {
                    return false;
                }
                let api_name = self.pe32.as_ref().unwrap().import_addr_to_name(addr as u32);
                if !api_name.is_empty() {
                    // winapi emulation case
                    if self.cfg.emulate_winapi {
                        let api_name = winapi32::kernel32::guess_api_name(self, addr as u32);
                        log_red!(self, "emulating {}", api_name);
                        self.regs_mut().set_eip(addr);
                        return true;
                    }

                    self.gateway_return = self.stack_pop32(false).unwrap_or(0) as u64;
                    self.regs_mut().rip = self.gateway_return;
                    winapi32::gateway(addr as u32, "not_loaded", self);
                    self.force_break = true;
                    self.is_api_run = true;
                    return true;
                } else {
                    log::error!("/!\\ setting eip to non mapped addr 0x{:x}", addr);
                    self.exception(ExceptionType::SettingRipToNonMappedAddr);
                    return false;
                }
            }
        };

        let map_name = self.filename_to_mapname(&self.filename);

        /*
        if name == "code"
            || addr < constants::LIBS32_MIN
            || (!map_name.is_empty() && name.starts_with(&map_name))
            || name == "loader.text"*/
        if addr < constants::LIBS32_MIN {
            if self.cfg.verbose > 0 {
                let eip = self.regs().get_eip();
                let prev = self.maps.get_addr_name(eip).unwrap_or("??");
                if prev != name {
                    log::trace!("{}:0x{:x} map change  {} -> {}", self.pos, eip, prev, name);
                }
            }

            self.regs_mut().set_eip(addr);
        } else {
            if self.cfg.verbose >= 1 && !self.cfg.emulate_winapi {
                log::trace!("/!\\ changing EIP to {} 0x{:x}", name, addr);
            }

            // winapi emulation case
            if self.cfg.emulate_winapi {
                let api_name = winapi32::kernel32::guess_api_name(self, addr as u32);
                if !api_name.is_empty() {
                    log_red!(self, "emulating {}", api_name);
                }
                self.regs_mut().set_eip(addr);
                return true;
            }

            if self.skip_apicall {
                self.its_apicall = Some(addr);
                return false;
            }

            self.gateway_return = self.stack_pop32(false).unwrap_or(0).into();
            let gateway_return = self.gateway_return;
            self.regs_mut().set_eip(gateway_return);

            let handle_winapi: bool =
                if let Some(mut hook_fn) = self.hooks.hook_on_winapi_call.take() {
                    let rip = self.regs().rip;
                    let result = hook_fn(self, rip, addr);
                    self.hooks.hook_on_winapi_call = Some(hook_fn);
                    result
                } else {
                    true
                };

            if handle_winapi {
                let name = self.maps.get_addr_name(addr).unwrap();
                winapi32::gateway(to32!(addr), name.to_string().as_str(), self);
            }
            self.force_break = true;
        }

        true
    }
}
