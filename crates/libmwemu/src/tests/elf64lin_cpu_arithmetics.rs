use crate::*;
use crate::tests::helpers;

#[test]
// arithmetic calculations on an 64bits elf
pub fn elf64lin_cpu_arithmetics() {
    helpers::setup();

    let mut emu = emu64();
    emu.load_code("../../test/elf64lin_cpu_arithmetics.bin");

    assert_eq!(emu.flags().dump(), 0x202); // initial flags (match with gdb linux)

    emu.run_to(5); // position 5 is emulated
    assert_eq!(emu.regs().rax, 3);
    assert_eq!(emu.flags().dump(), 0x206);

    emu.run_to(6);  // dec ax
    assert_eq!(emu.regs().rax, 2);
    assert_eq!(emu.flags().dump(), 0x202);

    emu.run_to(8); // last dec rax zero reached
    assert_eq!(emu.regs().rax, 0);
    assert_eq!(emu.flags().dump(), 0x246);

    emu.run_to(11); // neg ax 
    assert_eq!(emu.regs().rax, 0x1122334455668888);
    assert_eq!(emu.flags().dump(), 0x297); // [ CF PF AF SF IF ]

    emu.run_to(14); // sar al, 1
    assert_eq!(emu.regs().rax, 0xffffffff556688c4);
    assert_eq!(emu.flags().dump(), 0x292);

    emu.run_to(23); // shl ax, 1
    assert_eq!(emu.regs().rax, 0x15596260);
    assert_eq!(emu.flags().dump(), 0xa17);

    emu.run_to(29); // shl rax, cl
    assert_eq!(emu.regs().rax, 0x55658980);
    assert_eq!(emu.flags().dump(), 0x212);
    
    emu.run_to(30); // shr al, 1
    assert_eq!(emu.regs().rax, 0x55658940);                
    assert_eq!(emu.flags().dump(), 0xa12);
                    
    emu.run_to(31); // shr ax, 1
    assert_eq!(emu.regs().rax, 0x556544a0);                
    assert_eq!(emu.flags().dump(), 0xa16);

    emu.run_to(40); // imul eax
    assert_eq!(emu.regs().rax, 0x21000000);
    assert_eq!(emu.flags().dump(), 0xa17); // [ CF PF AF IF OF ]

    emu.run_to(41); // imul rax
    assert_eq!(emu.regs().rax, 0x441000000000000);
    assert_eq!(emu.flags().dump(), 0x216); // [ PF AF IF ]
                                            
    emu.run_to(43); // imul eax, eax
    assert_eq!(emu.regs().rax, 0);
    assert_eq!(emu.flags().dump(), 0x216); // [ PF AF IF ]

    emu.run_to(45); // imul rax, rax
    assert_eq!(emu.regs().rax, 0x1eace4a3c82fb840);
    assert_eq!(emu.flags().dump(), 0xa17); // [ CF PF AF IF OF ]

    emu.run_to(48); // imul  rax,2
    assert_eq!(emu.regs().rax, 0x120bdc200);
    assert_eq!(emu.flags().dump(), 0x216); // [ PF AF IF ]  

    emu.run_to(49); // rcl al, 1
    assert_eq!(emu.regs().rax, 0x120bdc200); 
    assert_eq!(emu.flags().dump(), 0x216); // [ PF AF IF ]

    emu.run_to(50); // rcl ax, 1
    assert_eq!(emu.regs().rax, 0x120bd8400);
    assert_eq!(emu.flags().dump(), 0x217); // [ CF PF AF IF ]

    emu.run_to(52); // rcl   rax,1
    assert_eq!(emu.regs().rax, 0x82f61002); // ERROR
    assert_eq!(emu.flags().dump(), 0x216); // [ PF AF IF ]


    emu.run_to(58); // rcr   ax,1
    assert_eq!(emu.regs().rax, 0x82f60800);
    assert_eq!(emu.flags().dump(), 0x217); // [ CF PF AF IF ]

    emu.run_to(64);
    assert_eq!(emu.regs().rax, 0x60bd8200);
    assert_eq!(emu.flags().dump(), 0x216); // [ PF AF IF ]

    emu.run_to(69);
}
