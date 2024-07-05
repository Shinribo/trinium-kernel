use crate::hal::arch::paging::init_paging;

pub(in crate::hal) mod cpu;
mod cpuid;
mod gdt;
mod idt;
pub(in crate::hal) mod memory;
pub(in crate::hal) mod paging;

///Called early in OS Boot
///No Heap and most other OS Services are not availible
pub fn init_arch() {
    compile_error!("TODO");
    init_paging();
}

///Called when OS is finishing its boot
///Nearly all OS-Services are availible
pub fn init_arch_finalization() {
    compile_error!("TODO");
}
