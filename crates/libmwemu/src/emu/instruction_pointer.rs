use crate::{
    console::Console, constants, emu::Emu, exception_type::ExceptionType, to32, winapi::winapi32,
    winapi::winapi64,
};

impl Emu {
    /// Redirect execution flow on 64bits.
    /// If the target address is a winapi, triggers it's implementation.
    pub fn set_rip(&mut self, addr: u64, is_branch: bool) -> bool {
        self.force_reload = true;

        if addr == constants::RETURN_THREAD as u64 {
            log::info!("/!\\ Thread returned, continuing the main thread");
            self.regs_mut().rip = self.main_thread_cont;
            Console::spawn_console(self);
            self.force_break = true;
            return true;
        }

        let name = match self.maps.get_addr_name(addr) {
            Some(n) => n,
            None => {
                if self.linux {
                    return false;
                }
                let api_name = self.pe64.as_ref().unwrap().import_addr_to_name(addr);
                if !api_name.is_empty() {
                    self.gateway_return = self.stack_pop64(false).unwrap_or(0);
                    self.regs_mut().rip = self.gateway_return;
                    winapi64::gateway(addr, "not_loaded", self);
                    self.force_break = true;
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
                let prev = match self.maps.get_addr_name(rip) {
                    Some(n) => n,
                    None => "??",
                };

                if prev != name {
                    log::info!("{}:0x{:x} map change  {} -> {}", self.pos, rip, prev, name);
                }
            }

            self.regs_mut().rip = addr;
        } else if self.linux {
            self.regs_mut().rip = addr; // in linux libs are no implemented are emulated
        } else {
            if self.cfg.verbose >= 1 {
                log::info!("/!\\ changing RIP to {} ", name);
            }

            if self.skip_apicall {
                self.its_apicall = Some(addr);
                return false;
            }

            self.gateway_return = self.stack_pop64(false).unwrap_or(0);
            self.regs_mut().rip = self.gateway_return;

            let handle_winapi: bool = match self.hooks.hook_on_winapi_call {
                Some(hook_fn) => hook_fn(self, self.regs().rip, addr),
                None => true,
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

    /// Redirect execution flow on 32bits.
    /// If the target address is a winapi, triggers it's implementation.
    pub fn set_eip(&mut self, addr: u64, is_branch: bool) -> bool {
        self.force_reload = true;

        if addr == constants::RETURN_THREAD as u64 {
            log::info!("/!\\ Thread returned, continuing the main thread");
            self.regs_mut().rip = self.main_thread_cont;
            Console::spawn_console(self);
            self.force_break = true;
            return true;
        }

        let name = match self.maps.get_addr_name(addr) {
            Some(n) => n,
            None => {
                if self.linux {
                    return false;
                }
                let api_name = self.pe32.as_ref().unwrap().import_addr_to_name(addr as u32);
                if !api_name.is_empty() {
                    self.gateway_return = self.stack_pop32(false).unwrap_or(0) as u64;
                    self.regs_mut().rip = self.gateway_return;
                    winapi32::gateway(addr as u32, "not_loaded", self);
                    self.force_break = true;
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
                let prev = match self.maps.get_addr_name(eip) {
                    Some(n) => n,
                    None => "??",
                };
                if prev != name {
                    log::info!("{}:0x{:x} map change  {} -> {}", self.pos, eip, prev, name);
                }
            }

            self.regs_mut().set_eip(addr);
        } else {
            if self.cfg.verbose >= 1 {
                log::info!("/!\\ changing EIP to {} 0x{:x}", name, addr);
            }

            if self.skip_apicall {
                self.its_apicall = Some(addr);
                return false;
            }

            self.gateway_return = self.stack_pop32(false).unwrap_or(0).into();
            let gateway_return = self.gateway_return;
            self.regs_mut().set_eip(gateway_return);

            let handle_winapi: bool = match self.hooks.hook_on_winapi_call {
                Some(hook_fn) => hook_fn(self, self.regs().rip, addr),
                None => true,
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
