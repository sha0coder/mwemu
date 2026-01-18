// Shared test helpers for x86_64 instruction tests.
//
// This module provides common utilities for setting up test VMs
// and checking instruction behavior.

use crate::*;

/// Standard code address for tests
pub const CODE_ADDR: u64 = 0x1000;

/// Standard stack address for tests
pub const STACK_ADDR: u64 = 0x8000;

/// Standard data address for memory operand tests
pub const DATA_ADDR: u64 = 0x2000;

/// Default SYSCALL handler address for tests
pub const SYSCALL_HANDLER_ADDR: u64 = 0x12000;

/// Default interrupt handler address (simple IRETQ stub)
pub const INT_HANDLER_ADDR: u64 = 0x13000;

/// IDT base address
pub const IDT_BASE: u64 = 0x11000;

/// GDT base address
pub const GDT_BASE: u64 = 0x10000;


/// Create a test CPU with the given code.
/// This is a convenience wrapper for tests that prefer the TestCpu API.
/*
pub fn create_test_cpu(code: &[u8]) -> TestCpu {
    TestCpu::new(code)
}*/

/// Create a test CPU in compatibility mode for instructions invalid in 64-bit mode.
/*
pub fn create_test_cpu_compat(code: &[u8]) -> TestCpu {
    TestCpu::new_compat(code)
}*/

/// Run the test CPU until HLT.
/*
pub fn run_test(cpu: &mut TestCpu) {
    emu.run(None).unwrap();
    cpu.refresh_regs();
}*/

/// Stub TestCase type for tests that use hex string parsing.
/// These tests are placeholders that just check if code parses and runs.
pub struct TestCase {
    code: Vec<u8>,
}

impl TestCase {
    /// Parse a hex string like "66 0f 3a cf c1 00" into bytes
    pub fn from(hex_str: &str) -> Self {
        let code: Vec<u8> = hex_str
            .split_whitespace()
            .filter_map(|s| u8::from_str_radix(s, 16).ok())
            .collect();
        Self {
            code
        }
    }

    /// Run the test - just check if code executes without panic
    pub fn check(&self) {
        let mut code_with_hlt = self.code.clone();
        code_with_hlt.push(0xf4); // HLT
        let mut emu = emu64();
        emu.load_code_bytes(&code_with_hlt);
        emu.run(None).unwrap();
    }
}


