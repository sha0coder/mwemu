use std::io::Write as _;

use iced_x86::Formatter as _;

use crate::{constants, emu::Emu, flags::Flags, regs64::Regs64};

impl Emu {
    pub fn open_trace_file(&mut self) {
        if let Some(filename) = self.cfg.trace_filename.clone() {
            let mut trace_file =
                std::fs::File::create(&filename).expect("Failed to create trace file");
            writeln!(
                trace_file,
                r#""Index","Address","Bytes","Disassembly","Registers","Memory","Comments""#
            )
            .expect("Failed to write trace file header");
            self.trace_file = Some(trace_file);
        }
    }

    #[inline]
    pub fn capture_pre_op(&mut self) {
        self.set_pre_op_regs(*self.regs());
        self.set_pre_op_flags(*self.flags());
    }

    #[inline]
    pub fn capture_post_op(&mut self) {
        self.set_post_op_regs(*self.regs());
        self.set_post_op_flags(*self.flags());
    }

    /// dump the registers and memory write operations to file
    pub fn write_to_trace_file(&mut self) {
        let index = self.pos - 1;

        let instruction = self.instruction.unwrap();
        let instruction_size = instruction.len();
        let instruction_bytes = self.maps.read_bytes(self.regs().rip, instruction_size);
        let mut output: String = String::new();
        self.formatter.format(&instruction, &mut output);

        let mut comments = String::new();

        // dump all registers on first, only differences on next
        let pre_op_regs = self.pre_op_regs();
        let post_op_regs = self.post_op_regs();
        let mut registers = String::new();
        if self.pos == self.cfg.trace_start || index == 0 {
            registers = format!(
                "{} rax: {:x}-> {:x}",
                registers, pre_op_regs.rax, post_op_regs.rax
            );
            registers = format!(
                "{} rbx: {:x}-> {:x}",
                registers, pre_op_regs.rbx, post_op_regs.rbx
            );
            registers = format!(
                "{} rcx: {:x}-> {:x}",
                registers, pre_op_regs.rcx, post_op_regs.rcx
            );
            registers = format!(
                "{} rdx: {:x}-> {:x}",
                registers, pre_op_regs.rdx, post_op_regs.rdx
            );
            registers = format!(
                "{} rsp: {:x}-> {:x}",
                registers, pre_op_regs.rsp, post_op_regs.rsp
            );
            registers = format!(
                "{} rbp: {:x}-> {:x}",
                registers, pre_op_regs.rbp, post_op_regs.rbp
            );
            registers = format!(
                "{} rsi: {:x}-> {:x}",
                registers, pre_op_regs.rsi, post_op_regs.rsi
            );
            registers = format!(
                "{} rdi: {:x}-> {:x}",
                registers, pre_op_regs.rdi, post_op_regs.rdi
            );
            registers = format!(
                "{} r8: {:x}-> {:x}",
                registers, pre_op_regs.r8, post_op_regs.r8
            );
            registers = format!(
                "{} r9: {:x}-> {:x}",
                registers, pre_op_regs.r9, post_op_regs.r9
            );
            registers = format!(
                "{} r10: {:x}-> {:x}",
                registers, pre_op_regs.r10, post_op_regs.r10
            );
            registers = format!(
                "{} r11: {:x}-> {:x}",
                registers, pre_op_regs.r11, post_op_regs.r11
            );
            registers = format!(
                "{} r12: {:x}-> {:x}",
                registers, pre_op_regs.r12, post_op_regs.r12
            );
            registers = format!(
                "{} r13: {:x}-> {:x}",
                registers, pre_op_regs.r13, post_op_regs.r13
            );
            registers = format!(
                "{} r14: {:x}-> {:x}",
                registers, pre_op_regs.r14, post_op_regs.r14
            );
            registers = format!(
                "{} r15: {:x}-> {:x}",
                registers, pre_op_regs.r15, post_op_regs.r15
            );
        } else {
            let post_op_regs = post_op_regs;
            registers = Regs64::diff(pre_op_regs, post_op_regs);
        }

        // temp test, always dump all?
        /*{
            registers = format!(
                "{} rax: {:x}-> {:x}",
                registers, pre_op_regs.rax, post_op_regs.rax
            );
            registers = format!(
                "{} rbx: {:x}-> {:x}",
                registers, pre_op_regs.rbx, post_op_regs.rbx
            );
            registers = format!(
                "{} rcx: {:x}-> {:x}",
                registers, pre_op_regs.rcx, post_op_regs.rcx
            );
            registers = format!(
                "{} rdx: {:x}-> {:x}",
                registers, pre_op_regs.rdx, post_op_regs.rdx
            );
            registers = format!(
                "{} rsp: {:x}-> {:x}",
                registers, pre_op_regs.rsp, post_op_regs.rsp
            );
            registers = format!(
                "{} rbp: {:x}-> {:x}",
                registers, pre_op_regs.rbp, post_op_regs.rbp
            );
            registers = format!(
                "{} rsi: {:x}-> {:x}",
                registers, pre_op_regs.rsi, post_op_regs.rsi
            );
            registers = format!(
                "{} rdi: {:x}-> {:x}",
                registers, pre_op_regs.rdi, post_op_regs.rdi
            );
            registers = format!(
                "{} r8: {:x}-> {:x}",
                registers, pre_op_regs.r8, post_op_regs.r8
            );
            registers = format!(
                "{} r9: {:x}-> {:x}",
                registers, pre_op_regs.r9, post_op_regs.r9
            );
            registers = format!(
                "{} r10: {:x}-> {:x}",
                registers, pre_op_regs.r10, post_op_regs.r10
            );
            registers = format!(
                "{} r11: {:x}-> {:x}",
                registers, pre_op_regs.r11, post_op_regs.r11
            );
            registers = format!(
                "{} r12: {:x}-> {:x}",
                registers, pre_op_regs.r12, post_op_regs.r12
            );
            registers = format!(
                "{} r13: {:x}-> {:x}",
                registers, pre_op_regs.r13, post_op_regs.r13
            );
            registers = format!(
                "{} r14: {:x}-> {:x}",
                registers, pre_op_regs.r14, post_op_regs.r14
            );
            registers = format!(
                "{} r15: {:x}-> {:x}",
                registers, pre_op_regs.r15, post_op_regs.r15
            );
        }*/

        let mut flags = String::new();
        // dump all flags on first, only differences on next
        let pre_op_flags = self.pre_op_flags();
        let post_op_flags = self.post_op_flags();
        if index == 0 {
            flags = format!(
                "rflags: {:x}-> {:x}",
                pre_op_flags.dump(),
                post_op_flags.dump()
            );
        } else if pre_op_flags.dump() != post_op_flags.dump() {
            flags = format!(
                "rflags: {:x}-> {:x}",
                pre_op_flags.dump(),
                post_op_flags.dump()
            );
            comments = format!("{} {}", comments, Flags::diff(pre_op_flags, post_op_flags));
        }

        // dump all write memory operations
        let mut memory = String::new();
        for memory_op in self.memory_operations.iter() {
            if memory_op.op == "read" {
                continue;
            }
            memory = format!(
                "{} {:016X}: {:X}-> {:X}",
                memory, memory_op.address, memory_op.old_value, memory_op.new_value
            );
        }

        /*
        let mut trace_file = self.trace_file.as_ref().unwrap();
        writeln!(
            trace_file,
            r#""{index}","{address:016X}","{bytes:02x?}","{disassembly}","{registers}","{memory}","{comments}""#,
            index = index,
            address = pre_op_regs.rip,
            bytes = instruction_bytes,
            disassembly = output,
            registers = format!("{} {}", registers, flags),
            memory = memory,
            comments = comments
        ).expect("failed to write to trace file");*/

        log::info!(
            r#"trace: "{index}","{address:016X}","{bytes:02x?}","{disassembly}","{registers}","{memory}","{comments}""#,
            index = index + 1,
            address = pre_op_regs.rip,
            bytes = instruction_bytes,
            disassembly = output,
            registers = format!("{} {}", registers, flags),
            memory = memory,
            comments = comments
        );
    }

    /// display specific register.
    pub(crate) fn trace_specific_register(&self, reg: &str) {
        //TODO: I think this is already implemented in regs64
        match reg {
            "rax" => self.regs().show_rax(&self.maps, self.pos),
            "rbx" => self.regs().show_rbx(&self.maps, self.pos),
            "rcx" => self.regs().show_rcx(&self.maps, self.pos),
            "rdx" => self.regs().show_rdx(&self.maps, self.pos),
            "rsi" => self.regs().show_rsi(&self.maps, self.pos),
            "rdi" => self.regs().show_rdi(&self.maps, self.pos),
            "rbp" => log::info!("\t{} rbp: 0x{:x}", self.pos, self.regs().rbp),
            "rsp" => log::info!("\t{} rsp: 0x{:x}", self.pos, self.regs().rsp),
            "rip" => log::info!("\t{} rip: 0x{:x}", self.pos, self.regs().rip),
            "r8" => self.regs().show_r8(&self.maps, self.pos),
            "r9" => self.regs().show_r9(&self.maps, self.pos),
            "r10" => self.regs().show_r10(&self.maps, self.pos),
            "r10d" => self.regs().show_r10d(&self.maps, self.pos),
            "r11" => self.regs().show_r11(&self.maps, self.pos),
            "r11d" => self.regs().show_r11d(&self.maps, self.pos),
            "r12" => self.regs().show_r12(&self.maps, self.pos),
            "r13" => self.regs().show_r13(&self.maps, self.pos),
            "r14" => self.regs().show_r14(&self.maps, self.pos),
            "r15" => self.regs().show_r15(&self.maps, self.pos),
            "eax" => self.regs().show_eax(&self.maps, self.pos),
            "ebx" => self.regs().show_ebx(&self.maps, self.pos),
            "ecx" => self.regs().show_ecx(&self.maps, self.pos),
            "edx" => self.regs().show_edx(&self.maps, self.pos),
            "esi" => self.regs().show_esi(&self.maps, self.pos),
            "edi" => self.regs().show_edi(&self.maps, self.pos),
            "esp" => log::info!("\t{} esp: 0x{:x}", self.pos, self.regs().get_esp() as u32),
            "ebp" => log::info!("\t{} ebp: 0x{:x}", self.pos, self.regs().get_ebp() as u32),
            "eip" => log::info!("\t{} eip: 0x{:x}", self.pos, self.regs().get_eip() as u32),
            "xmm1" => log::info!("\t{} xmm1: 0x{:x}", self.pos, self.regs().xmm1),
            _ => panic!("invalid register."),
        }
    }

    pub(crate) fn trace_string(&mut self) {
        let s = self.maps.read_string(self.cfg.string_addr);

        if s.len() >= 2 && s.len() < 80 {
            log::info!(
                "\t{} trace string -> 0x{:x}: '{}'",
                self.pos,
                self.cfg.string_addr,
                s
            );
        } else {
            let w = self.maps.read_wide_string_nocrash(self.cfg.string_addr);
            if w.len() == 0 {
                return;
            }
            if w.len() < 80 {
                log::info!(
                    "\t{} trace wide string -> 0x{:x}: '{}'",
                    self.pos,
                    self.cfg.string_addr,
                    w
                );
            } else {

                //log::info!("\t{} trace wide string -> 0x{:x}: ''", self.pos, self.cfg.string_addr);
            }
        }
    }

    /// trace that inspects memory
    pub(crate) fn trace_memory_inspection(&mut self) {
        let addr: u64 = self.memory_operand_to_address(self.cfg.inspect_seq.clone().as_str());
        let bits = self.get_size(self.cfg.inspect_seq.clone().as_str());
        let value = self
            .memory_read(self.cfg.inspect_seq.clone().as_str())
            .unwrap_or(0);

        let mut s = self.maps.read_string(addr);
        self.maps.filter_string(&mut s);
        let bytes = self
            .maps
            .read_string_of_bytes(addr, constants::NUM_BYTES_TRACE);
        log::info!(
            "\tmem_inspect: rip = {:x} (0x{:x}): 0x{:x} {} '{}' {{{}}}",
            self.regs().rip,
            addr,
            value,
            value,
            s,
            bytes
        );
    }
}
