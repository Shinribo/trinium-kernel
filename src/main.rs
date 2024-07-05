#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![allow(dead_code)]
#![feature(abi_x86_interrupt, inline_const)]
#![feature(associated_type_bounds)]
#![feature(allocator_api)]
#![feature(btreemap_alloc)]
#![feature(let_chains)]
#![feature(generic_const_exprs)]

extern crate alloc;
extern crate x86_64;

mod bal;
mod hal;
mod heap;
mod panic_handler;
mod pmm;
mod vmm;

//TODO check cpuid for required features and panic if they are missing (PCID......)

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default

    //Check for required Hardware Features
    //Setup Initial Numa Aware PMM
    //Setup Allocators
    //Setup all PMMS
    //Claim Paging Tables
    //Create Core-local Structs
    //Create Process and Thread Structs
    //Jump other cores to

    loop {}
}
