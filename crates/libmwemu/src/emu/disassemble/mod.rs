use crate::emu::Emu;
use iced_x86::{Decoder, DecoderOptions, Formatter as _, Instruction};
use serde::{Deserialize, Serialize};

// about 10 mb should be on l3 cache
// 8192 cache lines,
// 64 instructions for each one,
// 40 for the struct (I think we can make it smaller)
const INSTRUCTION_ARRAY_SIZE: usize = 8192 * 64;

// we want the cache size to be store in L1 cache or L2 cache which is lower than 40kb
const CACHE_SIZE: usize = 2048 * MAX_CACHE_PER_LINE;
const CACHE_MASK: usize = CACHE_SIZE - 1; // Assumes power of 2
const MAX_CACHE_PER_LINE: usize = 32;

// we need INVALID_KEY and INVALID_LEN to be the same as INVALID_LPF_ADDR to optimize for memset
pub const INVALID_LPF_ADDR: u64 = 0xffffffffffffffff;
pub const INVALID_KEY: usize = 0xffffffffffffffff;
pub const INVALID_LEN: usize = 0xffffffffffffffff;

pub fn LPF_OF(addr: u64) -> u64 {
    // Implementation of LPF_OF macro/function
    addr & 0xfffffffffffff000
}

#[derive(Clone, Serialize, Deserialize)]
struct CachedInstruction {
    pub lpf: u64,
    pub instruction_key: usize,
    pub instruction_len: usize,
}

impl Default for CachedInstruction {
    fn default() -> Self {
        CachedInstruction {
            lpf: INVALID_LPF_ADDR,
            instruction_key: INVALID_KEY,
            instruction_len: INVALID_LEN,
        }
    }
}

impl CachedInstruction {
    pub fn is_valid(&self) -> bool {
        self.lpf == INVALID_LPF_ADDR
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InstructionCache {
    cache_entries: Vec<CachedInstruction>,
    instructions: Vec<Instruction>,
    next_instruction_slot: usize,
    pub current_instruction_slot: usize,
    current_decode_len: usize,
    current_decode_idx: usize, // probe_stats: ProbeStats,
}

#[derive(Clone, Serialize, Deserialize, Default)]
struct ProbeStats {
    hits: usize,
    misses: usize,
    collisions: usize,
}

impl InstructionCache {
    pub fn new() -> Self {
        let mut cache = InstructionCache {
            cache_entries: vec![CachedInstruction::default(); CACHE_SIZE],
            instructions: vec![Instruction::default(); INSTRUCTION_ARRAY_SIZE],
            next_instruction_slot: 0,
            current_decode_len: 0,
            current_instruction_slot: 0,
            current_decode_idx: 0,
            // probe_stats: ProbeStats::default(),
        };

        // Initialize all instructions to default state
        for inst in &mut cache.instructions {
            *inst = Instruction::default();
        }

        cache
    }

    #[inline(always)]
    pub fn get_index_of(&self, lpf: u64, len: u64) -> usize {
        const TLB_MASK: u32 = ((CACHE_SIZE - 1) << 12) as u32;
        (((lpf + len) & (TLB_MASK as u64)) >> 12) as usize
    }

    #[inline]
    pub fn flush_cache_line(&mut self, idx: usize) {
        for i in 0..MAX_CACHE_PER_LINE {
            self.cache_entries[idx].lpf = INVALID_LPF_ADDR;
        }
    }

    pub fn lookup_entry(&mut self, addr: u64, len: u64) -> bool {
        let lpf = crate::maps::tlb::LPF_OF(addr);
        let idx = self.get_index_of(lpf, len);

        // do a linear probing for each cache line
        for i in 0..MAX_CACHE_PER_LINE {
            if self.cache_entries[idx + i].lpf == INVALID_LPF_ADDR {
                return false;
            }
            // found the instruction now do initialization and return true
            if self.cache_entries[idx + i].lpf == addr {
                let key = self.cache_entries[idx + i].instruction_key;
                self.current_instruction_slot = key;
                self.current_decode_len = self.cache_entries[idx + i].instruction_len;
                self.current_decode_idx = 0;
                return true;
            }
        }

        // the cache_line is full now we flush all the cache line
        self.flush_cache_line(idx);
        false
    }

    #[inline(always)]
    fn flush_cache(&mut self) {
        self.cache_entries.iter_mut().for_each(|entry| {
            entry.lpf = INVALID_LPF_ADDR;
            entry.instruction_key = INVALID_KEY;
            entry.instruction_len = INVALID_LEN;
        });
        self.next_instruction_slot = 0;
    }

    pub fn insert_from_decoder(&mut self, decoder: &mut Decoder, addition: usize, rip_addr: u64) {
        let lpf = crate::maps::tlb::LPF_OF(rip_addr);
        let idx = self.get_index_of(lpf, 0);

        // copy the instruction to the slot
        // now the case when instruction slot is full, instead of complex algorithm
        // we just fudge everything and rebuild from scratch can be a better way
        // but I think this is simple and good enough
        let slot = self.next_instruction_slot;

        let mut count: usize = 0;
        let max_position = decoder.max_position();
        if max_position + self.next_instruction_slot > INSTRUCTION_ARRAY_SIZE {
            self.flush_cache();
        }

        // we just need to decode until the  call or jump instruction but not the entire one
        while decoder.can_decode() && decoder.position() + addition <= max_position {
            decoder.decode_out(&mut self.instructions[slot + count]);
            let temp = self.instructions[slot + count];
            if temp.is_jmp_short_or_near()
                || temp.is_jmp_near_indirect()
                || temp.is_jmp_far()
                || temp.is_jmp_far_indirect()
                || temp.is_jcc_short_or_near()
                || temp.is_call_near_indirect()
                || temp.is_call_near()
                || temp.is_call_far_indirect()
                || temp.is_call_far()
            {
                count += 1;
                break;
            }
            count += 1;
        }
        self.next_instruction_slot += count;

        // insert to the cache
        for i in 0..MAX_CACHE_PER_LINE {
            if self.cache_entries[idx + i].lpf == INVALID_LPF_ADDR {
                self.cache_entries[idx + i].instruction_key = slot;
                self.cache_entries[idx + i].lpf = rip_addr;
                self.cache_entries[idx + i].instruction_len = count;
                break;
            }
        }

        assert!(self.lookup_entry(rip_addr, 0), "Cache Insertion FAILED: There is support to be entry after insertion using insert_from_decoder");
    }

    pub fn insert_instruction(&mut self, addr: u64, instrs: Vec<Instruction>) {
        let lpf = crate::maps::tlb::LPF_OF(addr);
        let idx = self.get_index_of(lpf, 0);

        // copy the instruction to the slot
        // now the case when instruction slot is full, instead of complex algorithm
        // we just fudge everything and rebuild from scratch can be a better way
        // but I think this is simple and good enough
        let slot = self.next_instruction_slot;
        self.next_instruction_slot += instrs.len();
        if self.next_instruction_slot >= INSTRUCTION_ARRAY_SIZE {
            self.flush_cache();
        }

        for i in 0..instrs.len() {
            self.instructions[slot + i] = instrs[i];
        }

        // insert to the cache
        for i in 0..MAX_CACHE_PER_LINE {
            if self.cache_entries[idx + i].lpf == INVALID_LPF_ADDR {
                self.cache_entries[idx + i].instruction_key = slot;
                self.cache_entries[idx + i].lpf = addr;
                self.cache_entries[idx + i].instruction_len = instrs.len();
                break;
            }
        }
    }

    pub fn decode_out(&mut self, instruction: &mut Instruction) {
        *instruction = self.instructions[self.current_instruction_slot + self.current_decode_idx];
        self.current_decode_idx += 1;
    }

    pub fn can_decode(&self) -> bool {
        self.current_decode_idx < self.current_decode_len
    }
}

impl Emu {
    /// Disassemble an amount of instruccions on an specified address.
    /// This not used on the emulation engine, just from console,
    /// but the api could be used programatilcally.
    pub fn disassemble(&mut self, addr: u64, amount: u32) -> String {
        let mut out = String::new();
        let code = self.maps.get_mem_by_addr(addr).expect("address not mapped");
        let block = code.read_from(addr);

        let bits: u32 = if self.cfg.is_64bits { 64 } else { 32 };
        let mut decoder = Decoder::with_ip(bits, block, addr, DecoderOptions::NONE);
        let mut output = String::new();
        let mut instruction = Instruction::default();
        let mut count: u32 = 1;
        while decoder.can_decode() {
            decoder.decode_out(&mut instruction);
            output.clear();
            self.formatter.format(&instruction, &mut output);
            if self.cfg.is_64bits {
                out.push_str(&format!("0x{:x}: {}\n", instruction.ip(), output));
                //log::info!("0x{:x}: {}", instruction.ip(), output);
            } else {
                out.push_str(&format!("0x{:x}: {}\n", instruction.ip32(), output));
                //log::info!("0x{:x}: {}", instruction.ip32(), output);
            }
            count += 1;
            if count == amount {
                break;
            }
        }
        out
    }
}
