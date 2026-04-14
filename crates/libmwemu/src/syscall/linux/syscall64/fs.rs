use crate::emu;
use crate::windows::constants;
use crate::winapi::helper;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub(super) fn dispatch(emu: &mut emu::Emu) -> bool {
    match emu.regs().rax {
        constants::NR64_READ => handle_syscall64_read(emu),
        constants::NR64_WRITE => handle_syscall64_write(emu),
        constants::NR64_OPEN => handle_syscall64_open(emu),
        constants::NR64_OPENAT => handle_syscall64_openat(emu),
        constants::NR64_CLOSE => handle_syscall64_close(emu),
        constants::NR64_EXECVE => handle_syscall64_execve(emu),
        constants::NR64_CHDIR => handle_syscall64_chdir(emu),
        constants::NR64_CHMOD => handle_syscall64_chmod(emu),
        constants::NR64_LSEEK => handle_syscall64_lseek(emu),
        _ => return false,
    }

    true
}

pub(super) fn handle_syscall64_read(emu: &mut emu::Emu) {
    let fd = emu.regs().rdi;
    let buff = emu.regs().rsi;
    let sz = emu.regs().rdx;

    if helper::handler_exist(fd) {
        let filepath = helper::handler_get_uri(fd);
        if filepath.contains(".so") {
            let mut lib_buff: Vec<u8> = Vec::new();

            match File::open(&filepath) {
                Ok(f) => {
                    let mut reader = BufReader::new(&f);
                    reader
                        .read_to_end(&mut lib_buff)
                        .expect("kernel64 cannot load dynamic library");
                    f.sync_all();

                    let map = emu
                        .maps
                        .get_mem_by_addr_mut(buff)
                        .expect("buffer send to read syscall point to no map");

                    let mem_end = map.get_base() + map.size() as u64 - 1;
                    let buff_end = buff + lib_buff.len() as u64 - 1;
                    if buff_end > mem_end {
                        let overflow = buff_end - mem_end;
                        lib_buff = lib_buff[0..lib_buff.len() - overflow as usize].to_vec();
                    }

                    emu.maps.write_bytes(buff, &lib_buff);
                    emu.regs_mut().rax = sz;
                }
                Err(_) => {
                    log::trace!("file not found");
                    emu.regs_mut().rax = 0;
                }
            };
        }
    } else {
        emu.regs_mut().rax = sz;
    }

    log::trace!(
        "{}** {} syscall read(fd:{} buf:0x{:x} sz:{}) ={} {}",
        emu.colors.light_red,
        emu.pos,
        fd,
        buff,
        sz,
        emu.regs().rax,
        emu.colors.nc
    );
}

pub(super) fn handle_syscall64_write(emu: &mut emu::Emu) {
    let fd = emu.regs().rdi;
    let buff = emu.regs().rsi;
    let sz = emu.regs().rdx;
    emu.regs_mut().rax = sz;
    log::trace!(
        "{}** {} syscall write() fd: {} buf: 0x{:x} sz: {} {}",
        emu.colors.light_red,
        emu.pos,
        fd,
        buff,
        sz,
        emu.colors.nc
    );
    if fd == 1 {
        let s = emu.maps.read_string(buff);
        log::trace!("stdout: `{}`", s)
    }
    if fd == 2 {
        let s = emu.maps.read_string(buff);
        log::trace!("stderr: `{}`", s)
    }
}

pub(super) fn handle_syscall64_open(emu: &mut emu::Emu) {
    let file_path = emu.maps.read_string(emu.regs().rdi);
    let fd = helper::handler_create(&file_path);

    emu.regs_mut().rax = fd;
    log::trace!(
        "{}** {} syscall open({}) ={} {}",
        emu.colors.light_red,
        emu.pos,
        file_path,
        fd,
        emu.colors.nc
    );
}

pub(super) fn handle_syscall64_openat(emu: &mut emu::Emu) {
    let dirfd = emu.regs().rdi;
    let file_path = emu.maps.read_string(emu.regs().rsi);
    let mut fd: u64 = 0xffffffff_ffffffff;

    let path = Path::new(&file_path);
    if path.exists() {
        fd = helper::handler_create(&file_path);
    }

    log::trace!(
        "{}** {} syscall openat({} '{}') ={} {}",
        emu.colors.light_red,
        emu.pos,
        dirfd,
        file_path,
        fd as i64,
        emu.colors.nc
    );

    emu.regs_mut().rax = fd;
}

pub(super) fn handle_syscall64_close(emu: &mut emu::Emu) {
    let fd = emu.regs().rdi;

    if helper::handler_exist(fd) {
        helper::handler_close(fd);
        emu.regs_mut().rax = 0;
    } else {
        helper::socket_close(fd);
        emu.regs_mut().rax = 0xffffffff_ffffffff;
    }

    log::trace!(
        "{}** {} syscall close(fd:{}) ={} {}",
        emu.colors.light_red,
        emu.pos,
        fd,
        emu.regs().rax,
        emu.colors.nc
    );
}

pub(super) fn handle_syscall64_execve(emu: &mut emu::Emu) {
    let cmd = emu.maps.read_string(emu.regs().rdi);
    log::trace!(
        "{}** {} syscall execve()  cmd: {} {}",
        emu.colors.light_red,
        emu.pos,
        cmd,
        emu.colors.nc
    );
    emu.regs_mut().rax = 0;
}

pub(super) fn handle_syscall64_chdir(emu: &mut emu::Emu) {
    let path = emu.maps.read_string(emu.regs().rdi);
    log::trace!(
        "{}** {} syscall chdir() path: {} {}",
        emu.colors.light_red,
        emu.pos,
        path,
        emu.colors.nc
    );
}

pub(super) fn handle_syscall64_chmod(emu: &mut emu::Emu) {
    let file_path = emu.maps.read_string(emu.regs().rdi);
    let perm = emu.regs().rsi;
    log::trace!(
        "{}** {} syscall chmod() file: {} perm: {} {}",
        emu.colors.light_red,
        emu.pos,
        file_path,
        perm,
        emu.colors.nc
    );
}

pub(super) fn handle_syscall64_lseek(emu: &mut emu::Emu) {
    let fd = emu.regs().rdi;
    log::trace!(
        "{}** {} syscall lseek()  fd: {} {}",
        emu.colors.light_red,
        emu.pos,
        fd,
        emu.colors.nc
    );
}
