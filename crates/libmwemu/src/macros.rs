#[macro_export]
macro_rules! popn {
    ($emu:expr, $n:expr) => {
        for _ in 0..$n {
            $emu.stack_pop(false);
        }
    };
}

#[macro_export]
macro_rules! get_bit {
    ($val:expr, $count:expr) => {
        ($val & (1 << $count)) >> $count
    };
}

#[macro_export]
macro_rules! set_bit {
    ($val:expr, $count:expr, $bit:expr) => {
        if $bit == 1 {
            $val |= 1 << $count;
        } else {
            $val &= !(1 << $count);
        }
    };
}

#[macro_export]
macro_rules! to32 {
    ($val:expr) => {
        ($val & 0xffffffff) as u32
    };
}

#[macro_export]
macro_rules! log_red {
    ($emu:expr, $($arg:tt)*) => {
        if $emu.cfg.nocolors {
            log::info!(
                "** {}:{:x} {}",
                $emu.pos,
                $emu.regs().rip,
                format!($($arg)*)
            );
        } else {
            log::info!(
                "{}** {}:{:x} {}{}",
                $emu.colors.light_red,
                $emu.pos,
                $emu.regs().rip,
                format!($($arg)*),
                $emu.colors.nc
            );
        }
    };
}
