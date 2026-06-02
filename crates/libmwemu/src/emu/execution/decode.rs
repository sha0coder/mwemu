use iced_x86::{Decoder, DecoderOptions};

use crate::debug::console::Console;
use crate::err::MwemuError;
use crate::windows::constants;

use super::Emu;

impl Emu {
    pub(crate) fn fill_code_block(
        &mut self,
        pc: u64,
        block: &mut Vec<u8>,
    ) -> Result<(), MwemuError> {
        let code = match self.maps.get_mem_by_addr(pc) {
            Some(code) => code,
            None => {
                log::trace!("code flow to unmapped address 0x{:x}", pc);
                Console::spawn_console(self);
                return Err(MwemuError::new("cannot read program counter"));
            }
        };

        let block_slice = code.read_bytes(pc, constants::BLOCK_LEN);
        if block_slice.len() != block.len() {
            block.resize(block_slice.len(), 0);
        }
        block.clone_from_slice(block_slice);

        Ok(())
    }

    pub(crate) fn ensure_instruction_cache_populated(
        &mut self,
        pc: u64,
        block: &[u8],
        arch_bits: u32,
        is_aarch64: bool,
    ) -> Result<(), MwemuError> {
        let cache_hit = match &mut self.arch_state {
            super::ArchState::X86 {
                instruction_cache, ..
            } => instruction_cache.lookup_entry(pc, 0),
            super::ArchState::AArch64 {
                instruction_cache, ..
            } => instruction_cache.lookup_entry(pc, 0),
        };

        if cache_hit {
            return Ok(());
        }

        if !is_aarch64 {
            // Only abort when the readable portion of the code block is *entirely*
            // zero AND we reached the end of the underlying map. A long stretch of
            // leading zeros embedded between real instructions (alignment padding,
            // string-table gaps, …) is legitimate code — `00 00` decodes to
            // `add byte ptr [eax], al`, and the donut/SRDi loaders sometimes hit
            // those regions while iterating exports. Killing them on a 100-byte
            // run of zeros (the previous threshold) made forward progress
            // impossible past those gaps.
            let all_zeros = !block.is_empty() && block.iter().all(|&b| b == 0);
            let near_map_end = block.len() < constants::BLOCK_LEN;
            if !self.cfg.allow_empty_code_blocks && all_zeros && near_map_end {
                if self.cfg.verbose > 0 {
                    log::trace!(
                        "{} empty code block at 0x{:x} ({} zero bytes to map end)",
                        self.pos, pc, block.len(),
                    );
                }
                return Err(MwemuError::new("empty code block"));
            }
        }

        if block.is_empty() {
            return Err(MwemuError::new("cannot read code block, weird address."));
        }

        match &mut self.arch_state {
            super::ArchState::X86 {
                instruction_cache, ..
            } => {
                let mut decoder = Decoder::with_ip(arch_bits, block, pc, DecoderOptions::NONE);
                self.rep = None;
                let addition = block.len().min(16);
                instruction_cache.insert_from_decoder(&mut decoder, addition, pc);
            }
            super::ArchState::AArch64 {
                instruction_cache, ..
            } => {
                instruction_cache.insert_from_block(block, pc);
            }
        }

        Ok(())
    }

    pub(crate) fn instruction_cache_can_decode(&self) -> bool {
        match &self.arch_state {
            super::ArchState::X86 {
                instruction_cache, ..
            } => instruction_cache.can_decode(),
            super::ArchState::AArch64 {
                instruction_cache, ..
            } => instruction_cache.can_decode(),
        }
    }
}
