use crate::{emu::Emu, structures::MemoryOperation};

impl Emu {
    /// Push a dword to the stack and dec the esp
    /// This will return false if stack pointer is pointing to non allocated place.
    pub fn stack_push32(&mut self, value: u32) -> bool {
        if self.cfg.stack_trace {
            log::info!("--- stack push32 ---");
            self.maps.dump_dwords(self.regs().get_esp(), 5);
        }

        if self.cfg.trace_mem {
            let name = self
                .maps
                .get_addr_name(self.regs().get_esp())
                .unwrap_or_else(|| "not mapped");
            let memory_operation = MemoryOperation {
                pos: self.pos,
                rip: self.regs().rip,
                op: "write".to_string(),
                bits: 32,
                address: self.regs().get_esp() - 4,
                old_value: self.maps.read_dword(self.regs().get_esp()).unwrap_or(0) as u64,
                new_value: value as u64,
                name: name.to_string(),
            };
            self.memory_operations.push(memory_operation);
            log::info!("\tmem_trace: pos = {} rip = {:x} op = write bits = {} address = 0x{:x} value = 0x{:x} name = '{}'",
                self.pos, self.regs().rip, 32, self.regs().get_esp(), value, name);
        }

        let esp = self.regs().get_esp() - 4;
        self.regs_mut().set_esp(esp);
        //self.stack_lvl[self.stack_lvl_idx] += 1;
        //log::info!("push32 stack level is {} deep {}", self.stack_lvl[self.stack_lvl_idx], self.stack_lvl_idx);

        /*
        let stack = self.maps.get_mem("stack");
        if stack.inside(self.regs().get_esp()) {
            if !self.maps.write_dword(self.regs().get_esp(), value) {
                //if !stack.write_dword(self.regs().get_esp(), value) {
                return false;
            }
        } else {
            let mem = match self.maps.get_mem_by_addr(self.regs().get_esp()) {
                Some(m) => m,
                None => {
                    log::info!(
                        "/!\\ pushing stack outside maps esp: 0x{:x}",
                        self.regs().get_esp()
                    );
                    Console::spawn_console(self);
                    return false;
                }
            };
            if !self.maps.write_dword(self.regs().get_esp(), value) {
                //if !mem.write_dword(self.regs().get_esp(), value) {
                return false;
            }
        }*/

        if self.maps.write_dword(self.regs().get_esp(), value) {
            true
        } else {
            log::info!(
                "/!\\ pushing in non mapped mem 0x{:x}",
                self.regs().get_esp()
            );
            false
        }
    }

    /// Push a qword to the stack and dec the rsp.
    /// This will return false if stack pointer is pointing to non allocated place.
    pub fn stack_push64(&mut self, value: u64) -> bool {
        if self.cfg.stack_trace {
            log::info!("--- stack push64  ---");
            self.maps.dump_qwords(self.regs().rsp, 5);
        }

        if self.cfg.trace_mem {
            let name = self
                .maps
                .get_addr_name(self.regs().rsp)
                .unwrap_or_else(|| "not mapped");
            let memory_operation = MemoryOperation {
                pos: self.pos,
                rip: self.regs().rip,
                op: "write".to_string(),
                bits: 64,
                address: self.regs().rsp - 8,
                old_value: self.maps.read_qword(self.regs().rsp).unwrap_or(0),
                new_value: value,
                name: name.to_string(),
            };
            self.memory_operations.push(memory_operation);
            log::info!("\tmem_trace: pos = {} rip = {:x} op = write bits = {} address = 0x{:x} value = 0x{:x} name = '{}'", self.pos, self.regs().rip, 64, self.regs().rsp, value, name);
        }

        self.regs_mut().rsp -= 8;
        //self.stack_lvl[self.stack_lvl_idx] += 1;
        //log::info!("push64 stack level is {} deep {}", self.stack_lvl[self.stack_lvl_idx], self.stack_lvl_idx);

        /*
        let stack = self.maps.get_mem("stack");
        if stack.inside(self.regs().rsp) {
            stack.write_qword(self.regs().rsp, value);
        } else {
            let mem = match self.maps.get_mem_by_addr(self.regs().rsp) {
                Some(m) => m,
                None => {
                    log::info!(
                        "pushing stack outside maps rsp: 0x{:x}",
                        self.regs().get_esp()
                    );
                    Console::spawn_console(self);
                    return false;
                }
            };
            mem.write_qword(self.regs().rsp, value);
        }*/

        if self.maps.write_qword(self.regs().rsp, value) {
            true
        } else {
            log::info!("/!\\ pushing in non mapped mem 0x{:x}", self.regs().rsp);
            false
        }
    }

    /// Pop a dword from stack and return it, None if esp points to unmapped zone.
    pub fn stack_pop32(&mut self, pop_instruction: bool) -> Option<u32> {
        if self.cfg.stack_trace {
            log::info!("--- stack pop32 ---");
            self.maps.dump_dwords(self.regs().get_esp(), 5);
        }

        /*
        let stack = self.maps.get_mem("stack");
        if stack.inside(self.regs().get_esp()) {
            //let value = stack.read_dword(self.regs().get_esp());
            let value = match self.maps.read_dword(self.regs().get_esp()) {
                Some(v) => v,
                None => {
                    log::info!("esp out of stack");
                    return None;
                }
            };
            if self.cfg.verbose >= 1
                && pop_instruction
                && self.maps.get_mem("code").inside(value.into())
            {
                log::info!("/!\\ poping a code address 0x{:x}", value);
            }
            let esp = self.regs().get_esp() + 4;
            self.regs_mut().set_esp(esp);
            return Some(value);
        }

        let mem = match self.maps.get_mem_by_addr(self.regs().get_esp()) {
            Some(m) => m,
            None => {
                log::info!(
                    "poping stack outside map  esp: 0x{:x}",
                    self.regs().get_esp() as u32
                );
                Console::spawn_console(self);
                return None;
            }
        };*/

        let value = match self.maps.read_dword(self.regs().get_esp()) {
            Some(v) => v,
            None => {
                log::info!("esp point to non mapped mem");
                return None;
            }
        };

        /*  walking mems in very pop is slow, and now we are not using "code" map
        if self.cfg.verbose >= 1
            && pop_instruction
            && self.maps.get_mem("code").inside(value.into())
        {
            log::info!("/!\\ poping a code address 0x{:x}", value);
        }
        */

        if self.cfg.trace_mem {
            // Record the read from stack memory
            let name = self
                .maps
                .get_addr_name(self.regs().get_esp())
                .unwrap_or_else(|| "not mapped");
            let read_operation = MemoryOperation {
                pos: self.pos,
                rip: self.regs().rip,
                op: "read".to_string(),
                bits: 32,
                address: self.regs().get_esp(),
                old_value: 0, // not needed for read
                new_value: value as u64,
                name: name.to_string(),
            };
            self.memory_operations.push(read_operation);
            log::info!("\tmem_trace: pos = {} rip = {:x} op = read bits = {} address = 0x{:x} value = 0x{:x} name = '{}'", 
                self.pos, self.regs().rip, 32, self.regs().get_esp(), value, name);

            // Record the write to register
            let write_operation = MemoryOperation {
                pos: self.pos,
                rip: self.regs().rip,
                op: "write".to_string(),
                bits: 32,
                address: self.regs().get_esp(),
                old_value: self.maps.read_dword(self.regs().get_esp()).unwrap_or(0) as u64,
                new_value: value as u64, // new value being written
                name: "register".to_string(),
            };
            self.memory_operations.push(write_operation);
            log::info!("\tmem_trace: pos = {} rip = {:x} op = write bits = {} address = 0x{:x} value = 0x{:x} name = 'register'", 
                self.pos, self.regs().rip, 32, self.regs().get_esp(), value);
        }

        let esp = self.regs().get_esp() + 4;
        self.regs_mut().set_esp(esp);
        //self.stack_lvl[self.stack_lvl_idx] -= 1;
        //log::info!("pop32 stack level is {} deep {}", self.stack_lvl[self.stack_lvl_idx], self.stack_lvl_idx);
        Some(value)
    }

    /// Pop a qword from stack, return None if cannot read the rsp address.
    pub fn stack_pop64(&mut self, pop_instruction: bool) -> Option<u64> {
        if self.cfg.stack_trace {
            log::info!("--- stack pop64 ---");
            self.maps.dump_qwords(self.regs().rsp, 5);
        }

        /*
        let stack = self.maps.get_mem("stack");
        if stack.inside(self.regs().rsp) {
            let value = stack.read_qword(self.regs().rsp);
            if self.cfg.verbose >= 1
                && pop_instruction
                && self.maps.get_mem("code").inside(value.into())
            {
                log::info!("/!\\ poping a code address 0x{:x}", value);
            }
            self.regs_mut().rsp += 8;
            return Some(value);
        }

        let mem = match self.maps.get_mem_by_addr(self.regs().rsp) {
            Some(m) => m,
            None => {
                log::info!("poping stack outside map  esp: 0x{:x}", self.regs().rsp);
                Console::spawn_console(self);
                return None;
            }
        };

        let value = mem.read_qword(self.regs().rsp);
        */

        let value = match self.maps.read_qword(self.regs().rsp) {
            Some(v) => v,
            None => {
                log::info!("rsp point to non mapped mem");
                return None;
            }
        };

        if self.cfg.trace_mem {
            // Record the read from stack memory
            let name = self
                .maps
                .get_addr_name(self.regs().rsp)
                .unwrap_or_else(|| "not mapped");
            let read_operation = MemoryOperation {
                pos: self.pos,
                rip: self.regs().rip,
                op: "read".to_string(),
                bits: 64, // Changed from 32 to 64 for 64-bit operations
                address: self.regs().rsp,
                old_value: 0, // not needed for read
                new_value: value as u64,
                name: name.to_string(),
            };
            self.memory_operations.push(read_operation);
            log::info!("\tmem_trace: pos = {} rip = {:x} op = read bits = {} address = 0x{:x} value = 0x{:x} name = '{}'", 
                self.pos, self.regs().rip, 64, self.regs().rsp, value, name);

            // Record the write to register
            let write_operation = MemoryOperation {
                pos: self.pos,
                rip: self.regs().rip,
                op: "write".to_string(),
                bits: 64, // Changed from 32 to 64 for 64-bit operations
                address: self.regs().rsp,
                old_value: self.maps.read_qword(self.regs().rsp).unwrap_or(0),
                new_value: value as u64, // new value being written
                name: "register".to_string(),
            };
            self.memory_operations.push(write_operation);
            log::info!("\tmem_trace: pos = {} rip = {:x} op = write bits = {} address = 0x{:x} value = 0x{:x} name = 'register'", 
                self.pos, self.regs().rip, 64, self.regs().rsp, value);
        }

        self.regs_mut().rsp += 8;
        //self.stack_lvl[self.stack_lvl_idx] -= 1;
        //log::info!("0x{:x} pop64 stack level is {} deep {}", self.regs().rip, self.stack_lvl[self.stack_lvl_idx], self.stack_lvl_idx);
        Some(value)
    }
}
