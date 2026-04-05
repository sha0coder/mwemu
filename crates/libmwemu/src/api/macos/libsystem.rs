use crate::emu::Emu;

pub fn gateway(symbol: &str, emu: &mut Emu) {
    match symbol {
        "_printf" | "printf" => api_printf(emu),
        "_fprintf" | "fprintf" => api_fprintf(emu),
        "_sprintf" | "sprintf" => api_sprintf(emu),
        "_snprintf" | "snprintf" => api_snprintf(emu),
        "_puts" | "puts" => api_puts(emu),
        "_putchar" | "putchar" => api_putchar(emu),
        "_exit" | "exit" | "__exit" => api_exit(emu),
        "_abort" | "abort" => api_abort(emu),
        "_malloc" | "malloc" => api_malloc(emu),
        "_calloc" | "calloc" => api_calloc(emu),
        "_realloc" | "realloc" => api_realloc(emu),
        "_free" | "free" => api_free(emu),
        "_atexit" | "atexit" => api_atexit(emu),
        "_write" | "write" => api_write(emu),
        "_read" | "read" => api_read(emu),
        "_open" | "open" => api_open(emu),
        "_close" | "close" => api_close(emu),
        "_memcpy" | "memcpy" => api_memcpy(emu),
        "_memmove" | "memmove" => api_memmove(emu),
        "_memset" | "memset" => api_memset(emu),
        "_memcmp" | "memcmp" => api_memcmp(emu),
        "_strlen" | "strlen" => api_strlen(emu),
        "_strcmp" | "strcmp" => api_strcmp(emu),
        "_strncmp" | "strncmp" => api_strncmp(emu),
        "_strcpy" | "strcpy" => api_strcpy(emu),
        "_strncpy" | "strncpy" => api_strncpy(emu),
        "_strcat" | "strcat" => api_strcat(emu),
        "_strchr" | "strchr" => api_strchr(emu),
        "_strrchr" | "strrchr" => api_strrchr(emu),
        "_strstr" | "strstr" => api_strstr(emu),
        "_strdup" | "strdup" => api_strdup(emu),
        "_mmap" | "mmap" => api_mmap(emu),
        "_munmap" | "munmap" => api_munmap(emu),
        _ => {
            log::warn!("libsystem: unimplemented API {}", symbol);
            todo!("libSystem API: {}", symbol);
        }
    }
}

fn api_printf(emu: &mut Emu) {
    let fmt_addr = emu.regs_aarch64().x[0];
    let fmt = emu.maps.read_string(fmt_addr);
    log::info!(
        "{}** {} macOS API printf(\"{}\") {}",
        emu.colors.light_red, emu.pos, fmt, emu.colors.nc
    );
    emu.regs_aarch64_mut().x[0] = fmt.len() as u64;
}

fn api_fprintf(emu: &mut Emu) {
    let _stream = emu.regs_aarch64().x[0];
    let fmt_addr = emu.regs_aarch64().x[1];
    let fmt = emu.maps.read_string(fmt_addr);
    log::info!(
        "{}** {} macOS API fprintf(\"{}\") {}",
        emu.colors.light_red, emu.pos, fmt, emu.colors.nc
    );
    emu.regs_aarch64_mut().x[0] = fmt.len() as u64;
}

fn api_sprintf(emu: &mut Emu) {
    todo!("sprintf");
}

fn api_snprintf(emu: &mut Emu) {
    todo!("snprintf");
}

fn api_puts(emu: &mut Emu) {
    let s_addr = emu.regs_aarch64().x[0];
    let s = emu.maps.read_string(s_addr);
    log::info!(
        "{}** {} macOS API puts(\"{}\") {}",
        emu.colors.light_red, emu.pos, s, emu.colors.nc
    );
    emu.regs_aarch64_mut().x[0] = 0;
}

fn api_putchar(emu: &mut Emu) {
    let c = emu.regs_aarch64().x[0] as u8 as char;
    log::info!(
        "{}** {} macOS API putchar('{}') {}",
        emu.colors.light_red, emu.pos, c, emu.colors.nc
    );
    emu.regs_aarch64_mut().x[0] = c as u64;
}

fn api_exit(emu: &mut Emu) {
    let status = emu.regs_aarch64().x[0];
    log::info!(
        "{}** {} macOS API exit({}) {}",
        emu.colors.light_red, emu.pos, status, emu.colors.nc
    );
    emu.stop();
}

fn api_abort(emu: &mut Emu) {
    log::info!(
        "{}** {} macOS API abort() {}",
        emu.colors.light_red, emu.pos, emu.colors.nc
    );
    emu.stop();
}

fn api_malloc(emu: &mut Emu) {
    let size = emu.regs_aarch64().x[0];
    log::info!(
        "{}** {} macOS API malloc({}) {}",
        emu.colors.light_red, emu.pos, size, emu.colors.nc
    );
    todo!("malloc({})", size);
}

fn api_calloc(emu: &mut Emu) { todo!("calloc"); }
fn api_realloc(emu: &mut Emu) { todo!("realloc"); }

fn api_free(emu: &mut Emu) {
    let _ptr = emu.regs_aarch64().x[0];
    log::info!(
        "{}** {} macOS API free(0x{:x}) {}",
        emu.colors.light_red, emu.pos, _ptr, emu.colors.nc
    );
    // no-op for now
}

fn api_atexit(emu: &mut Emu) {
    log::info!(
        "{}** {} macOS API atexit() {}",
        emu.colors.light_red, emu.pos, emu.colors.nc
    );
    emu.regs_aarch64_mut().x[0] = 0;
}

fn api_write(emu: &mut Emu) {
    let fd = emu.regs_aarch64().x[0];
    let buf = emu.regs_aarch64().x[1];
    let count = emu.regs_aarch64().x[2];
    let s = emu.maps.read_string(buf);
    log::info!(
        "{}** {} macOS API write(fd={}, \"{}\", {}) {}",
        emu.colors.light_red, emu.pos, fd, s, count, emu.colors.nc
    );
    emu.regs_aarch64_mut().x[0] = count;
}

fn api_read(emu: &mut Emu) { todo!("read"); }
fn api_open(emu: &mut Emu) { todo!("open"); }

fn api_close(emu: &mut Emu) {
    let fd = emu.regs_aarch64().x[0];
    log::info!(
        "{}** {} macOS API close(fd={}) {}",
        emu.colors.light_red, emu.pos, fd, emu.colors.nc
    );
    emu.regs_aarch64_mut().x[0] = 0;
}

fn api_memcpy(emu: &mut Emu) { todo!("memcpy"); }
fn api_memmove(emu: &mut Emu) { todo!("memmove"); }
fn api_memset(emu: &mut Emu) { todo!("memset"); }
fn api_memcmp(emu: &mut Emu) { todo!("memcmp"); }

fn api_strlen(emu: &mut Emu) {
    let s_addr = emu.regs_aarch64().x[0];
    let s = emu.maps.read_string(s_addr);
    emu.regs_aarch64_mut().x[0] = s.len() as u64;
}

fn api_strcmp(emu: &mut Emu) { todo!("strcmp"); }
fn api_strncmp(emu: &mut Emu) { todo!("strncmp"); }
fn api_strcpy(emu: &mut Emu) { todo!("strcpy"); }
fn api_strncpy(emu: &mut Emu) { todo!("strncpy"); }
fn api_strcat(emu: &mut Emu) { todo!("strcat"); }
fn api_strchr(emu: &mut Emu) { todo!("strchr"); }
fn api_strrchr(emu: &mut Emu) { todo!("strrchr"); }
fn api_strstr(emu: &mut Emu) { todo!("strstr"); }
fn api_strdup(emu: &mut Emu) { todo!("strdup"); }
fn api_mmap(emu: &mut Emu) { todo!("mmap"); }
fn api_munmap(emu: &mut Emu) { todo!("munmap"); }
