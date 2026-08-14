
use anyhow::anyhow;
use core::slice;
use multiboot::information::{MemoryManagement, Multiboot, PAddr, SIGNATURE_EAX};

static mut MEMORY_MANAGEMENT: Mem = Mem;

pub fn get_mbi<'a>(magic: u32, ptr: u32) -> anyhow::Result<Multiboot<'a, 'static>> { panic!("STUB: not implemented") }

struct Mem;

impl MemoryManagement for Mem {
    unsafe fn paddr_to_slice(&self, addr: PAddr, size: usize) -> Option<&'static [u8]> { panic!("STUB: not implemented") }

    unsafe fn allocate(&mut self, _length: usize) -> Option<(PAddr, &mut [u8])> { panic!("STUB: not implemented") }

    unsafe fn deallocate(&mut self, addr: PAddr) { panic!("STUB: not implemented") }
}
