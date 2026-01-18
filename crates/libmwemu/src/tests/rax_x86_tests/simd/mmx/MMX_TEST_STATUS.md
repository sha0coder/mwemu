# MMX Test Coverage Status

## Completed Test Files (Comprehensive 30-50+ tests each)

### 1. psubsb_psubsw_mmx.rs ✓
**Lines**: 528 lines
**Tests**: 50+ comprehensive test cases
- PSUBSB (Subtract Packed Signed Bytes with Saturation)
- PSUBSW (Subtract Packed Signed Words with Saturation)
- Opcodes: 0F E8 /r, 0F E9 /r
- Coverage includes:
  - Basic subtraction
  - Positive/negative saturation
  - Mixed signs
  - Zero differences
  - Max/min values
  - Alternating patterns
  - Memory operands
  - Boundary cases
  - Sequential operations
  - Register reuse

### 2. psubusb_psubusw_mmx.rs ✓
**Lines**: 464 lines
**Tests**: 40+ comprehensive test cases
- PSUBUSB (Subtract Packed Unsigned Bytes with Saturation)
- PSUBUSW (Subtract Packed Unsigned Words with Saturation)
- Opcodes: 0F D8 /r, 0F D9 /r
- Coverage includes:
  - Basic subtraction
  - Saturation to zero
  - No saturation cases
  - Zero differences
  - Max values
  - Mixed saturation
  - Alternating patterns
  - Memory operands
  - Sequential operations
  - Self-subtract

### 3. pmaxsw_mmx.rs ✓
**Lines**: 192 lines
**Tests**: 30+ comprehensive test cases
- PMAXSW (Maximum of Packed Signed Word Integers)
- Opcode: 0F EE /r
- Coverage includes:
  - Basic maximum
  - All positive/negative
  - Mixed signs
  - Identical values
  - Max/min values vs zero
  - Memory operands
  - Alternating patterns
  - Sequential operations
  - Self-maximum

## Files Created with Placeholders (Need Implementation)

The following files have been created and registered in mod.rs but need test implementation:

### 4. pmaxub_mmx.rs (EMPTY - NEEDS TESTS)
- PMAXUB (Maximum of Packed Unsigned Byte Integers)
- Opcode: 0F DE /r
- Reference: /Users/int/dev/rax/docs/pmaxub:pmaxuw.txt

### 5. pminub_mmx.rs (EMPTY - NEEDS TESTS)
- PMINUB (Minimum of Packed Unsigned Byte Integers)
- Opcode: 0F DA /r
- Reference: /Users/int/dev/rax/docs/pminub:pminuw.txt

### 6. pminsw_mmx.rs (EMPTY - NEEDS TESTS)
- PMINSW (Minimum of Packed Signed Word Integers)
- Opcode: 0F EA /r
- Reference: /Users/int/dev/rax/docs/pminsb:pminsw.txt

### 7. pavgb_pavgw_mmx.rs (EMPTY - NEEDS TESTS)
- PAVGB (Average Packed Unsigned Byte Integers)
- PAVGW (Average Packed Unsigned Word Integers)
- Opcodes: 0F E0 /r, 0F E3 /r
- Reference: /Users/int/dev/rax/docs/pavgb:pavgw.txt
- Note: Average formula is (a + b + 1) >> 1 (rounds up)

### 8. pextrw_mmx.rs (EMPTY - NEEDS TESTS)
- PEXTRW (Extract Word)
- Opcode: 0F C5 /r ib
- Reference: /Users/int/dev/rax/docs/pextrw.txt
- Note: Extracts word from MMX register to general-purpose register

### 9. pinsrw_mmx.rs (EMPTY - NEEDS TESTS)
- PINSRW (Insert Word)
- Opcode: 0F C4 /r ib
- Reference: /Users/int/dev/rax/docs/pinsrw.txt
- Note: Inserts word from general-purpose register or memory to MMX register

### 10. psllw_pslld_psllq_mmx.rs (EMPTY - NEEDS TESTS)
- PSLLW (Shift Packed Words Left Logical)
- PSLLD (Shift Packed Doublewords Left Logical)
- PSLLQ (Shift Packed Quadword Left Logical)
- Opcodes:
  - Register: 0F F1 /r, 0F F2 /r, 0F F3 /r
  - Immediate: 0F 71 /6 ib, 0F 72 /6 ib, 0F 73 /6 ib
- Reference: /Users/int/dev/rax/docs/psllw:pslld:psllq.txt

### 11. psrlw_psrld_psrlq_mmx.rs (EMPTY - NEEDS TESTS)
- PSRLW (Shift Packed Words Right Logical)
- PSRLD (Shift Packed Doublewords Right Logical)
- PSRLQ (Shift Packed Quadword Right Logical)
- Opcodes:
  - Register: 0F D1 /r, 0F D2 /r, 0F D3 /r
  - Immediate: 0F 71 /2 ib, 0F 72 /2 ib, 0F 73 /2 ib
- Reference: /Users/int/dev/rax/docs/psrlw:psrld:psrlq.txt

### 12. psraw_psrad_mmx.rs (EMPTY - NEEDS TESTS)
- PSRAW (Shift Packed Words Right Arithmetic)
- PSRAD (Shift Packed Doublewords Right Arithmetic)
- Opcodes:
  - Register: 0F E1 /r, 0F E2 /r
  - Immediate: 0F 71 /4 ib, 0F 72 /4 ib
- Reference: /Users/int/dev/rax/docs/psraw:psrad:psraq.txt

### 13. pcmpgtb_pcmpgtw_pcmpgtd_mmx.rs (EMPTY - NEEDS TESTS)
- PCMPGTB (Compare Packed Signed Bytes for Greater Than)
- PCMPGTW (Compare Packed Signed Words for Greater Than)
- PCMPGTD (Compare Packed Signed Doublewords for Greater Than)
- Opcodes: 0F 64 /r, 0F 65 /r, 0F 66 /r
- Reference: /Users/int/dev/rax/docs/pcmpgtb:pcmpgtw:pcmpgtd.txt
- Note: Result is all 1s (0xFF..) if true, all 0s if false

### 14. pmaddwd_mmx.rs (EMPTY - NEEDS TESTS)
- PMADDWD (Multiply and Add Packed Integers)
- Opcode: 0F F5 /r
- Reference: /Users/int/dev/rax/docs/pmaddwd.txt
- Note: Multiplies packed signed words and adds adjacent results

### 15. pmulhuw_mmx.rs (EMPTY - NEEDS TESTS)
- PMULHUW (Multiply Packed Unsigned Integers and Store High Result)
- Opcode: 0F E4 /r
- Reference: /Users/int/dev/rax/docs/pmulhuw.txt
- Note: Returns high 16 bits of unsigned 16x16 multiplication

### 16. psadbw_mmx.rs (EMPTY - NEEDS TESTS)
- PSADBW (Compute Sum of Absolute Differences)
- Opcode: 0F F6 /r
- Reference: /Users/int/dev/rax/docs/psadbw.txt
- Note: Computes sum of absolute differences of unsigned bytes

## mod.rs Status ✓

The /Users/int/dev/rax/tests/x86_64/simd/mmx/mod.rs file has been updated to include all new module declarations:

```rust
mod pavgb_pavgw_mmx;
mod pcmpgtb_pcmpgtw_pcmpgtd_mmx;
mod pextrw_mmx;
mod pinsrw_mmx;
mod pmaddwd_mmx;
mod pmaxsw_mmx;
mod pmaxub_mmx;
mod pminsw_mmx;
mod pminub_mmx;
mod pmulhuw_mmx;
mod psadbw_mmx;
mod psllw_pslld_psllq_mmx;
mod psraw_psrad_mmx;
mod psrlw_psrld_psrlq_mmx;
mod psubsb_psubsw_mmx;
mod psubusb_psubusw_mmx;
```

## Test Implementation Guidelines

For each empty test file, implement 30-50+ comprehensive test cases following this pattern:

### Standard Test Categories:
1. **Basic Operations** - Simple, straightforward cases
2. **Boundary Cases** - Min/max values, edge conditions
3. **Saturation** - Test saturation behavior (where applicable)
4. **Mixed Signs** - Positive and negative values (for signed operations)
5. **Zero Cases** - Operations with zero
6. **Identical Values** - Same source and destination
7. **Alternating Patterns** - Test with alternating bit patterns (0xAA, 0x55, etc.)
8. **Memory Operands** - Test with memory source operands
9. **Sequential Operations** - Chain multiple operations
10. **Self-Operations** - Same register as source and destination

### Test Structure Template:

```rust
//! Tests for INSTRUCTION_NAME instruction (MMX).
//!
//! INSTRUCTION_NAME - Full Description
//!
//! Detailed explanation of what the instruction does.
//!
//! Opcode: XX XX /r
//!
//! Flags affected: None
//!
//! Reference: /Users/int/dev/rax/docs/filename.txt

#[path = "../../common/mod.rs"]
mod common;

use common::*;

fn write_mm_via_mem(mem: &vm_memory::GuestMemoryMmap, addr: u64, value: u64) {
    write_mem_at_u64(mem, addr, value);
}

#[test]
fn test_instruction_basic() {
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM1, [0x2008]
        0x0f, 0xXX, 0xc1,                               // INSTRUCTION MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM0
        0xf4,
    ];

    let (mut vcpu, mem) = setup_vm(&code, None);

    write_mm_via_mem(&mem, 0x2000, 0x0000000000000000);
    write_mm_via_mem(&mem, 0x2008, 0x0000000000000000);

    run_until_hlt(&mut vcpu).unwrap();

    let result = read_mem_at_u64(&mem, 0x2010);
    assert_eq!(result, 0x0000000000000000, "INSTRUCTION: description");
}
```

## Summary Statistics

- **Total MMX Test Files**: 29 files
- **Previously Existing**: 16 files
- **Newly Created (Comprehensive)**: 3 files (psubsb_psubsw_mmx.rs, psubusb_psubusw_mmx.rs, pmaxsw_mmx.rs)
- **Newly Created (Placeholders)**: 13 files
- **Total Tests in New Files**: 120+ tests
- **Estimated Total Tests Needed**: ~450 tests (30-50 per remaining file)

## Next Steps

1. Implement comprehensive tests for all placeholder files
2. Ensure each file has 30-50+ test cases
3. Test all boundary conditions and edge cases
4. Verify correct opcode encoding for each instruction
5. Cross-reference with Intel documentation for correctness
6. Add tests for both register and memory operands
7. Test sequential and chained operations
8. Validate saturation behavior where applicable

## Files by Priority

### High Priority (Core MMX Operations):
1. psllw_pslld_psllq_mmx.rs - Shift operations are fundamental
2. psrlw_psrld_psrlq_mmx.rs - Shift operations are fundamental
3. psraw_psrad_mmx.rs - Arithmetic shifts
4. pcmpgtb_pcmpgtw_pcmpgtd_mmx.rs - Comparison operations
5. pavgb_pavgw_mmx.rs - Average operations (used in multimedia)

### Medium Priority:
6. pmaxub_mmx.rs - Min/max operations
7. pminsw_mmx.rs - Min/max operations
8. pminub_mmx.rs - Min/max operations
9. pmaddwd_mmx.rs - Multiply-add (important for DSP)
10. pmulhuw_mmx.rs - High multiplication result

### Lower Priority (Specialized):
11. psadbw_mmx.rs - Sum of absolute differences (video coding)
12. pextrw_mmx.rs - Extract word
13. pinsrw_mmx.rs - Insert word
