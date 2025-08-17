use crate::{tests::helpers, *};

#[test]
pub fn memmove_test() {
    helpers::setup();

    // memmove implementation in x86-64 assembly
    // Function signature: memmove(dest: RDX, src: RCX+RDX, len: R8) -> RAX (returns dest)
    let memmove_code = hex::decode("4C89C04829D10F849100000073094801C80F826D0100004983F8080F8C63000000F6C2077437F6C201740C8A041149FFC888024883C201F6C202740F668B04114983E8026689024883C202F6C204740D8B04114983E80489024883C2044D89C149C1E90575384D89C149C1E903741590488B04114889024883C20849FFC975F04983E0074D85C07E140F1F80000000008A0411880248FFC249FFC875F3C34981F90020000072094881F90010000073334883C220488B4411E04C8B5411E8488942E04C8952E849FFC9488B4411F04C8B5411F8488942F04C8952F875D34983E01FEB83B8200000000F1F8400000000000F1804110F184411404881C280000000FFC875EC4881EA00100000B8400000004883C2404C8B4C11C04C8B5411C84C0FC34AC04C0FC352C84C8B4C11D04C8B5411D84C0FC34AD04C0FC352D8FFC84C8B4C11E04C8B5411E84C0FC34AE04C0FC352E84C8B4C11F04C8B5411F84C0FC34AF04C0FC352F875A84981E8001000004981F8001000000F8367FFFFFF0FAEF0E9D9FEFFFF4C01C24983F8087C61F6C2077436F6C201740B48FFCA8A041149FFC88802F6C202740F4883EA02668B04114983E802668902F6C204740D4883EA048B04114983E80489024D89C149C1E905753D4D89C149C1E90374144883EA08488B041149FFC948890275F04983E0074D85C07E1A6666660F1F840000000000669048FFCA8A041149FFC8880275F3C34981F90020000072094881F900F0FFFF72344883EA20488B4411184C8B541110488942184C89521049FFC9488B4411084C8B1411488942084C891275D54983E01FE97DFFFFFFB8200000000F1F80000000004881EA800000000F1804110F18441140FFC875EC4881C200100000B8400000004883EA404C8B4C11384C8B5411304C0FC34A384C0FC352304C8B4C11284C8B5411204C0FC34A284C0FC35220FFC84C8B4C11184C8B5411104C0FC34A184C0FC352104C8B4C11084C8B14114C0FC34A084C0FC31275AA4981E8001000004981F8001000000F836AFFFFFF0FAEF0E9D6FEFFFF").unwrap();
    let memmove_code_len = memmove_code.len();
    
    let mut emu = emu64();
    
    // Load memmove code at address 0x400000
    let code_addr = 0x400000;
    emu.maps.create_map("code", code_addr, memmove_code_len as u64);
    emu.maps.write_bytes(code_addr, memmove_code);
    
    // Allocate test buffers
    let buffer_size = 0x1000;
    let src_addr = 0x500000;
    let dest_addr = 0x600000;
    
    emu.maps.create_map("src", src_addr, buffer_size);
    emu.maps.create_map("dest", dest_addr, buffer_size);
    
    // Test 1: Non-overlapping forward copy (simple case)
    {
        // Initialize source with pattern
        let test_pattern = b"Hello, World! This is a test pattern.";
        emu.maps.write_bytes(src_addr, test_pattern.to_vec());
        
        // Set up registers for memmove(dest, src, len)
        emu.regs_mut().rdx = dest_addr;
        emu.regs_mut().rcx = src_addr - dest_addr; // RCX = src - dest (offset)
        emu.regs_mut().r8 = test_pattern.len() as u64;
        emu.regs_mut().rip = code_addr;
        
        // Execute memmove
        emu.run(Some(code_addr + memmove_code_len as u64));
        
        // Verify the copy
        let copied_data = emu.maps.read_bytes(dest_addr, test_pattern.len());
        assert_eq!(copied_data, test_pattern);
        
        // Verify return value (should be dest)
        assert_eq!(emu.regs().rax, dest_addr);
    }
    
    // Test 2: Overlapping copy - forward direction (dest > src)
    {
        // Create overlapping scenario where dest overlaps with end of src
        let overlap_src = 0x700000;
        let overlap_dest = 0x700010; // 16 bytes overlap
        let test_data: Vec<u8> = (0..64).collect();
        
        emu.maps.create_map("overlap", overlap_src, 0x100);
        emu.maps.write_bytes(overlap_src, test_data.clone());
        
        // Set up for overlapping copy
        emu.regs_mut().rdx = overlap_dest;
        emu.regs_mut().rcx = overlap_src - overlap_dest;
        emu.regs_mut().r8 = 32; // Copy 32 bytes with 16-byte overlap
        emu.regs_mut().rip = code_addr;
        
        // Execute memmove
        emu.run(Some(code_addr + memmove_code_len as u64));
        
        // Verify correct backward copy (to avoid corruption)
        let result = emu.maps.read_bytes(overlap_dest, 32);
        let expected: Vec<u8> = (0..32).collect();
        assert_eq!(result, expected);
    }
    
    // Test 3: Overlapping copy - backward direction (dest < src)  
    {
        let overlap_src = 0x800010;
        let overlap_dest = 0x800000;
        let test_data: Vec<u8> = (0..64).collect();
        
        emu.maps.create_map("overlap2", 0x800000, 0x100);
        emu.maps.write_bytes(overlap_src, test_data.clone());
        
        // Set up for backward overlapping copy
        emu.regs_mut().rdx = overlap_dest;
        emu.regs_mut().rcx = overlap_src - overlap_dest;
        emu.regs_mut().r8 = 32;
        emu.regs_mut().rip = code_addr;
        
        // Execute memmove
        emu.run(Some(code_addr + memmove_code_len as u64));
        
        // Verify correct forward copy
        let result = emu.maps.read_bytes(overlap_dest, 32);
        let expected: Vec<u8> = (0..32).collect();
        assert_eq!(result, expected);
    }
    
    // Test 4: Large buffer copy (test MOVNTI optimization path)
    {
        let large_src = 0x900000;
        let large_dest = 0xA00000;
        let large_size = 0x2000; // 8KB
        
        emu.maps.create_map("large_src", large_src, large_size);
        emu.maps.create_map("large_dest", large_dest, large_size);
        
        // Fill with pattern
        let mut pattern = Vec::new();
        for i in 0..large_size {
            pattern.push((i % 256) as u8);
        }
        emu.maps.write_bytes(large_src, pattern.clone());
        
        // Set up for large copy
        emu.regs_mut().rdx = large_dest;
        emu.regs_mut().rcx = large_src - large_dest;
        emu.regs_mut().r8 = large_size;
        emu.regs_mut().rip = code_addr;
        
        // Execute memmove
        emu.run(Some(code_addr + memmove_code_len as u64));
        
        // Verify large copy
        let result = emu.maps.read_bytes(large_dest, large_size as usize);
        assert_eq!(result, pattern);
    }
    
    // Test 5: Zero-length copy
    {
        emu.regs_mut().rdx = dest_addr;
        emu.regs_mut().rcx = src_addr - dest_addr;
        emu.regs_mut().r8 = 0;
        emu.regs_mut().rip = code_addr;
        
        // Execute memmove with zero length
        emu.run(Some(code_addr + memmove_code_len as u64));
        
        // Should return dest address without doing anything
        assert_eq!(emu.regs().rax, dest_addr);
    }
    
    // Test 6: Unaligned addresses
    {
        let unaligned_src = 0xB00003;
        let unaligned_dest = 0xC00007;
        let test_data = b"Unaligned test data";
        
        emu.maps.create_map("unaligned_src", 0xB00000, 0x100);
        emu.maps.create_map("unaligned_dest", 0xC00000, 0x100);
        emu.maps.write_bytes(unaligned_src, test_data.to_vec());
        
        emu.regs_mut().rdx = unaligned_dest;
        emu.regs_mut().rcx = unaligned_src - unaligned_dest;
        emu.regs_mut().r8 = test_data.len() as u64;
        emu.regs_mut().rip = code_addr;
        
        // Execute memmove with unaligned addresses
        emu.run(Some(code_addr + memmove_code_len as u64));
        
        // Verify unaligned copy
        let result = emu.maps.read_bytes(unaligned_dest, test_data.len());
        assert_eq!(result, test_data);
    }
}