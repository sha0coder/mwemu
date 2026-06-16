use crate::tests::helpers;
use crate::*;

#[test]
pub fn elf64lin_syscall64() {
    helpers::setup();

    let path = match helpers::optional_test_data_path("elf64lin_syscall64.bin") {
        Some(p) => p,
        None => return,
    };

    let mut emu = emu64();
    emu.load_code(&path);
    emu.run_to(80000);
    assert_eq!(emu.regs().r12, 549);
}
