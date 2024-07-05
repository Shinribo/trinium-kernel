///Bootloader Abstraction Layer

//#[cfg_attr(package.bootloader = "limine", path = "limine/mod.rs")]
#[path = "limine/mod.rs"]
mod bootloader;

pub mod hhdm;
