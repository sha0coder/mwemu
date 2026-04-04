use crate::emu::Emu;

pub fn gateway(symbol: &str, emu: &mut Emu) {
    match symbol {
        "printf" => api_printf(emu),
        "fprintf" => api_fprintf(emu),
        "puts" => api_puts(emu),
        "putchar" => api_putchar(emu),
        "exit" => api_exit(emu),
        "_exit" => api_exit(emu),
        "abort" => api_abort(emu),
        "malloc" => api_malloc(emu),
        "calloc" => api_calloc(emu),
        "realloc" => api_realloc(emu),
        "free" => api_free(emu),
        "write" => api_write(emu),
        "read" => api_read(emu),
        "open" => api_open(emu),
        "close" => api_close(emu),
        "strlen" => api_strlen(emu),
        "memcpy" => api_memcpy(emu),
        "memset" => api_memset(emu),
        "mmap" => api_mmap(emu),
        "munmap" => api_munmap(emu),
        _ => {
            log::warn!("linuxapi libc: unimplemented API {}", symbol);
            todo!("Linux libc API: {}", symbol);
        }
    }
}

fn api_printf(emu: &mut Emu) {
    let fmt_addr = emu.regs_aarch64().x[0];
    let fmt = emu.maps.read_string(fmt_addr);
    log::info!(
        "{}** {} Linux API printf(\"{}\") {}",
        emu.colors.light_red, emu.pos, fmt, emu.colors.nc
    );
    emu.regs_aarch64_mut().x[0] = fmt.len() as u64;
}

fn api_fprintf(emu: &mut Emu) {
    let _stream = emu.regs_aarch64().x[0];
    let fmt_addr = emu.regs_aarch64().x[1];
    let fmt = emu.maps.read_string(fmt_addr);
    log::info!(
        "{}** {} Linux API fprintf(\"{}\") {}",
        emu.colors.light_red, emu.pos, fmt, emu.colors.nc
    );
    emu.regs_aarch64_mut().x[0] = fmt.len() as u64;
}

fn api_puts(emu: &mut Emu) {
    let s_addr = emu.regs_aarch64().x[0];
    let s = emu.maps.read_string(s_addr);
    log::info!(
        "{}** {} Linux API puts(\"{}\") {}",
        emu.colors.light_red, emu.pos, s, emu.colors.nc
    );
    emu.regs_aarch64_mut().x[0] = 0;
}

fn api_putchar(emu: &mut Emu) {
    let c = emu.regs_aarch64().x[0] as u8 as char;
    log::info!(
        "{}** {} Linux API putchar('{}') {}",
        emu.colors.light_red, emu.pos, c, emu.colors.nc
    );
    emu.regs_aarch64_mut().x[0] = c as u64;
}

fn api_exit(emu: &mut Emu) {
    let status = emu.regs_aarch64().x[0];
    log::info!(
        "{}** {} Linux API exit({}) {}",
        emu.colors.light_red, emu.pos, status, emu.colors.nc
    );
    emu.stop();
}

fn api_abort(emu: &mut Emu) {
    log::info!(
        "{}** {} Linux API abort() {}",
        emu.colors.light_red, emu.pos, emu.colors.nc
    );
    emu.stop();
}

fn api_malloc(emu: &mut Emu) {
    let size = emu.regs_aarch64().x[0];
    log::info!(
        "{}** {} Linux API malloc({}) {}",
        emu.colors.light_red, emu.pos, size, emu.colors.nc
    );
    todo!("malloc({})", size);
}

fn api_calloc(emu: &mut Emu) { todo!("calloc"); }
fn api_realloc(emu: &mut Emu) { todo!("realloc"); }

fn api_free(emu: &mut Emu) {
    let ptr = emu.regs_aarch64().x[0];
    log::info!(
        "{}** {} Linux API free(0x{:x}) {}",
        emu.colors.light_red, emu.pos, ptr, emu.colors.nc
    );
    // no-op for now
}

fn api_write(emu: &mut Emu) {
    let fd = emu.regs_aarch64().x[0];
    let buf = emu.regs_aarch64().x[1];
    let count = emu.regs_aarch64().x[2];
    let s = emu.maps.read_string(buf);
    log::info!(
        "{}** {} Linux API write(fd={}, \"{}\", {}) {}",
        emu.colors.light_red, emu.pos, fd, s, count, emu.colors.nc
    );
    emu.regs_aarch64_mut().x[0] = count;
}

fn api_read(emu: &mut Emu) { todo!("read"); }
fn api_open(emu: &mut Emu) { todo!("open"); }

fn api_close(emu: &mut Emu) {
    let fd = emu.regs_aarch64().x[0];
    log::info!(
        "{}** {} Linux API close(fd={}) {}",
        emu.colors.light_red, emu.pos, fd, emu.colors.nc
    );
    emu.regs_aarch64_mut().x[0] = 0;
}

fn api_strlen(emu: &mut Emu) {
    let s_addr = emu.regs_aarch64().x[0];
    let s = emu.maps.read_string(s_addr);
    emu.regs_aarch64_mut().x[0] = s.len() as u64;
}

fn api_memcpy(emu: &mut Emu) { todo!("memcpy"); }
fn api_memset(emu: &mut Emu) { todo!("memset"); }
fn api_mmap(emu: &mut Emu) { todo!("mmap"); }
fn api_munmap(emu: &mut Emu) { todo!("munmap"); }
