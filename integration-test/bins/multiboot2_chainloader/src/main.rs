#![no_main]
#![no_std]

mod loader;
mod multiboot;

extern crate alloc;

use integration_test_util::init_environment;

core::arch::global_asm!(include_str!("start.S"), options(att_syntax));

#[unsafe(no_mangle)]
fn rust_entry(multiboot_magic: u32, multiboot_hdr: *const u32) -> ! { panic!("STUB: not implemented") }
