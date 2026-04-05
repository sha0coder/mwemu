use crate::{
    console::Console,
    emu::Emu,
    exception::handlers::{self, HandlerKind},
    exception::types::ExceptionType,
    threading::context::ArchThreadState,
};

impl Emu {
    pub fn veh(&self) -> u64 {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { veh, .. } => *veh,
            _ => panic!("veh() called on aarch64 emu"),
        }
    }

    pub fn set_veh(&mut self, value: u64) {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { veh, .. } => *veh = value,
            _ => panic!("set_veh() called on aarch64 emu"),
        }
    }

    pub fn uef(&self) -> u64 {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { uef, .. } => *uef,
            _ => panic!("uef() called on aarch64 emu"),
        }
    }

    pub fn set_uef(&mut self, value: u64) {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { uef, .. } => *uef = value,
            _ => panic!("set_uef() called on aarch64 emu"),
        }
    }

    pub fn eh_ctx(&self) -> u32 {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { eh_ctx, .. } => *eh_ctx,
            _ => panic!("eh_ctx() called on aarch64 emu"),
        }
    }

    pub fn set_eh_ctx(&mut self, value: u32) {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { eh_ctx, .. } => *eh_ctx = value,
            _ => panic!("set_eh_ctx() called on aarch64 emu"),
        }
    }

    pub fn seh(&self) -> u64 {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { seh, .. } => *seh,
            _ => panic!("seh() called on aarch64 emu"),
        }
    }

    pub fn set_seh(&mut self, value: u64) {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { seh, .. } => *seh = value,
            _ => panic!("set_seh() called on aarch64 emu"),
        }
    }

    /// Trigger an exception.
    /// If it has to be handled initiate contex tand jump to the programmed error routine.
    /// Support SEH, VEH and UEF
    pub fn exception(&mut self, ex_type: ExceptionType) {
        self.fault_count += 1;

        let addr: u64;
        let next: u64;

        // hook
        let handle_exception: bool = if let Some(mut hook_fn) = self.hooks.hook_on_exception.take() {
            let pc = self.pc();
            let result = hook_fn(self, pc, ex_type);
            self.hooks.hook_on_exception = Some(hook_fn);
            result
        } else {
            true
        };

        // aarch64 has no SEH/VEH/UEF
        if self.cfg.arch.is_aarch64() {
            log::trace!(
                "exception on aarch64 (no SEH/VEH support). pos = {} pc = {:x}",
                self.pos,
                self.pc()
            );
            return;
        }

        // No handled exceptions
        if self.seh() == 0 && self.veh() == 0 && self.uef() == 0 {
            log::trace!(
                "exception without any SEH handler nor vector configured. pos = {} rip = {:x}",
                self.pos,
                self.pc()
            );
            return;
        }

        // hook replaced handler
        if !handle_exception {
            log::trace!("cancelled exception handling from hook.");
            return;
        }

        // VEH
        if self.veh() > 0 {
            addr = self.veh();

            handlers::enter_for_handler(self, ex_type, HandlerKind::Veh);

            if self.cfg.is_x64() {
                self.set_rip(addr, false);
            } else {
                self.set_eip(addr, false);
            }

        // SEH
        } else if self.seh() > 0 {
            if self.cfg.is_x64() {
                // 64bits seh

                unimplemented!("check .pdata if exists");
            } else {
                // 32bits seh
                next = match self.maps.read_dword(self.seh()) {
                    Some(value) => value.into(),
                    None => {
                        log::trace!("exception wihout correct SEH");
                        return;
                    }
                };

                addr = match self.maps.read_dword(self.seh() + 4) {
                    Some(value) => value.into(),
                    None => {
                        log::trace!("exception without correct SEH.");
                        return;
                    }
                };
            }

            let con = Console::new();
            if self.running_script {
                self.set_seh(next);
                handlers::enter_for_handler(self, ex_type, HandlerKind::Seh);
                if self.cfg.is_x64() {
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
                handlers::enter_for_handler(self, ex_type, HandlerKind::Seh);
                if self.cfg.is_x64() {
                    self.set_rip(addr, false);
                } else {
                    self.set_eip(addr, false);
                }
            }
        } else if self.uef() > 0 {
            // UEF

            addr = self.uef();

            handlers::enter_for_handler(self, ex_type, HandlerKind::Uef);
            if self.cfg.is_x64() {
                self.set_rip(addr, false);
            } else {
                self.set_eip(addr, false);
            }
        } else {
            unreachable!();
        }
    }
}
