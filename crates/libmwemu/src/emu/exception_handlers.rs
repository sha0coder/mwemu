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

           If the handler return the search constant will jump to next handler in this order:
               VEH -> SEH1 -> SEH2 -> ... -> SEHn -> UEF -> terminate process


           unhandling:
               VEH:
                   - the api RemoveVectoredExceptionHandler removes the handler
               SEH:
                   - automatically when is triggered. SEH point to next SEH.
               UEF:
                   - SetUnhandledExceptionFilter


           Responses:

               VEH:
                   - EXCEPTION_CONTINUE_EXECUTION (continue to eip/rip which could be modified)
                   - EXCEPTION_CONTINUE_SEARCH (jump to next handler SEH -> UEF -> end proces)

               SEH:
                   - EXCEPTION_CONTINUE_EXECUTION (continue and not jump to except)
                   - EXCEPTION_CONTINUE_SEARCH (jump to next handler SEH -> UEF -> end process)
                   - EXCEPTION_EXECUTE_HANDLER (jump to except)

               UEF:
                   - EXCEPTION_CONTINUE_EXECUTION (continue to eip/rip which could be modified)
                   - EXCEPTION_CONTINUE_SEARCH (end process)

           64bits SEH:
               - is not a stack chain
               - search RUNTIME_FUNCTION entry in the .pdata table using BeginAddress ≤ RIP < EndAddress
               - in that entry there is the RVA of UNWIND_INFO struct on the .xdata


               - at .pdata, 12 bytes of runtime entries:

                   typedef struct _RUNTIME_FUNCTION {
                       DWORD BeginAddress;
                       DWORD EndAddress;
                       DWORD UnwindInfo;    // RVA to UNWIND_INFO at .xdata
                   } RUNTIME_FUNCTION, *PRUNTIME_FUNCTION;

               - unwind info in the .xdata:

                   typedef struct _UNWIND_INFO {
                       UBYTE Version : 3;   // always 1
                       UBYTE Flags   : 5;   // 0, EHANDLER, UHANDLER, etc.
                       UBYTE SizeOfProlog;
                       UBYTE CountOfCodes;  // Nº of UNWIND_CODE
                       UBYTE FrameRegister : 4; // (ie. RBP=5)
                       UBYTE FrameOffset   : 4; // frame scale
                       UNWIND_CODE UnwindCode[]; // descriptors
                       // opcional:
                       //  DWORD ExceptionHandler;   // RVA to handler if Flags indicate it
                       //  DWORD ExceptionData[];    // extra data for handler
                   } UNWIND_INFO, *PUNWIND_INFO;


        */

        let addr: u64;
        let next: u64;

        // hook
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

        // hook replaced handler
        if !handle_exception {
            log::info!("cancelled exception handling from hook.");
            return;
        }

        // VEH
        if self.veh() > 0 {
            addr = self.veh();

            exception::enter(self, ex_type);

            if self.cfg.is_64bits {
                self.set_rip(addr, false);
            } else {
                self.set_eip(addr, false);
            }

        // SEH
        } else if self.seh() > 0 {
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
