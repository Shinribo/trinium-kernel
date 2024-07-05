#[cfg_attr(target_arch = "x86_64", path = "x86_64/mod.rs")]
mod arch;

pub mod cpu;
pub mod cpuid;
pub mod interrupt;
pub mod irqlvmutex;
pub mod memory;
///! Wrapper of varying thicknes around the arch module that implements/wraps needed stuff and ensures that no code outside of the hal mod needs to access the arch mod
///! Intention is that a someone who implements a new arch can see what is missing
pub mod paging;
