use crate::{console::Console, emu::Emu, exception, exception_type::ExceptionType};

impl Emu {
    pub fn veh(&self) -> u64 {
        self.threads[self.current_thread_id].veh
    }
    
    pub fn set_veh(&mut self, value: u64) {
        self.threads[self.current_thread_id].veh = value;
    }
    
    pub fn uef(&self) -> u64 {
        self.threads[self.current_thread_id].uef
    }
    
    pub fn set_uef(&mut self, value: u64) {
        self.threads[self.current_thread_id].uef = value;
    }
    
    pub fn eh_ctx(&self) -> u32 {
        self.threads[self.current_thread_id].eh_ctx
    }
    
    pub fn set_eh_ctx(&mut self, value: u32) {
        self.threads[self.current_thread_id].eh_ctx = value;
    }

    pub fn seh(&self) -> u64 {
        self.threads[self.current_thread_id].seh
    }
    
    pub fn set_seh(&mut self, value: u64) {
        self.threads[self.current_thread_id].seh = value;
    }


    /// Trigger an exception.
    /// If it has to be handled initiate contex tand jump to the programmed error routine.
    /// Support SEH, VEH and UEF
    pub fn exception(&mut self, ex_type: ExceptionType) {
        /*
            Priority: VEH → SEH → UEF
         */


        let addr: u64;
        let next: u64;

        let handle_exception: bool = match self.hooks.hook_on_exception {
            Some(hook_fn) => hook_fn(self, self.regs().rip, ex_type),
            None => true,
        };

        // No handled exceptions
        if self.seh() == 0 && self.veh() == 0 && self.uef() == 0 {
                log::info!(
                    "exception without any SEH handler nor vector configured. pos = {} rip = {:x}",
                    self.pos,
                    self.regs().rip
                );
                return;
        }

        if !handle_exception {
            log::info!("cancelled exception handling from hook.");
            return;
        }

        if self.veh() > 0 {
            // VEH
            
            addr = self.veh();

            exception::enter(self, ex_type);

            if self.cfg.is_64bits {
                self.set_rip(addr, false);
            } else {
                self.set_eip(addr, false);
            }

        } else if self.seh() > 0 {
            // SEH

            
            if self.cfg.is_64bits {
                // 64bits seh
                
                unimplemented!("check .pdata if exists");
                
            } else {
                // 32bits seh
                next = match self.maps.read_dword(self.seh()) {
                    Some(value) => value.into(),
                    None => {
                        log::info!("exception wihout correct SEH");
                        return;
                    }
                };

                addr = match self.maps.read_dword(self.seh() + 4) {
                    Some(value) => value.into(),
                    None => {
                        log::info!("exception without correct SEH.");
                        return;
                    }
                };
            }

            let con = Console::new();
            if self.running_script {
                self.set_seh(next);
                exception::enter(self, ex_type);
                if self.cfg.is_64bits {
                    self.set_rip(addr, false);
                } else {
                    self.set_eip(addr, false);
                }
                return;
            } 
            con.print("jump the exception pointer (y/n)?");

            let cmd = con.cmd();
            if cmd == "y" {
                self.set_seh(next);
                exception::enter(self, ex_type);
                if self.cfg.is_64bits {
                    self.set_rip(addr, false);
                } else {
                    self.set_eip(addr, false);
                }
            }


        } else if self.uef() > 0 {
            // UEF
            
            addr = self.uef();

            exception::enter(self, ex_type);
            if self.cfg.is_64bits {
                self.set_rip(addr, false);
            } else {
                self.set_eip(addr, false);
            }

        } else {
            unreachable!();
        }
    }
    
}
