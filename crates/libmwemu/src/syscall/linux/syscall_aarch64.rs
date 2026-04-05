use crate::emu;

// Linux aarch64 syscall numbers (from asm-generic/unistd.h)
// Convention: x8 = syscall number, x0-x5 = args, x0 = return
const SYS_IO_SETUP: u64 = 0;
const SYS_CLOSE: u64 = 57;
const SYS_READ: u64 = 63;
const SYS_WRITE: u64 = 64;
const SYS_OPENAT: u64 = 56;
const SYS_WRITEV: u64 = 66;
const SYS_MMAP: u64 = 222;
const SYS_MUNMAP: u64 = 215;
const SYS_MPROTECT: u64 = 226;
const SYS_BRK: u64 = 214;
const SYS_IOCTL: u64 = 29;
const SYS_EXIT: u64 = 93;
const SYS_EXIT_GROUP: u64 = 94;
const SYS_SET_TID_ADDRESS: u64 = 96;
const SYS_FUTEX: u64 = 98;
const SYS_CLOCK_GETTIME: u64 = 113;
const SYS_GETPID: u64 = 172;
const SYS_GETTID: u64 = 178;
const SYS_UNAME: u64 = 160;
const SYS_GETRANDOM: u64 = 278;
const SYS_FSTAT: u64 = 80;
const SYS_NEWFSTATAT: u64 = 79;
const SYS_RT_SIGACTION: u64 = 134;
const SYS_RT_SIGPROCMASK: u64 = 135;

pub fn gateway(emu: &mut emu::Emu) {
    let syscall_nr = emu.regs_aarch64().x[8];

    match syscall_nr {
        SYS_EXIT | SYS_EXIT_GROUP => {
            let status = emu.regs_aarch64().x[0];
            log::info!(
                "{}** {} linux aarch64 syscall exit({}) {}",
                emu.colors.light_red, emu.pos, status, emu.colors.nc
            );
            emu.stop();
        }

        SYS_WRITE => {
            let fd = emu.regs_aarch64().x[0];
            let buf = emu.regs_aarch64().x[1];
            let count = emu.regs_aarch64().x[2];

            log::info!(
                "{}** {} linux aarch64 syscall write(fd={}, buf=0x{:x}, count={}) {}",
                emu.colors.light_red, emu.pos, fd, buf, count, emu.colors.nc
            );

            if fd == 1 || fd == 2 {
                let s = emu.maps.read_string(buf);
                log::info!("{}:  \"{}\"", if fd == 1 { "stdout" } else { "stderr" }, s);
            }

            emu.regs_aarch64_mut().x[0] = count;
        }

        SYS_READ => {
            let fd = emu.regs_aarch64().x[0];
            let buf = emu.regs_aarch64().x[1];
            let count = emu.regs_aarch64().x[2];
            log::info!(
                "{}** {} linux aarch64 syscall read(fd={}, buf=0x{:x}, count={}) {}",
                emu.colors.light_red, emu.pos, fd, buf, count, emu.colors.nc
            );
            todo!("linux aarch64 read syscall");
        }

        SYS_OPENAT => {
            let dirfd = emu.regs_aarch64().x[0] as i64;
            let path_addr = emu.regs_aarch64().x[1];
            let flags = emu.regs_aarch64().x[2];
            let path = emu.maps.read_string(path_addr);
            log::info!(
                "{}** {} linux aarch64 syscall openat(dirfd={}, \"{}\", 0x{:x}) {}",
                emu.colors.light_red, emu.pos, dirfd, path, flags, emu.colors.nc
            );
            todo!("linux aarch64 openat syscall");
        }

        SYS_CLOSE => {
            let fd = emu.regs_aarch64().x[0];
            log::info!(
                "{}** {} linux aarch64 syscall close(fd={}) {}",
                emu.colors.light_red, emu.pos, fd, emu.colors.nc
            );
            emu.regs_aarch64_mut().x[0] = 0;
        }

        SYS_BRK => {
            let addr = emu.regs_aarch64().x[0];
            log::info!(
                "{}** {} linux aarch64 syscall brk(0x{:x}) {}",
                emu.colors.light_red, emu.pos, addr, emu.colors.nc
            );
            todo!("linux aarch64 brk syscall");
        }

        SYS_MMAP => {
            let addr = emu.regs_aarch64().x[0];
            let len = emu.regs_aarch64().x[1];
            let prot = emu.regs_aarch64().x[2];
            let flags = emu.regs_aarch64().x[3];
            let fd = emu.regs_aarch64().x[4];
            let off = emu.regs_aarch64().x[5];
            log::info!(
                "{}** {} linux aarch64 syscall mmap(0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}, {}, 0x{:x}) {}",
                emu.colors.light_red, emu.pos, addr, len, prot, flags, fd as i64, off, emu.colors.nc
            );
            todo!("linux aarch64 mmap syscall");
        }

        SYS_MUNMAP => {
            let addr = emu.regs_aarch64().x[0];
            let len = emu.regs_aarch64().x[1];
            log::info!(
                "{}** {} linux aarch64 syscall munmap(0x{:x}, 0x{:x}) {}",
                emu.colors.light_red, emu.pos, addr, len, emu.colors.nc
            );
            todo!("linux aarch64 munmap syscall");
        }

        SYS_MPROTECT => {
            let addr = emu.regs_aarch64().x[0];
            let len = emu.regs_aarch64().x[1];
            let prot = emu.regs_aarch64().x[2];
            log::info!(
                "{}** {} linux aarch64 syscall mprotect(0x{:x}, 0x{:x}, 0x{:x}) {}",
                emu.colors.light_red, emu.pos, addr, len, prot, emu.colors.nc
            );
            todo!("linux aarch64 mprotect syscall");
        }

        SYS_IOCTL => {
            let fd = emu.regs_aarch64().x[0];
            let request = emu.regs_aarch64().x[1];
            log::info!(
                "{}** {} linux aarch64 syscall ioctl(fd={}, 0x{:x}) {}",
                emu.colors.light_red, emu.pos, fd, request, emu.colors.nc
            );
            todo!("linux aarch64 ioctl syscall");
        }

        SYS_SET_TID_ADDRESS => {
            log::info!(
                "{}** {} linux aarch64 syscall set_tid_address() => 1 {}",
                emu.colors.light_red, emu.pos, emu.colors.nc
            );
            emu.regs_aarch64_mut().x[0] = 1; // fake tid
        }

        SYS_GETPID => {
            log::info!(
                "{}** {} linux aarch64 syscall getpid() => 1000 {}",
                emu.colors.light_red, emu.pos, emu.colors.nc
            );
            emu.regs_aarch64_mut().x[0] = 1000;
        }

        SYS_GETTID => {
            log::info!(
                "{}** {} linux aarch64 syscall gettid() => 1000 {}",
                emu.colors.light_red, emu.pos, emu.colors.nc
            );
            emu.regs_aarch64_mut().x[0] = 1000;
        }

        SYS_RT_SIGACTION => {
            log::info!(
                "{}** {} linux aarch64 syscall rt_sigaction() => 0 {}",
                emu.colors.light_red, emu.pos, emu.colors.nc
            );
            emu.regs_aarch64_mut().x[0] = 0;
        }

        SYS_RT_SIGPROCMASK => {
            log::info!(
                "{}** {} linux aarch64 syscall rt_sigprocmask() => 0 {}",
                emu.colors.light_red, emu.pos, emu.colors.nc
            );
            emu.regs_aarch64_mut().x[0] = 0;
        }

        SYS_CLOCK_GETTIME => {
            log::info!(
                "{}** {} linux aarch64 syscall clock_gettime() => 0 {}",
                emu.colors.light_red, emu.pos, emu.colors.nc
            );
            emu.regs_aarch64_mut().x[0] = 0;
        }

        SYS_GETRANDOM => {
            let buf = emu.regs_aarch64().x[0];
            let count = emu.regs_aarch64().x[1];
            log::info!(
                "{}** {} linux aarch64 syscall getrandom(0x{:x}, {}) {}",
                emu.colors.light_red, emu.pos, buf, count, emu.colors.nc
            );
            // Write zeros (not random, but functional)
            for i in 0..count {
                emu.maps.write_byte(buf + i, 0);
            }
            emu.regs_aarch64_mut().x[0] = count;
        }

        SYS_UNAME => {
            log::info!(
                "{}** {} linux aarch64 syscall uname() => 0 {}",
                emu.colors.light_red, emu.pos, emu.colors.nc
            );
            emu.regs_aarch64_mut().x[0] = 0;
        }

        _ => {
            log::warn!(
                "{}** {} linux aarch64 unimplemented syscall {} (x0=0x{:x}, x1=0x{:x}, x2=0x{:x}) {}",
                emu.colors.light_red,
                emu.pos,
                syscall_nr,
                emu.regs_aarch64().x[0],
                emu.regs_aarch64().x[1],
                emu.regs_aarch64().x[2],
                emu.colors.nc
            );
        }
    }
}
