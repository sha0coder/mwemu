use iced_x86::{Formatter as _, Instruction};

use crate::emu::Emu;

impl Emu {
    /// display 32bits main registers
    pub fn featured_regs32(&self) {
        self.regs().show_eax(&self.maps, 0);
        self.regs().show_ebx(&self.maps, 0);
        self.regs().show_ecx(&self.maps, 0);
        self.regs().show_edx(&self.maps, 0);
        self.regs().show_esi(&self.maps, 0);
        self.regs().show_edi(&self.maps, 0);
        log::info!("\tesp: 0x{:x}", self.regs().get_esp() as u32);
        log::info!("\tebp: 0x{:x}", self.regs().get_ebp() as u32);
        log::info!("\teip: 0x{:x}", self.regs().get_eip() as u32);
    }

    /// display 64bits main registers
    pub fn featured_regs64(&self) {
        self.regs().show_rax(&self.maps, 0);
        self.regs().show_rbx(&self.maps, 0);
        self.regs().show_rcx(&self.maps, 0);
        self.regs().show_rdx(&self.maps, 0);
        self.regs().show_rsi(&self.maps, 0);
        self.regs().show_rdi(&self.maps, 0);
        log::info!("\trsp: 0x{:x}", self.regs().rsp);
        log::info!("\trbp: 0x{:x}", self.regs().rbp);
        log::info!("\trip: 0x{:x}", self.regs().rip);
        self.regs().show_r8(&self.maps, 0);
        self.regs().show_r9(&self.maps, 0);
        self.regs().show_r10(&self.maps, 0);
        self.regs().show_r11(&self.maps, 0);
        self.regs().show_r12(&self.maps, 0);
        self.regs().show_r13(&self.maps, 0);
        self.regs().show_r14(&self.maps, 0);
        self.regs().show_r15(&self.maps, 0);
    }


    pub fn show_definition(&mut self) {
        let rip = self.regs().rip;
        let definitions = &self.cfg.definitions;
        if let Some(definition) = definitions.get(&rip) {
            log::info!("Function definition: {} (0x{:x})", definition.name, rip);
                
            for (i, param) in definition.parameters.iter().enumerate() {
                let value = match i {
                    0 => self.regs().rcx,
                    1 => self.regs().rdx,
                    2 => self.regs().r8,
                    3 => self.regs().r9,
                    _ => {
                        // Stack parameters (5th and beyond) at rsp + 0x20 + (i-4)*8
                        let addr = self.regs().rsp + 0x20 + ((i - 4) * 8) as u64;
                        self.maps.read_qword(addr).unwrap_or(0)
                    }
                };
                
                let display_value = match param.param_type.as_str() {
                    "pointer" => format!("0x{:x}", value),
                    "wide_string" => {
                        if value != 0 {
                            let s = self.maps.read_wide_string(value);
                            format!("L\"{}\" (0x{:x})", s, value)
                        } else {
                            "NULL".to_string()
                        }
                    }
                    _ => format!("0x{:x}", value),
                };
                
                log::info!("    {}: {}", param.name, display_value);
            }
        }
    }

    #[inline]
    pub fn show_instruction_comment(&mut self, color: &str, ins: &Instruction, comment: &str) {
        if self.cfg.verbose < 2 {
            return;
        }
        let mut out: String = String::new();
        self.formatter.format(ins, &mut out);
        if self.cfg.verbose >= 2 {
            if self.cfg.nocolors {
                log::info!(
                    "{} 0x{:x}: {} ; {}",
                    self.pos,
                    ins.ip(),
                    out,
                    comment
                );
            } else {
                log::info!(
                    "{}{} 0x{:x}: {} ; {}{}",
                    color,
                    self.pos,
                    ins.ip(),
                    out,
                    comment,
                    self.colors.nc
                );
            }
            self.show_definition();
        }
    }

    #[inline]
    pub fn show_instruction(&mut self, color: &str, ins: &Instruction) {
        if self.cfg.verbose < 2 {
            return;
        }
        let mut out: String = String::new();
        self.formatter.format(ins, &mut out);
        if self.cfg.verbose >= 2 {
            if self.cfg.nocolors {
                log::info!(
                    "{} 0x{:x}: {}",
                    self.pos,
                    ins.ip(),
                    out
                );
            } else {
                log::info!(
                    "{}{} 0x{:x}: {}{}",
                    color,
                    self.pos,
                    ins.ip(),
                    out,
                    self.colors.nc
                );
            }
            self.show_definition();
        }
    }

    #[inline]
    pub fn show_instruction_ret(&mut self, color: &str, ins: &Instruction, addr: u64) {
        if self.cfg.verbose < 2 {
            return;
        }
        let mut out: String = String::new();
        self.formatter.format(ins, &mut out);
        if self.cfg.verbose >= 2 {
            if self.cfg.nocolors {
                log::info!(
                    "{} 0x{:x}: {} ; ret-addr: 0x{:x} ret-value: 0x{:x}",
                    self.pos,
                    ins.ip(),
                    out,
                    addr,
                    self.regs().rax
                );
            } else {
                log::info!(
                    "{}{} 0x{:x}: {} ; ret-addr: 0x{:x} ret-value: 0x{:x} {}",
                    color,
                    self.pos,
                    ins.ip(),
                    out,
                    addr,
                    self.regs().rax,
                    self.colors.nc
                );
            }
            self.show_definition();
        }
    }

    #[inline]
    pub fn show_instruction_pushpop(&mut self, color: &str, ins: &Instruction, value: u64) {
        if self.cfg.verbose < 2 {
            return;
        }
        let mut out: String = String::new();
        self.formatter.format(ins, &mut out);
        if self.cfg.verbose >= 2 {
            if self.cfg.nocolors {
                log::info!(
                    "{} 0x{:x}: {} ;0x{:x}",
                    self.pos,
                    ins.ip(),
                    out,
                    value
                );
            } else {
                log::info!(
                    "{}{} 0x{:x}: {} ;0x{:x} {}",
                    color,
                    self.pos,
                    ins.ip(),
                    out,
                    value,
                    self.colors.nc
                );
            }
            self.show_definition();
        }
    }

    #[inline]
    pub fn show_instruction_taken(&mut self, color: &str, ins: &Instruction) {
        if self.cfg.verbose < 2 {
            return;
        }
        let mut out: String = String::new();
        self.formatter.format(ins, &mut out);
        if self.cfg.verbose >= 2 {
            if self.cfg.nocolors {
                log::info!(
                    "{} 0x{:x}: {} taken",
                    self.pos,
                    ins.ip(),
                    out
                );
            } else {
                log::info!(
                    "{}{} 0x{:x}: {} taken {}",
                    color,
                    self.pos,
                    ins.ip(),
                    out,
                    self.colors.nc
                );
            }
            self.show_definition();
        }
    }

    pub fn show_instruction_not_taken(&mut self, color: &str, ins: &Instruction) {
        if self.cfg.verbose < 2 {
            return;
        }
        let mut out: String = String::new();
        self.formatter.format(ins, &mut out);
        if self.cfg.verbose >= 2 {
            if self.cfg.nocolors {
                log::info!(
                    "{} 0x{:x}: {} not taken",
                    self.pos,
                    ins.ip(),
                    out
                );
            } else {
                log::info!(
                    "{}{} 0x{:x}: {} not taken {}",
                    color,
                    self.pos,
                    ins.ip(),
                    out,
                    self.colors.nc
                );
            }
            self.show_definition();
        }
    }
}
