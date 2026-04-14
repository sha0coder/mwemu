use crate::emu;
use crate::maps::mem64::Permission;
use crate::windows::constants;
use crate::windows::structures;
use crate::winapi::helper; // TODO: why not winapi64 helper?
use std::fs as stdfs;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;
macro_rules! syscall_name {
    ($name:literal) => {
        $name
    };
    ($name:ident) => {
        stringify!($name)
    };
}

macro_rules! syscall_names {
    ($($name:tt),* $(,)?) => {
        &[$(syscall_name!($name)),*]
    };
}

pub(super) fn dispatch(emu: &mut emu::Emu) {
    dispatch_legacy_syscall64(emu);
}

fn trace_simple_syscall64(emu: &mut emu::Emu, name: &str) {
    super::trace_syscall64(emu, &format!("{name}()"));
}

fn trace_socketcall64(emu: &mut emu::Emu, name: &str) {
    super::trace_syscall64(emu, &format!("socketcall {name}()"));
}

fn format_trace_args(args: &[(&str, String)]) -> String {
    if args.is_empty() {
        return "()".to_string();
    }

    let mut rendered = String::from("(");
    for (idx, (label, value)) in args.iter().enumerate() {
        if idx > 0 {
            rendered.push(' ');
        }
        rendered.push_str(label);
        rendered.push_str(": ");
        rendered.push_str(value);
    }
    rendered.push(')');
    rendered
}

fn trace_syscall64_args(emu: &mut emu::Emu, name: &str, args: &[(&str, String)]) {
    super::trace_syscall64(emu, &format!("{name}{}", format_trace_args(args)));
}

fn trace_socketcall64_args(emu: &mut emu::Emu, name: &str, args: &[(&str, String)]) {
    super::trace_syscall64(
        emu,
        &format!("socketcall {name}{}", format_trace_args(args)),
    );
}

fn trace_legacy_syscall64(emu: &mut emu::Emu, name: &str) {
    trace_simple_syscall64(emu, name);
}

fn trace_bad_syscall64(emu: &mut emu::Emu, nr: u64) {
    super::trace_syscall64(emu, &format!("bad_nr(0x{nr:x})"));
}
fn dispatch_legacy_syscall64(emu: &mut emu::Emu) {
    match emu.regs().rax {
        constants::NR64_RESTART_SYSCALL => super::proc::handle_syscall64_restart(emu),

        constants::NR64_EXIT => {
            super::proc::handle_syscall64_exit(emu);
            return;
        }

        constants::NR64_FORK => super::proc::handle_syscall64_fork(emu),

        constants::NR64_READ => super::fs::handle_syscall64_read(emu),

        constants::NR64_WRITE => super::fs::handle_syscall64_write(emu),

        /*
        constants::NR64_WRITEV => {
            let fd = emu.regs().rdi;
            let buff = emu.regs().rsi;
            let sz = emu.regs().rdx;
            emu.regs_mut().rax = sz;
            log::trace!(
                "{}** {} syscall write() fd: {} buf: 0x{:x} sz: {} {}",
                emu.colors.light_red, emu.pos, fd, buff, sz, emu.colors.nc
            );
            if fd == 1 {
                let s = emu.maps.read_string(buff);
                log::trace!("stdout: `{}`", s)
            }
            if fd == 2 {
                let s = emu.maps.read_string(buff);
                log::trace!("stderr: `{}`", s)
            }
        }*/
        constants::NR64_OPEN => super::fs::handle_syscall64_open(emu),

        constants::NR64_OPENAT => super::fs::handle_syscall64_openat(emu),

        constants::NR64_CLOSE => super::fs::handle_syscall64_close(emu),

        constants::NR64_BRK => super::memory::handle_syscall64_brk(emu),

        constants::NR64_EXECVE => super::fs::handle_syscall64_execve(emu),

        constants::NR64_CHDIR => super::fs::handle_syscall64_chdir(emu),

        constants::NR64_CHMOD => super::fs::handle_syscall64_chmod(emu),

        constants::NR64_LSEEK => super::fs::handle_syscall64_lseek(emu),

        constants::NR64_KILL => super::proc::handle_syscall64_kill(emu),

        constants::NR64_DUP => super::proc::handle_syscall64_dup(emu),

        constants::NR64_DUP2 => super::proc::handle_syscall64_dup2(emu),

        constants::NR64_SOCKET => super::net::handle_syscall64_socket(emu),

        constants::NR64_BIND => {
            let sock = emu.regs().rdi;
            let sockaddr = emu.regs().rsi;
            let len = emu.regs().rdx;

            if sockaddr > 0 && emu.maps.is_mapped(sockaddr) {
                let fam: u16 = emu.maps.read_word(sockaddr).expect("cannot read family id");
                let port: u16 = emu
                    .maps
                    .read_word(sockaddr + 2)
                    .expect("cannot read the port")
                    .to_be();
                let ip: u32 = emu
                    .maps
                    .read_dword(sockaddr + 4)
                    .expect("cannot read the ip");
                let sip = format!(
                    "{}.{}.{}.{}",
                    ip & 0xff,
                    (ip & 0xff00) >> 8,
                    (ip & 0xff0000) >> 16,
                    (ip & 0xff000000) >> 24
                );

                trace_socketcall64_args(
                    emu,
                    "bind",
                    &[
                        ("sock", sock.to_string()),
                        ("family", fam.to_string()),
                        ("addr", format!("{sip}:{port}")),
                        ("addr_len", len.to_string()),
                    ],
                );

                if !helper::socket_exist(sock) {
                    log::trace!("\tbad socket/");
                    emu.regs_mut().rax = constants::ENOTSOCK;
                } else {
                    emu.regs_mut().rax = 0;
                }
            } else {
                trace_socketcall64_args(
                    emu,
                    "bind",
                    &[
                        ("sock", sock.to_string()),
                        ("addr", "bad_sockaddr".to_string()),
                        ("addr_len", len.to_string()),
                    ],
                );
                emu.regs_mut().rax = constants::EINVAL;
            }
        }

        constants::NR64_CONNECT => {
            let sock = emu.regs().rdi;
            let sockaddr = emu.regs().rsi;
            let len = emu.regs().rdx;

            if sockaddr > 0 && emu.maps.is_mapped(sockaddr) {
                let fam: u16 = emu.maps.read_word(sockaddr).expect("cannot read family id");
                let port: u16 = emu
                    .maps
                    .read_word(sockaddr + 2)
                    .expect("cannot read the port")
                    .to_be();
                let ip: u32 = emu
                    .maps
                    .read_dword(sockaddr + 4)
                    .expect("cannot read the ip");
                let sip = format!(
                    "{}.{}.{}.{}",
                    ip & 0xff,
                    (ip & 0xff00) >> 8,
                    (ip & 0xff0000) >> 16,
                    (ip & 0xff000000) >> 24
                );

                trace_socketcall64_args(
                    emu,
                    "connect",
                    &[
                        ("sock", sock.to_string()),
                        ("family", fam.to_string()),
                        ("addr", format!("{sip}:{port}")),
                        ("addr_len", len.to_string()),
                    ],
                );

                if !helper::socket_exist(sock) {
                    log::trace!("\tbad socket/");
                    emu.regs_mut().rax = constants::ENOTSOCK;
                    return;
                }

                /*
                if emu.cfg.endpoint {
                    if endpoint::sock_connect(sip.as_str(), port) {
                        log::trace!("\tconnected to the endpoint.");
                    } else {
                        log::trace!("\tcannot connect. dont use -e");
                    }
                }*/

                emu.regs_mut().rax = 0;
            } else {
                trace_socketcall64_args(
                    emu,
                    "connect",
                    &[
                        ("sock", sock.to_string()),
                        ("addr", "bad_sockaddr".to_string()),
                        ("addr_len", len.to_string()),
                    ],
                );
                emu.regs_mut().rax = constants::EINVAL;
            }
        }

        constants::NR64_LISTEN => {
            let sock = emu.regs().rdi;
            let conns = emu.regs().rsi;

            trace_socketcall64_args(
                emu,
                "listen",
                &[("sock", sock.to_string()), ("backlog", conns.to_string())],
            );

            if !helper::socket_exist(sock) {
                log::trace!("\tbad socket/");
                emu.regs_mut().rax = constants::ENOTSOCK;
            } else {
                emu.regs_mut().rax = 0;
            }
        }

        constants::NR64_ACCEPT => {
            let sock = emu.regs().rdi;
            let sockaddr = emu.regs().rsi;
            let len = emu.regs().rdx;

            let port: u16 = 8080;
            let incoming_ip: u32 = 0x11223344;

            if sockaddr != 0 && emu.maps.is_mapped(sockaddr) {
                emu.maps.write_word(sockaddr, 0x0002);
                emu.maps.write_word(sockaddr + 2, port.to_le()); //TODO: port should be the same than bind()
                emu.maps.write_dword(sockaddr + 4, incoming_ip);
            }

            trace_socketcall64_args(
                emu,
                "accept",
                &[
                    ("sock", sock.to_string()),
                    ("addr", format!("0x{sockaddr:x}")),
                    ("addr_len", len.to_string()),
                ],
            );

            if !helper::socket_exist(sock) {
                log::trace!("\tbad socket/");
                emu.regs_mut().rax = constants::ENOTSOCK;
            } else {
                emu.regs_mut().rax = 0;
            }
        }

        constants::NR64_GETSOCKNAME => {
            let sock = emu.regs().rdi;
            trace_socketcall64_args(emu, "getsockname", &[("sock", sock.to_string())]);
            emu.regs_mut().rax = 0;
        }

        constants::NR64_GETPEERNAME => {
            trace_socketcall64(emu, "getpeername");
            emu.regs_mut().rax = 0;
        }

        constants::NR64_SOCKETPAIR => {
            trace_socketcall64(emu, "socketpair");
            emu.regs_mut().rax = 0;
        }

        /*constants::NR64_SEND => {
            let sock = emu.maps.read_dword(emu.regs().rsp).expect("send() cannot read sock");
            let buf = emu.maps.read_dword(emu.regs().rsp+4).expect("send() cannot read buff");
            let len = emu.maps.read_dword(emu.regs().rsp+8).expect("send() cannot read len");
            let flags = emu.maps.read_dword(emu.regs().rsp+12).expect("send() cannot read flags");

            log::trace!("{}** {} syscall socketcall send() sock: {} buff: {} len: {} {}", emu.colors.light_red, emu.pos, sock, buf, len, emu.colors.nc);

            if !helper::socket_exist(sock) {
                log::trace!("\tbad socket/");
                emu.regs_mut().rax = constants::ENOTSOCK;
                return;
            }

            if emu.cfg.endpoint {
                let buffer = emu.maps.read_buffer(buf, len as usize);
                let n = endpoint::sock_send(&buffer);
                log::trace!("\tsent {} bytes.", n);
                emu.regs_mut().rax = n;
            } else {
                emu.regs_mut().rax = len;
            }
        }

        constants::NR64_RECV => {
            let sock = emu.maps.read_dword(emu.regs().rsp).expect("recv() cannot read sock");
            let buf = emu.maps.read_dword(emu.regs().rsp+4).expect("recv() cannot read buff");
            let len = emu.maps.read_dword(emu.regs().rsp+8).expect("recv() cannot read len");
            let flags = emu.maps.read_dword(emu.regs().rsp+12).expect("recv() cannot read flags");

            log::trace!("{}** {} syscall socketcall recv() sock: {} buff: {} len: {}  {}", emu.colors.light_red, emu.pos, sock, buf, len, emu.colors.nc);

            if !helper::socket_exist(sock) {
                log::trace!("\tbad socket/");
                emu.regs_mut().rax = constants::ENOTSOCK;
                return;
            }

            if emu.cfg.endpoint {

                let mut rbuff:Vec<u8> = vec![0;len as usize];
                let n = endpoint::sock_recv(&mut rbuff);
                emu.maps.write_buffer(buf, &rbuff);
                log::trace!("\nreceived {} bytes from the endpoint.", n);
                emu.regs_mut().rax = n;

            } else {
                emu.regs_mut().rax = len; //TODO: avoid loops
            }
        }*/
        constants::NR64_SENDTO => {
            let sock = emu.regs().rdi;
            let buf = emu.regs().rsi;
            let len = emu.regs().rdx;
            let flags = emu.regs().r10;
            let sockaddr = emu.regs().r8;
            let addrlen = emu.regs().r9;

            if sockaddr != 0 && emu.maps.is_mapped(sockaddr) {
                let fam: u16 = emu.maps.read_word(sockaddr).expect("cannot read family id");
                let port: u16 = emu
                    .maps
                    .read_word(sockaddr + 2)
                    .expect("cannot read the port")
                    .to_be();
                let ip: u32 = emu
                    .maps
                    .read_dword(sockaddr + 4)
                    .expect("cannot read the ip");
                let sip = format!(
                    "{}.{}.{}.{}",
                    ip & 0xff,
                    (ip & 0xff00) >> 8,
                    (ip & 0xff0000) >> 16,
                    (ip & 0xff000000) >> 24
                );

                trace_socketcall64_args(
                    emu,
                    "sendto",
                    &[
                        ("sock", sock.to_string()),
                        ("buf", buf.to_string()),
                        ("len", len.to_string()),
                        ("flags", flags.to_string()),
                        ("family", fam.to_string()),
                        ("addr", format!("{sip}:{port}")),
                        ("addr_len", addrlen.to_string()),
                    ],
                );
            } else {
                trace_socketcall64_args(
                    emu,
                    "sendto",
                    &[
                        ("sock", sock.to_string()),
                        ("buf", buf.to_string()),
                        ("len", len.to_string()),
                        ("flags", flags.to_string()),
                        ("addr", "bad_sockaddr".to_string()),
                        ("addr_len", addrlen.to_string()),
                    ],
                );
            }

            if !helper::socket_exist(sock) {
                log::trace!("\tbad socket/");
                emu.regs_mut().rax = constants::ENOTSOCK;
            } else {
                emu.regs_mut().rax = len;
            }
        }

        constants::NR64_RECVFROM => {
            let sock = emu.regs().rdi;
            let buf = emu.regs().rsi;
            let len = emu.regs().rdx;
            let flags = emu.regs().r10;
            let sockaddr = emu.regs().r8;
            let addrlen = emu.regs().r9;

            if sockaddr != 0 && emu.maps.is_mapped(sockaddr) {
                let port: u16 = 8080;
                let incoming_ip: u32 = 0x11223344;

                emu.maps.write_word(sockaddr, 0x0002);
                emu.maps.write_word(sockaddr + 2, port.to_le()); //TODO: port should be the same than bind()
                emu.maps.write_dword(sockaddr + 4, incoming_ip);
            }

            trace_socketcall64_args(
                emu,
                "recvfrom",
                &[
                    ("sock", sock.to_string()),
                    ("buf", buf.to_string()),
                    ("len", len.to_string()),
                    ("flags", flags.to_string()),
                    ("addr", format!("0x{sockaddr:x}")),
                    ("addr_len", addrlen.to_string()),
                ],
            );

            if !helper::socket_exist(sock) {
                log::trace!("\tbad socket/");
                emu.regs_mut().rax = constants::ENOTSOCK;
            } else {
                emu.regs_mut().rax = len; //TODO: avoid loops
            }
        }

        constants::NR64_SHUTDOWN => {
            trace_socketcall64(emu, "shutdown");
            //endpoint::sock_close();
        }

        constants::NR64_SETSOCKOPT => {
            trace_socketcall64(emu, "setsockopt");
        }

        constants::NR64_GETSOCKOPT => {
            trace_socketcall64(emu, "getsockopt");
        }

        constants::NR64_SENDMSG => {
            trace_socketcall64(emu, "sendmsg");
        }

        constants::NR64_RECVMSG => {
            trace_socketcall64(emu, "recvmsg");
        }

        constants::NR64_ACCEPT4 => {
            trace_socketcall64(emu, "accept4");
        }

        constants::NR64_RECVMMSG => {
            trace_socketcall64(emu, "recvmmsg");
        }

        constants::NR64_SENDMMSG => {
            trace_socketcall64(emu, "sendmmsg");
        }

        constants::NR64_ARCH_PRCTL => {
            let mode = emu.regs().rdi;
            let ptr = emu.regs().rsi;
            emu.regs_mut().rax = 0;
            let mut op: String = "unimplemented operation".to_string();

            match mode {
                constants::ARCH_SET_GS => {
                    op = "set gs".to_string();
                    emu.regs_mut().gs = emu
                        .maps
                        .read_qword(ptr)
                        .expect("kernel64 cannot read ptr for set gs");
                }
                constants::ARCH_SET_FS => {
                    op = "set fs".to_string();
                    emu.regs_mut().fs = emu
                        .maps
                        .read_qword(ptr)
                        .expect("kernel64 cannot read ptr for set fs");
                }
                constants::ARCH_GET_FS => {
                    op = "get fs".to_string();
                    emu.maps.write_qword(ptr, emu.regs().fs);
                }
                constants::ARCH_GET_GS => {
                    op = "get gs".to_string();
                    emu.maps.write_qword(ptr, emu.regs().gs);
                }
                _ => {}
            }

            trace_syscall64_args(emu, "arch_prctl", &[("op", op)]);
        }

        constants::NR64_UNAME => {
            emu.regs_mut().rax = 0;
            let ptr = emu.regs().rdi;

            if emu.maps.is_valid_ptr(ptr) {
                emu.maps.write_bytes(ptr, &constants::UTSNAME);
                emu.regs_mut().rax = 0;
            } else {
                emu.regs_mut().rax = constants::EINVAL;
            }

            trace_syscall64_args(emu, "uname", &[("buf", format!("0x{ptr:x}"))]);
        }

        constants::NR64_ACCESS => {
            let filename = emu.maps.read_string(emu.regs().rdi);

            trace_syscall64_args(emu, "access", &[("path", filename.clone())]);

            if filename == "/etc/ld.so.preload" {
                emu.regs_mut().rax = constants::ENOENT;
            } else {
                emu.regs_mut().rax = 0;
            }
        }

        constants::NR64_MUNMAP => {
            let addr = emu.regs().rdi;
            let sz = emu.regs().rsi;

            emu.maps.dealloc(addr);

            trace_syscall64_args(
                emu,
                "munmap",
                &[("addr", format!("0x{addr:x}")), ("len", sz.to_string())],
            );

            emu.regs_mut().rax = 0;
        }

        constants::NR64_MMAP => {
            let mut addr = emu.regs().rdi;
            let mut sz = emu.regs().rsi;
            let prot = emu.regs().rdx;
            let flags = emu.regs().r10;
            let fd = emu.regs().r8;
            let off = emu.regs().r9;

            if addr == 0 || emu.maps.is_mapped(addr) {
                if sz > 0xffffff {
                    log::trace!("/!\\ Warning trying to allocate {} bytes", sz);
                    sz = 0xffffff;
                }
                addr = emu
                    .maps
                    .lib64_alloc(sz)
                    .expect("syscall64 mmap cannot alloc");
            }

            let map = emu
                .maps
                .create_map(
                    &format!("mmap_{:x}", addr),
                    addr,
                    sz,
                    Permission::from_bits(prot as u8),
                )
                .expect("cannot create mmap map");

            if helper::handler_exist(fd) {
                let filepath = helper::handler_get_uri(fd);
                if filepath.contains(".so") {
                    let mut lib_buff: Vec<u8> = Vec::new();

                    //log::trace!("opening lib: {}", filepath);
                    match File::open(&filepath) {
                        Ok(f) => {
                            let len = f.metadata().unwrap().len();
                            let mut reader = BufReader::new(&f);
                            reader
                                .seek(SeekFrom::Start(off))
                                .expect("mmap offset out of file");
                            reader
                                .read_to_end(&mut lib_buff)
                                .expect("kernel64 cannot load dynamic library");
                            f.sync_all();

                            //log::trace!("readed a lib with sz: {} buff:0x{:x}", lib_buff.len(), addr);

                            let map = emu
                                .maps
                                .get_mem_by_addr_mut(addr)
                                .expect("buffer send to read syscall point to no map");

                            // does the file data bypasses mem end?
                            let mem_end = map.get_base() + map.size() as u64 - 1;
                            let buff_end = addr + lib_buff.len() as u64 - 1;
                            if buff_end > mem_end {
                                let overflow = buff_end - mem_end;
                                lib_buff = lib_buff[0..lib_buff.len() - overflow as usize].to_vec();
                            }

                            emu.maps.write_bytes(addr, &lib_buff);
                            emu.regs_mut().rax = sz;
                        }
                        Err(_) => {
                            log::trace!("file not found");
                            emu.regs_mut().rax = 0;
                        }
                    };
                }
            }

            trace_syscall64_args(
                emu,
                "mmap",
                &[
                    ("fd", (fd as i32).to_string()),
                    ("len", sz.to_string()),
                    ("off", off.to_string()),
                    ("addr", format!("0x{addr:x}")),
                ],
            );

            emu.regs_mut().rax = addr;
        }

        constants::NR64_FSTAT => {
            let fd = emu.regs().rdi;
            let stat_ptr = emu.regs().rsi;
            let mut stat = structures::Stat::fake();

            if helper::handler_exist(fd) {
                let filepath = helper::handler_get_uri(fd);
                let path = Path::new(&filepath);
                let metadata = stdfs::metadata(path)
                    .expect("this file should exist because was opened by kernel64");
                let file_size = metadata.len();
                stat.size = file_size as i64;
            }

            if stat_ptr > 0 && emu.maps.is_mapped(stat_ptr) {
                println!("saving stat at 0x{:x}", stat_ptr);
                stat.save(stat_ptr, &mut emu.maps);
            }

            trace_syscall64_args(
                emu,
                "fstat",
                &[("fd", fd.to_string()), ("stat", format!("0x{stat_ptr:x}"))],
            );

            emu.regs_mut().rax = 0;
        }

        constants::NR64_STAT => {
            let filename_ptr = emu.regs().rdi;
            let stat_ptr = emu.regs().rsi;
            let filename = emu.maps.read_string(filename_ptr);

            let stat = structures::Stat::fake();
            //let path = Path::new(&filename);
            //let metadata =
            //    stdfs::metadata(path).expect("this file should exist because was opened by kernel64");
            //let file_size = metadata.len();
            //stat.size = file_size as i64;
            if stat_ptr > 0 && emu.maps.is_mapped(stat_ptr) {
                stat.save(stat_ptr, &mut emu.maps);
            }

            trace_syscall64_args(
                emu,
                "stat",
                &[
                    ("path", filename),
                    ("stat", format!("0x{stat_ptr:x}")),
                ],
            );

            emu.regs_mut().rax = 0;
        }

        constants::NR64_READLINK => {
            let link_ptr = emu.regs().rdi;
            let buff = emu.regs().rsi;
            let buffsz = emu.regs().rdx;

            let link = emu.maps.read_string(link_ptr);
            trace_syscall64_args(
                emu,
                "readlink",
                &[
                    ("path", link.clone()),
                    ("buf", format!("0x{buff:x}")),
                    ("len", buffsz.to_string()),
                ],
            );
            let sym_link_dest = match stdfs::read_link(&link) {
                Ok(link) => link,
                Err(_) => {
                    emu.regs_mut().rax = 0xffffffffffffffff;
                    return;
                }
            };

            emu.maps.write_string(buff, sym_link_dest.to_str().unwrap());

            emu.regs_mut().rax = sym_link_dest.as_os_str().len() as u64;
        }

        constants::NR64_MPROTECT => {
            let addr = emu.regs().rdi;
            let sz = emu.regs().rsi;
            let prot = emu.regs().rdx;

            /*if emu.maps.is_mapped(addr) {
                emu.regs_mut().rax = 0;
            } else {
                emu.regs_mut().rax = 0xffffffff_ffffffff;
            }*/
            emu.regs_mut().rax = 0;

            trace_syscall64_args(
                emu,
                "mprotect",
                &[
                    ("addr", format!("0x{addr:x}")),
                    ("len", sz.to_string()),
                    ("prot", prot.to_string()),
                    ("result", format!("0x{:x}", emu.regs().rax)),
                ],
            );
        }

        constants::NR64_NANOSLEEP => super::memory::handle_syscall64_nanosleep(emu),

        constants::NR64_MREMAP => super::memory::handle_syscall64_mremap(emu),

        _ => {
            const SYS64_SYSCALL_NAMES: &[&str] = syscall_names![
                read,
                write,
                open,
                close,
                stat,
                fstat,
                lstat,
                poll,
                lseek,
                mmap,
                mprotect,
                munmap,
                brk,
                rt_sigaction,
                rt_sigprocmask,
                rt_sigreturn,
                ioctl,
                pread64,
                pwrite64,
                readv,
                writev,
                access,
                pipe,
                select,
                sched_yield,
                mremap,
                msync,
                mincore,
                madvise,
                shmget,
                shmat,
                shmctl,
                dup,
                dup2,
                pause,
                nanosleep,
                getitimer,
                alarm,
                setitimer,
                getpid,
                sendfile,
                socket,
                connect,
                accept,
                sendto,
                recvfrom,
                sendmsg,
                recvmsg,
                shutdown,
                bind,
                listen,
                getsockname,
                getpeername,
                socketpair,
                setsockopt,
                getsockopt,
                clone,
                fork,
                vfork,
                execve,
                exit,
                wait4,
                kill,
                uname,
                semget,
                semop,
                semctl,
                shmdt,
                msgget,
                msgsnd,
                msgrcv,
                msgctl,
                fcntl,
                flock,
                fsync,
                fdatasync,
                truncate,
                ftruncate,
                getdents,
                getcwd,
                chdir,
                fchdir,
                rename,
                mkdir,
                rmdir,
                creat,
                link,
                unlink,
                symlink,
                readlink,
                chmod,
                fchmod,
                chown,
                fchown,
                lchown,
                umask,
                gettimeofday,
                getrlimit,
                getrusage,
                sysinfo,
                times,
                ptrace,
                getuid,
                syslog,
                getgid,
                setuid,
                setgid,
                geteuid,
                getegid,
                setpgid,
                getppid,
                getpgrp,
                setsid,
                setreuid,
                setregid,
                getgroups,
                setgroups,
                setresuid,
                getresuid,
                setresgid,
                getresgid,
                getpgid,
                setfsuid,
                setfsgid,
                getsid,
                capget,
                capset,
                rt_sigpending,
                rt_sigtimedwait,
                rt_sigqueueinfo,
                rt_sigsuspend,
                sigaltstack,
                utime,
                mknod,
                uselib,
                personality,
                ustat,
                statfs,
                fstatfs,
                sysfs,
                getpriority,
                setpriority,
                sched_setparam,
                sched_getparam,
                sched_setscheduler,
                sched_getscheduler,
                sched_get_priority_max,
                sched_get_priority_min,
                sched_rr_get_interval,
                mlock,
                munlock,
                mlockall,
                munlockall,
                vhangup,
                modify_ldt,
                pivot_root,
                _sysctl,
                prctl,
                arch_prctl,
                adjtimex,
                setrlimit,
                chroot,
                sync,
                acct,
                settimeofday,
                mount,
                umount2,
                swapon,
                swapoff,
                reboot,
                sethostname,
                setdomainname,
                iopl,
                ioperm,
                create_module,
                init_module,
                delete_module,
                get_kernel_syms,
                query_module,
                quotactl,
                nfsservctl,
                getpmsg,
                putpmsg,
                afs_syscall,
                tuxcall,
                security,
                gettid,
                readahead,
                setxattr,
                lsetxattr,
                fsetxattr,
                getxattr,
                lgetxattr,
                fgetxattr,
                listxattr,
                llistxattr,
                flistxattr,
                removexattr,
                lremovexattr,
                fremovexattr,
                tkill,
                time,
                futex,
                sched_setaffinity,
                sched_getaffinity,
                set_thread_area,
                io_setup,
                io_destroy,
                io_getevents,
                io_submit,
                io_cancel,
                get_thread_area,
                lookup_dcookie,
                epoll_create,
                epoll_ctl_old,
                epoll_wait_old,
                remap_file_pages,
                getdents64,
                set_tid_address,
                restart_syscall,
                semtimedop,
                fadvise64,
                timer_create,
                timer_settime,
                timer_gettime,
                timer_getoverrun,
                timer_delete,
                clock_settime,
                clock_gettime,
                clock_getres,
                clock_nanosleep,
                exit_group,
                epoll_wait,
                epoll_ctl,
                tgkill,
                utimes,
                vserver,
                mbind,
                set_mempolicy,
                get_mempolicy,
                mq_open,
                mq_unlink,
                mq_timedsend,
                mq_timedreceive,
                mq_notify,
                mq_getsetattr,
                kexec_load,
                waitid,
                add_key,
                request_key,
                keyctl,
                ioprio_set,
                ioprio_get,
                inotify_init,
                inotify_add_watch,
                inotify_rm_watch,
                migrate_pages,
                openat,
                mkdirat,
                mknodat,
                fchownat,
                futimesat,
                newfstatat,
                unlinkat,
                renameat,
                linkat,
                symlinkat,
                readlinkat,
                fchmodat,
                faccessat,
                pselect6,
                ppoll,
                unshare,
                set_robust_list,
                get_robust_list,
                splice,
                tee,
                sync_file_range,
                vmsplice,
                move_pages,
                utimensat,
                epoll_pwait,
                signalfd,
                timerfd_create,
                eventfd,
                fallocate,
                timerfd_settime,
                timerfd_gettime,
                accept4,
                signalfd4,
                eventfd2,
                epoll_create1,
                dup3,
                pipe2,
                inotify_init1,
                preadv,
                pwritev,
                rt_tgsigqueueinfo,
                perf_event_open,
                recvmmsg,
                fanotify_init,
                fanotify_mark,
                prlimit64,
                name_to_handle_at,
                open_by_handle_at,
                clock_adjtime,
                syncfs,
                sendmmsg,
                setns,
                getcpu,
                process_vm_readv,
                process_vm_writev,
                kcmp,
                finit_module,
                sched_setattr,
                sched_getattr,
                renameat2,
                seccomp,
                getrandom,
                memfd_create,
                kexec_file_load,
                bpf,
                execveat,
                userfaultfd,
                membarrier,
                mlock2,
                copy_file_range,
                preadv2,
                pwritev2,
                pkey_mprotect,
                pkey_alloc,
                pkey_free,
                statx,
                io_pgetevents,
                rseq,
                pidfd_send_signal,
                io_uring_setup,
                io_uring_enter,
                io_uring_register,
                open_tree,
                move_mount,
                fsopen,
                fsconfig,
                fsmount,
                fspick,
                pidfd_open,
                clone3,
                close_range,
                openat2,
                pidfd_getfd,
                faccessat2,
                process_madvise,
                epoll_pwait2,
                mount_setattr,
                quotactl_fd,
                landlock_create_ruleset,
                landlock_add_rule,
                landlock_restrict_self,
                memfd_secret,
                process_mrelease,
            ];

            if emu.regs().rax >= SYS64_SYSCALL_NAMES.len() as u64 {
                trace_bad_syscall64(emu, emu.regs().rax);
            } else {
                trace_legacy_syscall64(emu, SYS64_SYSCALL_NAMES[emu.regs().rax as usize]);
            }
        }
    }
}
