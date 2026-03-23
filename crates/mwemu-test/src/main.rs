use iced_x86::{Decoder, DecoderOptions, Instruction};
use libmwemu::{engine, emu64};
use libmwemu::maps::mem64::Permission;
use rayon::prelude::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

const CODE_ADDR: u64 = 0x4000000;
const STACK_ADDR: u64 = 0x1000000;
const SCRATCH_ADDR: u64 = 0x2000000;

#[derive(Debug, Clone)]
struct TestCase {
    inputs: HashMap<String, u128>,
    outputs: HashMap<String, u128>,
}

#[derive(Debug)]
struct InstructionTest {
    address: u64,
    bytes: Vec<u8>,
    assembly: String,
    test_cases: Vec<TestCase>,
}

fn parse_hex(s: &str) -> u128 {
    u128::from_str_radix(s.trim_start_matches('#'), 16).unwrap_or(0)
}

fn parse_register_value(s: &str) -> Option<(String, u128)> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return None;
    }
    let reg = parts[0].to_lowercase();
    let val = parse_hex(parts[1]);
    Some((reg, val))
}

fn parse_test_line(line: &str) -> Option<TestCase> {
    let line = line.trim();
    if !line.starts_with("in:") {
        return None;
    }

    let parts: Vec<&str> = line.splitn(2, "|out:").collect();
    if parts.len() != 2 {
        return None;
    }

    let in_part = parts[0].trim_start_matches("in:").trim();
    let out_part = parts[1].trim();

    let mut inputs = HashMap::new();
    let mut outputs = HashMap::new();

    for item in in_part.split(',') {
        if let Some((reg, val)) = parse_register_value(item.trim()) {
            inputs.insert(reg, val);
        }
    }

    for item in out_part.split(',') {
        if let Some((reg, val)) = parse_register_value(item.trim()) {
            outputs.insert(reg, val);
        }
    }

    Some(TestCase { inputs, outputs })
}

fn parse_instruction_line(line: &str) -> Option<(u64, Vec<u8>, String)> {
    if !line.starts_with("instr:") {
        return None;
    }

    let content = line.trim_start_matches("instr:");
    let parts: Vec<&str> = content.split(';').collect();
    if parts.len() < 3 {
        return None;
    }

    let address = u64::from_str_radix(parts[0].trim_start_matches("0x"), 16).unwrap_or(CODE_ADDR);
    let bytes_str = parts[1].trim_start_matches('#');
    let bytes: Vec<u8> = (0..bytes_str.len())
        .step_by(2)
        .filter_map(|i| u8::from_str_radix(&bytes_str[i..i + 2], 16).ok())
        .collect();
    let assembly = parts[2].to_string();

    Some((address, bytes, assembly))
}

fn parse_test_file(path: &Path) -> Vec<InstructionTest> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open {}: {}", path.display(), e);
            return vec![];
        }
    };

    let reader = BufReader::new(file);
    let mut tests = Vec::new();
    let mut current_test: Option<InstructionTest> = None;

    for line in reader.lines().flatten() {
        if let Some((addr, bytes, asm)) = parse_instruction_line(&line) {
            if let Some(test) = current_test.take() {
                if !test.test_cases.is_empty() {
                    tests.push(test);
                }
            }
            current_test = Some(InstructionTest {
                address: addr,
                bytes,
                assembly: asm,
                test_cases: Vec::new(),
            });
        } else if let Some(ref mut test) = current_test {
            if let Some(tc) = parse_test_line(&line) {
                test.test_cases.push(tc);
            }
        }
    }

    if let Some(test) = current_test {
        if !test.test_cases.is_empty() {
            tests.push(test);
        }
    }

    tests
}

// Test files store flags shifted by 24 bits in the format #XXNNNNNN where XX is the flags byte
fn decode_test_flags(value: u128) -> u32 {
    ((value >> 24) & 0xFF) as u32
}

fn encode_test_flags(flags: u32) -> u128 {
    // Mask out bit 1 (always 1 in real x86, but test files don't include it)
    (((flags & !0x02) & 0xFF) as u128) << 24
}

// Test files use big-endian encoding for 64-bit GPRs
fn decode_gpr64(value: u128) -> u64 {
    (value as u64).swap_bytes()
}

fn encode_gpr64(value: u64) -> u128 {
    value.swap_bytes() as u128
}

fn set_register(emu: &mut libmwemu::emu::Emu, name: &str, value: u128) {
    let regs = emu.regs_mut();
    match name {
        "rax" => regs.rax = decode_gpr64(value),
        "rbx" => regs.rbx = decode_gpr64(value),
        "rcx" => regs.rcx = decode_gpr64(value),
        "rdx" => regs.rdx = decode_gpr64(value),
        "rsi" => regs.rsi = decode_gpr64(value),
        "rdi" => regs.rdi = decode_gpr64(value),
        "rbp" => regs.rbp = decode_gpr64(value),
        "rsp" => regs.rsp = decode_gpr64(value),
        "r8" => regs.r8 = decode_gpr64(value),
        "r9" => regs.r9 = decode_gpr64(value),
        "r10" => regs.r10 = decode_gpr64(value),
        "r11" => regs.r11 = decode_gpr64(value),
        "r12" => regs.r12 = decode_gpr64(value),
        "r13" => regs.r13 = decode_gpr64(value),
        "r14" => regs.r14 = decode_gpr64(value),
        "r15" => regs.r15 = decode_gpr64(value),
        "xmm0" => regs.xmm0 = value,
        "xmm1" => regs.xmm1 = value,
        "xmm2" => regs.xmm2 = value,
        "xmm3" => regs.xmm3 = value,
        "xmm4" => regs.xmm4 = value,
        "xmm5" => regs.xmm5 = value,
        "xmm6" => regs.xmm6 = value,
        "xmm7" => regs.xmm7 = value,
        "xmm8" => regs.xmm8 = value,
        "xmm9" => regs.xmm9 = value,
        "xmm10" => regs.xmm10 = value,
        "xmm11" => regs.xmm11 = value,
        "xmm12" => regs.xmm12 = value,
        "xmm13" => regs.xmm13 = value,
        "xmm14" => regs.xmm14 = value,
        "xmm15" => regs.xmm15 = value,
        "flags" => emu.flags_mut().load(decode_test_flags(value)),
        _ => {}
    }
}

fn get_register(emu: &libmwemu::emu::Emu, name: &str) -> u128 {
    let regs = emu.regs();
    match name {
        "rax" => encode_gpr64(regs.rax),
        "rbx" => encode_gpr64(regs.rbx),
        "rcx" => encode_gpr64(regs.rcx),
        "rdx" => encode_gpr64(regs.rdx),
        "rsi" => encode_gpr64(regs.rsi),
        "rdi" => encode_gpr64(regs.rdi),
        "rbp" => encode_gpr64(regs.rbp),
        "rsp" => encode_gpr64(regs.rsp),
        "r8" => encode_gpr64(regs.r8),
        "r9" => encode_gpr64(regs.r9),
        "r10" => encode_gpr64(regs.r10),
        "r11" => encode_gpr64(regs.r11),
        "r12" => encode_gpr64(regs.r12),
        "r13" => encode_gpr64(regs.r13),
        "r14" => encode_gpr64(regs.r14),
        "r15" => encode_gpr64(regs.r15),
        "xmm0" => regs.xmm0,
        "xmm1" => regs.xmm1,
        "xmm2" => regs.xmm2,
        "xmm3" => regs.xmm3,
        "xmm4" => regs.xmm4,
        "xmm5" => regs.xmm5,
        "xmm6" => regs.xmm6,
        "xmm7" => regs.xmm7,
        "xmm8" => regs.xmm8,
        "xmm9" => regs.xmm9,
        "xmm10" => regs.xmm10,
        "xmm11" => regs.xmm11,
        "xmm12" => regs.xmm12,
        "xmm13" => regs.xmm13,
        "xmm14" => regs.xmm14,
        "xmm15" => regs.xmm15,
        "flags" => encode_test_flags(emu.flags().dump()),
        _ => 0,
    }
}

fn run_single_test(
    instr_test: &InstructionTest,
    test_case: &TestCase,
    test_idx: usize,
) -> Result<(), String> {
    let mut emu = emu64();

    // Create memory maps
    emu.maps
        .create_map("code", CODE_ADDR, 0x1000, Permission::READ_WRITE_EXECUTE)
        .map_err(|e| format!("Failed to create code map: {:?}", e))?;
    emu.maps
        .create_map("stack", STACK_ADDR, 0x10000, Permission::READ_WRITE)
        .map_err(|e| format!("Failed to create stack map: {:?}", e))?;
    emu.maps
        .create_map("scratch", SCRATCH_ADDR, 0x10000, Permission::READ_WRITE)
        .map_err(|e| format!("Failed to create scratch map: {:?}", e))?;

    // Write instruction bytes
    emu.maps.write_bytes(CODE_ADDR, &instr_test.bytes.clone());

    // Set up initial state
    emu.regs_mut().rip = CODE_ADDR;
    emu.regs_mut().rsp = STACK_ADDR + 0x8000;

    // Set input registers
    for (reg, val) in &test_case.inputs {
        set_register(&mut emu, reg, *val);
    }

    // Decode the instruction
    let mut decoder = Decoder::with_ip(64, &instr_test.bytes, CODE_ADDR, DecoderOptions::NONE);
    let mut ins = Instruction::default();
    decoder.decode_out(&mut ins);

    if ins.is_invalid() {
        return Err(format!("Failed to decode instruction: {}", instr_test.assembly));
    }

    // Execute the instruction
    let result = engine::emulate_instruction(&mut emu, &ins, instr_test.bytes.len(), false);
    if !result {
        return Err(format!(
            "Instruction execution failed: {}",
            instr_test.assembly
        ));
    }

    // Check output registers
    for (reg, expected) in &test_case.outputs {
        let actual = get_register(&emu, reg);
        if actual != *expected {
            return Err(format!(
                "Test #{}: {} mismatch for {}: expected 0x{:X}, got 0x{:X}\n  Inputs: {:?}",
                test_idx, instr_test.assembly, reg, expected, actual, test_case.inputs
            ));
        }
    }

    Ok(())
}

fn run_instruction_tests(instr_test: &InstructionTest) -> (usize, usize, Vec<String>) {
    let mut passed = 0;
    let mut failed = 0;
    let mut errors = Vec::new();

    for (idx, test_case) in instr_test.test_cases.iter().enumerate() {
        match run_single_test(instr_test, test_case, idx) {
            Ok(()) => passed += 1,
            Err(e) => {
                failed += 1;
                if errors.len() < 5 {
                    errors.push(e);
                }
            }
        }
    }

    (passed, failed, errors)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <test_directory> [instruction_filter]", args[0]);
        eprintln!("Example: {} '/Users/duncan/Downloads/Threadripper PRO 3975WX' adc", args[0]);
        std::process::exit(1);
    }

    let test_dir = Path::new(&args[1]);
    let filter = args.get(2).map(|s| s.to_lowercase());

    if !test_dir.is_dir() {
        eprintln!("Error: {} is not a directory", test_dir.display());
        std::process::exit(1);
    }

    let files: Vec<_> = std::fs::read_dir(test_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name().to_string_lossy().to_lowercase();
            name.ends_with(".txt")
                && filter
                    .as_ref()
                    .map(|f| name.starts_with(f))
                    .unwrap_or(true)
        })
        .collect();

    println!("Found {} test files to process", files.len());

    let total_passed = AtomicUsize::new(0);
    let total_failed = AtomicUsize::new(0);
    let total_tests = AtomicUsize::new(0);

    files.par_iter().for_each(|entry| {
        let path = entry.path();
        let filename = path.file_name().unwrap().to_string_lossy();

        println!("Processing: {}", filename);

        let tests = parse_test_file(&path);
        let mut file_passed = 0;
        let mut file_failed = 0;
        let mut file_errors = Vec::new();

        for instr_test in &tests {
            let (p, f, e) = run_instruction_tests(instr_test);
            file_passed += p;
            file_failed += f;
            file_errors.extend(e);
        }

        total_passed.fetch_add(file_passed, Ordering::Relaxed);
        total_failed.fetch_add(file_failed, Ordering::Relaxed);
        total_tests.fetch_add(file_passed + file_failed, Ordering::Relaxed);

        let status = if file_failed == 0 { "PASS" } else { "FAIL" };
        println!(
            "  {}: {} passed, {} failed (from {} instruction variants)",
            status, file_passed, file_failed, tests.len()
        );

        for err in file_errors.iter().take(3) {
            println!("    ERROR: {}", err);
        }
    });

    println!("\n=== SUMMARY ===");
    println!("Total tests: {}", total_tests.load(Ordering::Relaxed));
    println!("Passed: {}", total_passed.load(Ordering::Relaxed));
    println!("Failed: {}", total_failed.load(Ordering::Relaxed));

    let pass_rate = if total_tests.load(Ordering::Relaxed) > 0 {
        (total_passed.load(Ordering::Relaxed) as f64 / total_tests.load(Ordering::Relaxed) as f64)
            * 100.0
    } else {
        0.0
    };
    println!("Pass rate: {:.2}%", pass_rate);
}
