use crate::maps::mem64::Mem64;

// Partial _HEAP structure for Windows 10 x86 (32-bit).
// Mirrors heap64.rs but with 32-bit offsets and pointer widths.
// Only the fields observed being read by ntdll!RtlAllocateHeap and
// anti-emulation checks are modeled here.

pub struct Heap32 {
    // +0x000  Entry (HEAP_ENTRY, 8 bytes)
    pub entry: [u8; 8],
    // +0x008  SegmentSignature
    pub segment_signature: u32,
    // +0x00C  SegmentFlags
    pub segment_flags: u32,
    // +0x010  gap until +0x040
    pub pad_010: [u8; 0x040 - 0x10],
    // +0x040  Flags
    pub flags: u32,
    // +0x044  ForceFlags
    pub force_flags: u32,
    // +0x048  gap until +0x04C
    pub pad_048: [u8; 0x04C - 0x48],
    // +0x04C  VirtualMemoryThreshold
    pub virtual_memory_threshold: u32,
    // +0x050  gap until +0x0A4
    pub pad_050: [u8; 0x0A4 - 0x50],
    // +0x0A4  SegmentList (LIST_ENTRY: Flink, Blink — 4 bytes each)
    pub segment_list_flink: u32,
    pub segment_list_blink: u32,
    // +0x0AC  gap until +0x150
    pub pad_0ac: [u8; 0x150 - 0xAC],
    // +0x150  BlocksIndex region (~0x10 bytes)
    pub blocks_index: [u8; 0x10],
    // +0x160  gap until +0x1AC
    pub pad_160: [u8; 0x1AC - 0x160],
    // +0x1AC  FreeLists (LIST_ENTRY)
    pub free_lists_flink: u32,
    pub free_lists_blink: u32,
    // +0x1B4  gap until +0x24C
    pub pad_1b4: [u8; 0x24C - 0x1B4],
    // +0x24C  LockVariable pointer
    pub lock_variable_ptr: u32,
}

impl Heap32 {
    pub fn new(base_addr: u32) -> Self {
        let mut h = Heap32 {
            entry: [0; 8],
            segment_signature: 0xDDEE_DDEE,
            segment_flags: 0,
            pad_010: [0; 0x040 - 0x10],
            flags: 0,
            force_flags: 0,
            pad_048: [0; 0x04C - 0x48],
            virtual_memory_threshold: 0xFE00,
            pad_050: [0; 0x0A4 - 0x50],
            segment_list_flink: base_addr.wrapping_add(0x0A4),
            segment_list_blink: base_addr.wrapping_add(0x0A4),
            pad_0ac: [0; 0x150 - 0xAC],
            blocks_index: [0; 0x10],
            pad_160: [0; 0x1AC - 0x160],
            free_lists_flink: base_addr.wrapping_add(0x1AC),
            free_lists_blink: base_addr.wrapping_add(0x1AC),
            pad_1b4: [0; 0x24C - 0x1B4],
            lock_variable_ptr: base_addr.wrapping_add(0x300),
        };
        h.blocks_index[0] = 0x80;
        h
    }

    pub fn save(&self, mem: &mut Mem64) {
        let base = mem.get_base();
        mem.write_bytes(base, &self.entry);
        mem.write_dword(base + 0x008, self.segment_signature);
        mem.write_dword(base + 0x00C, self.segment_flags);
        mem.write_dword(base + 0x040, self.flags);
        mem.write_dword(base + 0x044, self.force_flags);
        mem.write_dword(base + 0x04C, self.virtual_memory_threshold);
        mem.write_dword(base + 0x0A4, self.segment_list_flink);
        mem.write_dword(base + 0x0A8, self.segment_list_blink);
        for (i, &b) in self.blocks_index.iter().enumerate() {
            mem.write_byte(base + 0x150 + i as u64, b);
        }
        mem.write_dword(base + 0x1AC, self.free_lists_flink);
        mem.write_dword(base + 0x1B0, self.free_lists_blink);
        mem.write_dword(base + 0x24C, self.lock_variable_ptr);
    }
}
