use lazy_static::lazy_static;
use spin::mutex::Mutex;

use crate::hal::memory::{PhysLv1PageAddress, PhysLv2PageAddress, PhysLv3PageAddress};

lazy_static! {
    static ref PMM_STATIC: Mutex<PhysLv1PageAddress> = Mutex::new(init_pmm());
}

compile_error!("Maybe split pmm init into two phases: on boot alloc a small amount of memory for datastructures and use that to manage like 10MB and once ACPI is up set up NUMA Aware Structures");

fn init_pmm() -> PhysLv1PageAddress {
    compile_error!("TODO");
}

//returns a page
pub fn alloc_lv1(zeroed: bool) -> PhysLv1PageAddress {
    compile_error!("TODO");
}

pub fn alloc_lv2(zeroed: bool) -> PhysLv2PageAddress {
    compile_error!("TODO");
}

pub fn alloc_lv3(zeroed: bool) -> PhysLv3PageAddress {
    compile_error!("TODO");
}

//tells the pmm that the give page is allocated again
pub fn multi_alloc_lv1(page: PhysLv1PageAddress) {
    compile_error!("TODO");
}

pub fn multi_alloc_lv2(page: PhysLv2PageAddress) {
    compile_error!("TODO");
}

pub fn multi_alloc_lv3(page: PhysLv3PageAddress) {
    compile_error!("TODO");
}

//frees the give page when the last user of the page calls this function
pub fn free_lv1(page: PhysLv1PageAddress) {
    compile_error!("TODO");
}

pub fn free_lv2(page: PhysLv2PageAddress) {
    compile_error!("TODO");
}

pub fn free_lv3(page: PhysLv3PageAddress) {
    compile_error!("TODO");
}
