// tracing.rs

use std::cell::UnsafeCell;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::emu::Emu;

// Fixed-size trace record: 144 bytes per instruction
// This covers all general purpose registers + RIP + instruction counter
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct TraceRecord {
    pub instruction_count: u64,  // 8 bytes
    pub rip: u64,                // 8 bytes
    pub rflags: u64,              // 8 bytes
    pub rax: u64,                 // 8 bytes
    pub rbx: u64,                 // 8 bytes
    pub rcx: u64,                 // 8 bytes
    pub rdx: u64,                 // 8 bytes
    pub rsi: u64,                 // 8 bytes
    pub rdi: u64,                 // 8 bytes
    pub rbp: u64,                 // 8 bytes
    pub rsp: u64,                 // 8 bytes
    pub r8: u64,                  // 8 bytes
    pub r9: u64,                  // 8 bytes
    pub r10: u64,                 // 8 bytes
    pub r11: u64,                 // 8 bytes
    pub r12: u64,                 // 8 bytes
    pub r13: u64,                 // 8 bytes
    pub r14: u64,                 // 8 bytes
    pub r15: u64,                 // 8 bytes
    // Total: 152 bytes
}

impl TraceRecord {
    pub fn capture(emu: &Emu, instruction_count: u64) -> Self {
        Self {
            instruction_count,
            rip: emu.regs().rip,
            rflags: emu.flags().dump() as u64, // TODO: u32?
            rax: emu.regs().rax,
            rbx: emu.regs().rbx,
            rcx: emu.regs().rcx,
            rdx: emu.regs().rdx,
            rsi: emu.regs().rsi,
            rdi: emu.regs().rdi,
            rbp: emu.regs().rbp,
            rsp: emu.regs().rsp,
            r8: emu.regs().r8,
            r9: emu.regs().r9,
            r10: emu.regs().r10,
            r11: emu.regs().r11,
            r12: emu.regs().r12,
            r13: emu.regs().r13,
            r14: emu.regs().r14,
            r15: emu.regs().r15,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self as *const Self as *const u8,
                std::mem::size_of::<Self>()
            )
        }
    }
}

// Thread-local trace writer
thread_local! {
    // The actual writer - None if tracing is disabled
    static TRACE_WRITER: UnsafeCell<Option<BufWriter<File>>> = UnsafeCell::new(None);
    
    // Counter for periodic flushing
    static TRACE_RECORDS_WRITTEN: UnsafeCell<u64> = UnsafeCell::new(0);
    
    // Reusable buffer for the trace record to avoid allocations
    static TRACE_RECORD_BUFFER: UnsafeCell<TraceRecord> = UnsafeCell::new(unsafe { std::mem::zeroed() });
}

pub fn init_tracing(path: impl AsRef<Path>) -> std::io::Result<()> {
    let file = File::create(path)?;
    // 16MB buffer for maximum efficiency
    let writer = BufWriter::with_capacity(16 * 1024 * 1024, file);
    
    TRACE_WRITER.with(|w| unsafe {
        *w.get() = Some(writer);
    });
    
    log::info!("📝 Trace logging initialized");
    Ok(())
}

#[inline(always)]
pub fn trace_instruction(emu: &Emu, instruction_count: u64) {
    TRACE_WRITER.with(|writer_cell| {
        let writer = unsafe { &mut *writer_cell.get() };
        if let Some(w) = writer {
            // Capture directly into our reusable buffer
            TRACE_RECORD_BUFFER.with(|rec_cell| {
                let record = unsafe { &mut *rec_cell.get() };
                *record = TraceRecord::capture(emu, instruction_count);
                
                // Write the record
                if let Err(e) = w.write_all(record.as_bytes()) {
                    log::error!("Failed to write trace record: {}", e);
                    return;
                }
            });
            
            // Update counter and flush periodically
            TRACE_RECORDS_WRITTEN.with(|count_cell| {
                let count = unsafe { &mut *count_cell.get() };
                *count += 1;
                
                // Flush every 1M records to avoid losing too much data if we crash
                if *count % 1_000_000 == 0 {
                    if let Err(e) = w.flush() {
                        log::error!("Failed to flush trace: {}", e);
                    } else {
                        log::debug!("Trace: Flushed {} records", count);
                    }
                }
            });
        }
    });
}

pub fn flush_trace() {
    TRACE_WRITER.with(|writer_cell| {
        let writer = unsafe { &mut *writer_cell.get() };
        if let Some(w) = writer {
            if let Err(e) = w.flush() {
                log::error!("Failed to flush trace: {}", e);
            } else {
                TRACE_RECORDS_WRITTEN.with(|count_cell| {
                    let count = unsafe { *count_cell.get() };
                    log::info!("📝 Flushed {} trace records", count);
                });
            }
        }
    });
}
