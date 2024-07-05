use lazy_static::lazy_static;

use crate::hal::memory::VirtLv1PageAddress;

lazy_static! {
    pub static ref HHDM_OFFSET: VirtLv1PageAddress = super::bootloader::hhdm::get_hhdm_start();
}
