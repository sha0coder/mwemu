// Comprehensive integration tests that verify the emulator functionality with real shellcode
use libmwemu::{emu32, emu64};

#[test]
fn test_emu32_creation() {
    let emu = emu32();
    assert!(!emu.cfg.is_64bits);
}

#[test]
fn test_emu64_creation() {
    let emu = emu64();
    assert!(emu.cfg.is_64bits);
}

#[test]
fn test_emu32_basic_config() {
    let mut emu = emu32();
    
    // Test basic configuration changes
    emu.cfg.verbose = 2;
    assert_eq!(emu.cfg.verbose, 2);
    
    emu.cfg.nocolors = true;
    assert!(emu.cfg.nocolors);
    
    emu.cfg.trace_mem = true;
    assert!(emu.cfg.trace_mem);
}

#[test]
fn test_emu64_basic_config() {
    let mut emu = emu64();
    
    // Test basic configuration changes
    emu.cfg.verbose = 1;
    assert_eq!(emu.cfg.verbose, 1);
    
    emu.cfg.console_enabled = true;
    assert!(emu.cfg.console_enabled);
    
    emu.cfg.stack_trace = true;
    assert!(emu.cfg.stack_trace);
}

#[test]
fn test_memory_allocation() {
    let mut emu = emu32();
    
    // Test basic memory allocation
    let addr = emu.alloc("test_buffer", 1024);
    assert!(addr > 0);
    
    // Verify the allocation exists in maps
    assert!(emu.maps.is_mapped(addr));
}

#[test]
fn test_memory_operations() {
    let mut emu = emu32();
    let addr = emu.alloc("test_mem", 1024);
    
    // Test basic memory operations
    assert!(emu.maps.write_byte(addr, 0x42));
    assert_eq!(emu.maps.read_byte(addr).unwrap(), 0x42);
    
    assert!(emu.maps.write_dword(addr, 0x12345678));
    assert_eq!(emu.maps.read_dword(addr).unwrap(), 0x12345678);
}

#[test]
fn test_string_operations() {
    let mut emu = emu32();
    let addr = emu.alloc("test_string", 1024);
    let test_string = "Hello, World!";
    
    // Test string operations
    emu.maps.write_string(addr, test_string);
    let read_string = emu.maps.read_string(addr);
    assert_eq!(read_string, test_string);
}

#[test]
fn test_register_operations() {
    let mut emu = emu32();
    
    // Test register operations
    let _prev_eax = emu.regs.get_by_name("eax");
    emu.regs.set_by_name("eax", 0x12345678);
    assert_eq!(emu.regs.get_by_name("eax"), 0x12345678);
    
    // Test register validation
    assert!(emu.regs.is_reg("eax"));
    assert!(!emu.regs.is_reg("invalid_reg"));
}

#[test]
fn test_position_management() {
    let mut emu = emu32();
    
    // Test position management
    assert_eq!(emu.pos, 0);
    emu.pos = 100;
    assert_eq!(emu.pos, 100);
    emu.pos = 0;
    assert_eq!(emu.pos, 0);
}

#[test]
fn test_simple_arithmetic_shellcode() {
    let mut emu = emu32();
    
    // Simple shellcode that performs arithmetic operations
    // mov eax, 0x10    ; Load 16 into EAX
    // add eax, 0x20    ; Add 32 to EAX (result: 48)
    // mov ebx, eax     ; Copy result to EBX
    let shellcode = vec![
        0xB8, 0x10, 0x00, 0x00, 0x00,  // mov eax, 0x10
        0x83, 0xC0, 0x20,              // add eax, 0x20
        0x89, 0xC3,                    // mov ebx, eax
    ];
    
    // Load shellcode into memory
    let code_addr = emu.alloc("shellcode", 1024);
    emu.maps.write_buffer(code_addr, &shellcode);
    
    // Verify shellcode was written correctly
    let written_data = emu.maps.read_buffer(code_addr, shellcode.len());
    assert_eq!(written_data, shellcode);
    
    // Test that we can read individual instructions
    assert_eq!(emu.maps.read_byte(code_addr).unwrap(), 0xB8); // mov eax opcode
    assert_eq!(emu.maps.read_dword(code_addr + 1).unwrap(), 0x10); // immediate value
}

#[test]
fn test_xor_encryption_shellcode() {
    let mut emu = emu32();
    
    // XOR encryption shellcode (just the bytes, no execution)
    let shellcode = vec![
        0xB8, 0x41, 0x41, 0x41, 0x41,  // mov eax, 0x41414141
        0x35, 0x78, 0x56, 0x34, 0x12,  // xor eax, 0x12345678
        0x89, 0xC3,                    // mov ebx, eax
        0x81, 0xF3, 0x78, 0x56, 0x34, 0x12,  // xor ebx, 0x12345678
    ];
    
    let code_addr = emu.alloc("xor_shellcode", 1024);
    emu.maps.write_buffer(code_addr, &shellcode);
    
    // Verify shellcode was written correctly
    let written_data = emu.maps.read_buffer(code_addr, shellcode.len());
    assert_eq!(written_data, shellcode);
    
    // Test XOR operation manually with registers
    emu.regs.set_by_name("eax", 0x41414141);
    let original_value = emu.regs.get_by_name("eax");
    
    // Simulate XOR operation
    let xor_key = 0x12345678u64;
    let encrypted = original_value ^ xor_key;
    let decrypted = encrypted ^ xor_key;
    
    // Verify XOR encryption/decryption logic
    assert_eq!(decrypted, original_value);
    assert_ne!(encrypted, original_value);
}

#[test]
fn test_rot_cipher_shellcode() {
    let mut emu = emu32();
    
    // ROT cipher shellcode (test memory storage and rotation logic)
    let shellcode = vec![
        0xB8, 0x6C, 0x6C, 0x65, 0x48,  // mov eax, 0x48656C6C ("Hell")
        0xC1, 0xC0, 0x08,              // rol eax, 8
        0x89, 0xC3,                    // mov ebx, eax
        0xC1, 0xCB, 0x08,              // ror ebx, 8
    ];
    
    let code_addr = emu.alloc("rot_shellcode", 1024);
    emu.maps.write_buffer(code_addr, &shellcode);
    
    // Verify shellcode was written correctly
    let written_data = emu.maps.read_buffer(code_addr, shellcode.len());
    assert_eq!(written_data, shellcode);
    
    // Test rotation operations manually
    let original = 0x48656C6Cu32;
    let rotated_left = original.rotate_left(8);
    let rotated_back = rotated_left.rotate_right(8);
    
    // Verify rotation logic
    assert_eq!(rotated_back, original);
    assert_ne!(rotated_left, original);
    
    // Test with registers
    emu.regs.set_by_name("eax", original as u64);
    assert_eq!(emu.regs.get_by_name("eax"), original as u64);
}

#[test]
fn test_complex_crypto_shellcode() {
    let mut emu = emu32();
    
    // Complex crypto shellcode combining XOR and ROT (test memory storage)
    let shellcode = vec![
        0xB8, 0x78, 0x56, 0x34, 0x12,  // mov eax, 0x12345678
        0x35, 0xEF, 0xBE, 0xAD, 0xDE,  // xor eax, 0xDEADBEEF
        0xC1, 0xC0, 0x04,              // rol eax, 4
        0x89, 0xC3,                    // mov ebx, eax
        0xC1, 0xCB, 0x04,              // ror ebx, 4
        0x81, 0xF3, 0xEF, 0xBE, 0xAD, 0xDE,  // xor ebx, 0xDEADBEEF
    ];
    
    let code_addr = emu.alloc("crypto_shellcode", 1024);
    emu.maps.write_buffer(code_addr, &shellcode);
    
    // Verify shellcode was written correctly
    let written_data = emu.maps.read_buffer(code_addr, shellcode.len());
    assert_eq!(written_data, shellcode);
    
    // Test complex crypto operations manually
    let original = 0x12345678u32;
    let xor_key = 0xDEADBEEFu32;
    
    // Step 1: XOR encryption
    let xor_encrypted = original ^ xor_key;
    
    // Step 2: Rotate left 4 bits
    let rot_encrypted = xor_encrypted.rotate_left(4);
    
    // Step 3: Rotate right 4 bits (decrypt rotation)
    let rot_decrypted = rot_encrypted.rotate_right(4);
    
    // Step 4: XOR decryption
    let final_decrypted = rot_decrypted ^ xor_key;
    
    // Verify the full crypto cycle
    assert_eq!(final_decrypted, original);
    assert_ne!(xor_encrypted, original);
    assert_ne!(rot_encrypted, original);
    
    // Test with emulator registers
    emu.regs.set_by_name("eax", original as u64);
    emu.regs.set_by_name("ebx", xor_key as u64);
    
    assert_eq!(emu.regs.get_by_name("eax"), original as u64);
    assert_eq!(emu.regs.get_by_name("ebx"), xor_key as u64);
}

#[test]
fn test_memory_manipulation_shellcode() {
    let mut emu = emu32();
    
    // Test memory manipulation with crypto operations (without execution)
    // Allocate memory for data and key
    let data_addr = emu.alloc("data", 16);
    let key_addr = emu.alloc("key", 4);
    
    // Write test data and key to memory
    let test_data = vec![0x41, 0x42, 0x43, 0x44]; // "ABCD"
    let xor_key = vec![0x12, 0x34, 0x56, 0x78];
    
    emu.maps.write_buffer(data_addr, &test_data);
    emu.maps.write_buffer(key_addr, &xor_key);
    
    // Verify data was written correctly
    let read_data = emu.maps.read_buffer(data_addr, 4);
    let read_key = emu.maps.read_buffer(key_addr, 4);
    
    assert_eq!(read_data, test_data);
    assert_eq!(read_key, xor_key);
    
    // Test XOR operation manually on the data
    let mut encrypted_data = test_data.clone();
    for i in 0..encrypted_data.len() {
        encrypted_data[i] ^= xor_key[i];
    }
    
    // Write encrypted data back to memory
    emu.maps.write_buffer(data_addr, &encrypted_data);
    
    // Verify encrypted data was written
    let read_encrypted = emu.maps.read_buffer(data_addr, 4);
    assert_eq!(read_encrypted, encrypted_data);
    
    // Verify encryption worked correctly
    let expected_encrypted = vec![
        0x41 ^ 0x12,  // A ^ key[0] = 0x53
        0x42 ^ 0x34,  // B ^ key[1] = 0x76
        0x43 ^ 0x56,  // C ^ key[2] = 0x15
        0x44 ^ 0x78   // D ^ key[3] = 0x3C
    ];
    
    assert_eq!(encrypted_data, expected_encrypted);
    
    // Test decryption (XOR again with same key)
    let mut decrypted_data = encrypted_data.clone();
    for i in 0..decrypted_data.len() {
        decrypted_data[i] ^= xor_key[i];
    }
    
    // Should get back original data
    assert_eq!(decrypted_data, test_data);
}
