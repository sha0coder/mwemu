use iced_x86::{Code, Instruction, Mnemonic};

use super::Emu;

impl Emu {
    pub(crate) fn handle_x86_rep_pre_execution(
        &mut self,
        instruction: Instruction,
        size: usize,
    ) -> bool {
        let is_ret = matches!(instruction.code(), Code::Retnw | Code::Retnd | Code::Retnq);
        let has_rep_prefix = instruction.has_rep_prefix()
            || instruction.has_repe_prefix()
            || instruction.has_repne_prefix();

        if is_ret || !has_rep_prefix {
            return false;
        }

        if self.rep.is_none() {
            self.rep = Some(0);
        }

        if self.regs().rcx == 0 {
            self.rep = None;
            if self.cfg.is_x64() {
                self.regs_mut().rip += size as u64;
            } else {
                let new_eip = self.regs().get_eip() + size as u64;
                self.regs_mut().set_eip(new_eip);
            }
            return true;
        }

        false
    }

    pub(crate) fn update_x86_rep_state_after_execution(&mut self, instruction: Instruction) {
        let Some(rep_count) = self.rep else {
            return;
        };

        if self.cfg.verbose >= 3 {
            log::trace!("    rcx: {}", self.regs().rcx);
        }

        if self.regs().rcx > 0 {
            self.regs_mut().rcx -= 1;
            if self.regs().rcx == 0 {
                self.rep = None;
            } else {
                self.rep = Some(rep_count + 1);
            }
        }

        let is_string_movement = matches!(
            instruction.mnemonic(),
            Mnemonic::Movsb
                | Mnemonic::Movsw
                | Mnemonic::Movsd
                | Mnemonic::Movsq
                | Mnemonic::Stosb
                | Mnemonic::Stosw
                | Mnemonic::Stosd
                | Mnemonic::Stosq
                | Mnemonic::Lodsb
                | Mnemonic::Lodsw
                | Mnemonic::Lodsd
                | Mnemonic::Lodsq
        );
        let is_string_comparison = matches!(
            instruction.mnemonic(),
            Mnemonic::Cmpsb
                | Mnemonic::Cmpsw
                | Mnemonic::Cmpsd
                | Mnemonic::Cmpsq
                | Mnemonic::Scasb
                | Mnemonic::Scasw
                | Mnemonic::Scasd
                | Mnemonic::Scasq
        );

        if is_string_movement {
            return;
        }

        if is_string_comparison {
            if instruction.has_repe_prefix() && !self.flags().f_zf {
                self.rep = None;
            }
            if instruction.has_repne_prefix() && self.flags().f_zf {
                self.rep = None;
            }
            return;
        }

        self.rep = None;
    }
}
