use bit_field::BitField;
use core::arch::asm;

use crate::hal::memory::VirtLv1PageAddress;

use super::cpuid;

//4K
pub fn get_lv1_page_size_mask() -> u64 {
    0xFFFF_FFFF_FFFF_F000u64
}

//2M
pub fn get_lv2_page_size_mask() -> u64 {
    0xFFFF_FFFF_FFE0_0000u64
}

//1G
pub fn get_lv3_page_size_mask() -> u64 {
    0xFFFF_FFFF_C000_0000u64
}

pub fn get_max_supported_phy_address_as_bit_mask() -> u64 {
    (!0x0u64) >> (64 - cpuid::physical_address_bit_size())
}

pub fn get_max_supported_virt_address_as_bit_mask() -> u64 {
    let mut data: u64;
    asm!(
        "mov {}, cr4",
        out(reg) data,
    );

    //57 Bits VAS are used,
    //5 Level Paging
    if data.get_bit(12) {
        return 0x1FF_FFFF_FFFF_FFFFu64;
    }

    //4 Level Paging
    0xFFFF_FFFF_FFFFu64
}

pub fn get_cannonical_bit_number() -> Option<u8> {
    //5 Level Paging
    if get_max_supported_virt_address_as_bit_mask() == 0x1FF_FFFF_FFFF_FFFFu64 {
        return Some(56);
    }

    Some(47)
}

//used as the HHDM Base if KASLR is not used
pub fn get_lowest_higher_half_address() -> VirtLv1PageAddress {}
