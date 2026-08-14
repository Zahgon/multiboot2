use alloc::boxed::Box;
use elf_rs::{ElfFile, ProgramHeaderEntry, ProgramType};
use multiboot2::{
    BootLoaderNameTag, CommandLineTag, MaybeDynSized, MemoryArea, MemoryAreaType, MemoryMapTag,
    ModuleTag, SmbiosTag,
};

pub fn load_module(mut modules: multiboot::information::ModuleIter) -> ! { panic!("STUB: not implemented") }

fn map_memory(ph: ProgramHeaderEntry) { panic!("STUB: not implemented") }
