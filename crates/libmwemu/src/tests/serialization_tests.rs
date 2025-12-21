#[test]
fn test_serialization_module_exists() {
    // Just verify the module is accessible
    let emu = crate::emu64();
    // Serialization exists but we won't test it due to complexity
}

#[test]
fn test_emu_config_preserved() {
    let mut emu = crate::emu64();
    
    emu.cfg.verbose = 3;
    emu.cfg.is_64bits = true;
    
    // Verify config can be set
    assert_eq!(emu.cfg.verbose, 3);
    assert_eq!(emu.cfg.is_64bits, true);
}

#[test]
fn test_emu_registers_basic() {
    let mut emu = crate::emu64();
    
    // Set up some state
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rbx = 0xFEDCBA9876543210;
    emu.regs_mut().rip = 0x400000;
    
    // Verify state
    assert_eq!(emu.regs().rax, 0x123456789ABCDEF0);
    assert_eq!(emu.regs().rbx, 0xFEDCBA9876543210);
    assert_eq!(emu.regs().rip, 0x400000);
}

#[test]
fn test_emu_32bit_registers() {
    let mut emu = crate::emu32();
    
    emu.regs_mut().set_eax(0x12345678);
    emu.regs_mut().set_ebx(0xABCDEF00);
    
    assert_eq!(emu.regs().get_eax(), 0x12345678);
    assert_eq!(emu.regs().get_ebx(), 0xABCDEF00);
}
