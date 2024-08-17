use crate::hal::paging::PageAttributes;
use alloc::vec::Vec;

use crate::hal::{memory::*, paging::*};

pub const PML5ENTRYINDEXMASK: u64 = 0x1FF_0000_0000_0000;
pub const PML4ENTRYINDEXMASK: u64 = 0xFF80_0000_0000;
pub const PML3ENTRYINDEXMASK: u64 = 0x7F_C000_0000;
pub const PML2ENTRYINDEXMASK: u64 = 0x3FE0_0000;
pub const PML1ENTRYINDEXMASK: u64 = 0x1F_F000;

//Paging Bits
pub const PRESENT_BIT: usize = 0;
pub const READ_WRITE_BIT: usize = 1;
pub const USER_SUPERVISOR_BIT: usize = 2;
pub const PAGE_LEVEL_WRITETHROUGH_BIT: usize = 3;
pub const PAGE_LEVEL_CACHE_DISABLE_BIT: usize = 4;
pub const ACCESSED_BIT: usize = 5;
pub const DIRTY_BIT: usize = 6;
pub const GLOBAL_BIT: usize = 8;
pub const VALID_BIT: usize = 9; //Indicates that the given page is valid
pub const NO_EXECUTE_BIT: usize = 63;
pub const MEMORY_PROTECTION_KEY_START_BIT: usize = 59;
pub const MEMORY_PROTECTION_KEY_END_BIT: usize = 62;

pub const DEFAULTPHYSADDRESSMASK: u64 = 0xF_FFFF_FFFF_F000;
pub const LV2PHYSADDRESSMASK: u64 = 0xF_FFFF_FFE0_0000; //Used for the PML2 on 2MB Pages
pub const LV3PHYSADDRESSMASK: u64 = 0xF_FFFF_C000_0000; //Used for the PML3 on 1GB Pages

pub(super) fn init_paging() {
    compile_error!("TODO: SET PAT MSR");
}

pub(in crate::hal) unsafe fn map_slice_lv1_page(
    root: PageRoot,
    phys_pages_for_pt: &mut [PhysLv1PageAddress],
    phys_pages: &mut [PhysLv1PageAddress],
    virt_start_addr: VirtLv1PageAddress,
    attributes: PageAttributes,
) -> Result<(), PagingErros> {
    compile_error!("TODO")
}

pub(in crate::hal) unsafe fn map_slice_lv2_page(
    root: PageRoot,
    phys_pages_for_pt: &mut [PhysLv1PageAddress],
    phys_pages: &mut [PhysLv2PageAddress],
    virt_start_addr: VirtLv2PageAddress,
    attributes: PageAttributes,
) -> Result<(), PagingErros> {
    compile_error!("TODO")
}

pub(in crate::hal) unsafe fn map_slice_lv3_page(
    root: PageRoot,
    phys_pages_for_pt: &mut [PhysLv1PageAddress],
    phys_page: &mut [PhysLv3PageAddress],
    virt_start_addr: VirtLv3PageAddress,
    attributes: PageAttributes,
) -> Result<(), PagingErros> {
    compile_error!("TODO")
}

pub(in crate::hal) unsafe fn map_vec_lv1_page(
    root: PageRoot,
    phys_pages_for_pt: Vec<PhysLv1PageAddress>,
    phys_pages: Vec<PhysLv1PageAddress>,
    virt_start_addr: VirtLv1PageAddress,
    attributes: PageAttributes,
) -> Result<(), PagingErros> {
    compile_error!("TODO")
}

pub(in crate::hal) unsafe fn map_vec_lv2_page(
    root: PageRoot,
    phys_pages_for_pt: Vec<PhysLv1PageAddress>,
    phys_pages: Vec<PhysLv2PageAddress>,
    virt_start_addr: VirtLv2PageAddress,
    attributes: PageAttributes,
) -> Result<(), PagingErros> {
    compile_error!("TODO")
}

pub(in crate::hal) unsafe fn map_vec_lv3_page(
    root: PageRoot,
    phys_pages_for_pt: Vec<PhysLv1PageAddress>,
    phys_pages: Vec<PhysLv3PageAddress>,
    virt_start_addr: VirtLv3PageAddress,
    attributes: PageAttributes,
) -> Result<(), PagingErros> {
    compile_error!("TODO")
}

/// Performs necessary TLB Invalidations
pub(in crate::hal) unsafe fn unmap_lv1_page(
    root: PageRoot,
    start: VirtLv1PageAddress,
    number_of_pages: u64,
) -> Result<Vec<PhysLv1PageAddress>, PagingErros> {
    compile_error!("TODO")
}

/// Performs necessary TLB Invalidations
pub(in crate::hal) unsafe fn unmap_lv2_page(
    root: PageRoot,
    start: VirtLv2PageAddress,
    number_of_pages: u64,
) -> Result<Vec<PhysLv2PageAddress>, PagingErros> {
    compile_error!("TODO")
}

/// Performs necessary TLB Invalidations
pub(in crate::hal) unsafe fn unmap_lv3_page(
    root: PageRoot,
    start: VirtLv3PageAddress,
    number_of_pages: u64,
) -> Result<Vec<PhysLv3PageAddress>, PagingErros> {
    compile_error!("TODO")
}

///Updates the Attributes of the give Range
/// Performs necessary TLB Invalidations
pub(in crate::hal) unsafe fn update_page_attributes(
    root: PageRoot,
    virt_start_addr: VirtAddress,
    virt_end_addr: VirtAddress,
    attributes: PageAttributes,
) {
    compile_error!("TODO")
}

pub(in crate::hal) fn get_single_page(
    root: PageRoot,
    virt_start_addr: VirtAddress,
) -> Option<Page> {
    compile_error!("TODO")
}

pub(in crate::hal) fn get_vec_page(
    root: PageRoot,
    virt_start_addr: VirtAddress,
    virt_end_addr: VirtAddress,
) -> Vec<Option<Page>> {
    compile_error!("TODO")
}

pub(in crate::hal) fn needed_pt_pages_lv1(
    root: PageRoot,
    start: VirtLv1PageAddress,
    number_of_pages: u64,
) -> u64 {
    compile_error!("TODO")
}

pub(in crate::hal) fn needed_pt_pages_lv2(
    root: PageRoot,
    start: VirtLv2PageAddress,
    number_of_pages: u64,
) -> u64 {
    compile_error!("TODO")
}

pub(in crate::hal) fn needed_pt_pages_lv3(
    root: PageRoot,
    start: VirtLv3PageAddress,
    number_of_pages: u64,
) -> u64 {
    compile_error!("TODO")
}

///Return (PML5, PML4, PML3, PML2, PML1)
fn decode_virtual_address(
    address: VirtAddress,
) -> (
    u16,
    u16,
    u16,
    u16,
    u16,
) {
    (
        ((address.get_u64() >> 48) & 0x1FF) as u16,
        ((address.get_u64() >> 39) & 0x1FF) as u16,
        ((address.get_u64() >> 30) & 0x1FF) as u16,
        ((address.get_u64() >> 21) & 0x1FF) as u16,
        ((address.get_u64() >> 12) & 0x1FF) as u16,

    )
}

fn check_virt_space_is_free(start: VirtLv1PageAddress, number_of_pages: u64) -> bool {
    compile_error!("TODO");
}

fn decode_caching_bits(pwt: bool, pcd: bool, pat: bool) -> CachingMode {
    compile_error!("TODO");
}

///(PWT, PCD, PAT)
fn encode_caching_bits(caching_mode: CachingMode) -> (bool, bool, bool) {
    compile_error!("TODO");
}

pub struct PMLEntryIndex {
    index: u64,
}

impl PMLEntryIndex {
    
    pub fn new(index: u64) -> Option<PMLEntryIndex> {
        if index > 511 {
            return None;
        }

        Some(Self { index: index })

    }

    pub fn new_maskoff(index: u64) -> PMLEntryIndex {
        Self { index: index & 0x1FF }
    }

    pub unsafe fn new_unchecked(index: u64) -> PMLEntryIndex {
        Self { index: index }
    }

    pub fn get_index(&self) -> u64 {
        self.index
    }

}


//Should be guarded by a Mutex
pub struct ArchPageRoot {
    address: PhysLv1PageAddress,
}


mod pml4_5 {

    use super::{
        PhysLv1PageAddress, ACCESSED_BIT, PRESENT_BIT, READ_WRITE_BIT, USER_SUPERVISOR_BIT, VALID_BIT
    };
    use bit_field::BitField;


    pub(in crate::hal::arch::paging) fn read_page(
        page_level_base_address: PhysLv1PageAddress, index: u16
    ) -> Option<PhysLv1PageAddress> {

        let page_entry: u64 = unsafe { 
            page_level_base_address.get_address().offset_unchecked::<u64>(index.into()).read_unchecked()
        };

        if page_entry.get_bit(VALID_BIT) {
            return Some(PhysLv1PageAddress::new_maskoff(page_entry));
        } else {
            return None;
        }

    }

    ///Returns Accessed
    pub(in crate::hal::arch::paging) fn read_page_flag(page_level_base_address: PhysLv1PageAddress, index: u16) -> Option<bool> {

        let page_entry: u64 = unsafe { 
            page_level_base_address.get_address().offset_unchecked::<u64>(index.into()).read_unchecked()
        };

        if page_entry.get_bit(VALID_BIT) {
            
            return Some(page_entry.get_bit(ACCESSED_BIT));

        } else {
            return None;
        }

    }

    pub(in crate::hal::arch::paging) unsafe fn write_page_pml3_4(
        page_level_base_address: PhysLv1PageAddress, 
        index: u16,
        pml3_4_base_address: PhysLv1PageAddress,
    ) {

        let mut page_entry: u64 = 0;

        page_entry.set_bit(PRESENT_BIT, true);
        page_entry.set_bit(READ_WRITE_BIT, true);
        page_entry.set_bit(USER_SUPERVISOR_BIT, true);
        page_entry.set_bit(VALID_BIT, true);

        page_entry = page_entry | pml3_4_base_address.get_address().get_u64();

        page_level_base_address.get_address().offset_unchecked::<u64>(index.into()).write_unchecked::<u64>(&page_entry);


    }


    ///Sets the page to 0
    ///Unmaps the page for the software paging system
    pub(in crate::hal::arch::paging) unsafe fn unmap_page(page_level_base_address: PhysLv1PageAddress, index: u16) {
        page_level_base_address.get_address().offset_unchecked::<u64>(index.into()).write_unchecked::<u64>(&0);
    }
    
}

mod pml3 {

    use super::{
        decode_caching_bits, encode_caching_bits, PhysLv1PageAddress, PhysLv3PageAddress, ACCESSED_BIT, DIRTY_BIT, GLOBAL_BIT, NO_EXECUTE_BIT, PAGE_LEVEL_CACHE_DISABLE_BIT, PAGE_LEVEL_WRITETHROUGH_BIT, PRESENT_BIT, READ_WRITE_BIT, USER_SUPERVISOR_BIT, VALID_BIT
    };
    use crate::hal::paging::PageAttributes;
    use bit_field::BitField;

    const LARGE_PAGE_BIT: usize = 7;
    const PAGE_ATTRIBUTE_TABLE_BIT: usize = 12;

    pub enum Pml2Or1G {
        Pml2(PhysLv1PageAddress),
        G1(PageAttributes, PhysLv3PageAddress),
    }

    pub(in crate::hal::arch::paging) fn read_page(
        page_level_base_address: PhysLv1PageAddress, index: u16
    ) -> Option<Pml2Or1G> {

        let page_entry: u64 = unsafe { 
            page_level_base_address.get_address().offset_unchecked::<u64>(index.into()).read_unchecked()
        };

        if page_entry.get_bit(VALID_BIT) {
            
            if page_entry.get_bit(LARGE_PAGE_BIT) {
                //1G Page
                return Some(Pml2Or1G::G1(
                    PageAttributes {
                        present: page_entry.get_bit(PRESENT_BIT),
                        readonly: !page_entry.get_bit(READ_WRITE_BIT),
                        executable: !page_entry.get_bit(NO_EXECUTE_BIT),
                        supervisor: !page_entry.get_bit(USER_SUPERVISOR_BIT),
                        global: page_entry.get_bit(GLOBAL_BIT),
                        caching_mode: decode_caching_bits(
                            page_entry.get_bit(PAGE_LEVEL_WRITETHROUGH_BIT),
                            page_entry.get_bit(PAGE_LEVEL_CACHE_DISABLE_BIT),
                            page_entry.get_bit(PAGE_ATTRIBUTE_TABLE_BIT),
                        ),
                    },
                    PhysLv3PageAddress::new_maskoff(page_entry),
                ));

            } else {
                //Points to PML2
                return Some(
                    Pml2Or1G::Pml2(
                        PhysLv1PageAddress::new_maskoff(page_entry)
                    )
                );

            }

        } else {
            return None;
        }

    }


    ///Returns (Accessed, Dirty&LARGE_PAGE_BIT)
    pub(in crate::hal::arch::paging) fn read_page_flags(page_level_base_address: PhysLv1PageAddress, index: u16) -> Option<(bool, bool)> {

        let page_entry: u64 = unsafe { 
            page_level_base_address.get_address().offset_unchecked::<u64>(index.into()).read_unchecked()
        };

        if page_entry.get_bit(VALID_BIT) {
            return Some((
                page_entry.get_bit(ACCESSED_BIT),
                page_entry.get_bit(DIRTY_BIT) & page_entry.get_bit(LARGE_PAGE_BIT),
            ));
        } else {
            return None;
        }
    }

    pub(in crate::hal::arch::paging) unsafe fn write_page_pml2(
        page_level_base_address: PhysLv1PageAddress, 
        index: u16,
        pml2_base_address: PhysLv1PageAddress,
    ) {

        let mut page_entry: u64 = 0;

        page_entry.set_bit(PRESENT_BIT, true);
        page_entry.set_bit(READ_WRITE_BIT, true);
        page_entry.set_bit(USER_SUPERVISOR_BIT, true);
        page_entry.set_bit(VALID_BIT, true);

        page_entry = page_entry | pml2_base_address.get_address().get_u64();

        page_level_base_address.get_address().offset_unchecked::<u64>(index.into()).write_unchecked::<u64>(&page_entry);

    }

    pub(in crate::hal::arch::paging) unsafe fn write_page_1g(
        page_level_base_address: PhysLv1PageAddress, 
        index: u16,
        attributes: PageAttributes,
        phys_address: PhysLv3PageAddress,
    ) {

        let mut page_entry: u64 = 0;
        page_entry.set_bit(VALID_BIT, true);
        page_entry.set_bit(PRESENT_BIT, attributes.present);
        page_entry.set_bit(READ_WRITE_BIT, !attributes.readonly);
        page_entry.set_bit(NO_EXECUTE_BIT, !attributes.executable);
        page_entry.set_bit(USER_SUPERVISOR_BIT, !attributes.supervisor);
        page_entry.set_bit(GLOBAL_BIT, attributes.global);
        page_entry.set_bit(LARGE_PAGE_BIT, true);

        let caching: (bool, bool, bool) = encode_caching_bits(attributes.caching_mode);

        page_entry.set_bit(PAGE_LEVEL_WRITETHROUGH_BIT, caching.0);
        page_entry.set_bit(PAGE_LEVEL_CACHE_DISABLE_BIT, caching.1);
        page_entry.set_bit(PAGE_ATTRIBUTE_TABLE_BIT, caching.2);

        page_entry = page_entry | phys_address.get_address().get_u64();

        page_level_base_address.get_address().offset_unchecked::<u64>(index.into()).write_unchecked::<u64>(&page_entry);

    }

    ///Sets the page to 0
    ///Unmaps the page for the software paging system
    pub(in crate::hal::arch::paging) unsafe fn unmap_page(page_level_base_address: PhysLv1PageAddress, index: u16) {
        page_level_base_address.get_address().offset_unchecked::<u64>(index.into()).write_unchecked::<u64>(&0);
    }

    
}

mod pml2 {

    use super::{
        decode_caching_bits, encode_caching_bits, PhysLv1PageAddress, PhysLv2PageAddress, ACCESSED_BIT, DIRTY_BIT, GLOBAL_BIT, NO_EXECUTE_BIT, PAGE_LEVEL_CACHE_DISABLE_BIT, PAGE_LEVEL_WRITETHROUGH_BIT, PRESENT_BIT, READ_WRITE_BIT, USER_SUPERVISOR_BIT, VALID_BIT
    };
    use crate::hal::paging::PageAttributes;
    use bit_field::BitField;

    const LARGE_PAGE_BIT: usize = 7;
    const PAGE_ATTRIBUTE_TABLE_BIT: usize = 12;

    pub enum Pml1Or2M {
        Pml1(PhysLv1PageAddress),
        MB2(PageAttributes, PhysLv2PageAddress),
    }

    pub(in crate::hal::arch::paging) fn read_page(
        page_level_base_address: PhysLv1PageAddress, index: u16
    ) -> Option<Pml1Or2M> {

        let page_entry: u64 = unsafe { 
            page_level_base_address.get_address().offset_unchecked::<u64>(index.into()).read_unchecked()
        };

        if page_entry.get_bit(VALID_BIT) {
            
            if page_entry.get_bit(LARGE_PAGE_BIT) {
                //2MB Page
                return Some(Pml1Or2M::MB2(
                    PageAttributes {
                        present: page_entry.get_bit(PRESENT_BIT),
                        readonly: !page_entry.get_bit(READ_WRITE_BIT),
                        executable: !page_entry.get_bit(NO_EXECUTE_BIT),
                        supervisor: !page_entry.get_bit(USER_SUPERVISOR_BIT),
                        global: page_entry.get_bit(GLOBAL_BIT),
                        caching_mode: decode_caching_bits(
                            page_entry.get_bit(PAGE_LEVEL_WRITETHROUGH_BIT),
                            page_entry.get_bit(PAGE_LEVEL_CACHE_DISABLE_BIT),
                            page_entry.get_bit(PAGE_ATTRIBUTE_TABLE_BIT),
                        ),
                    },
                    PhysLv2PageAddress::new_maskoff(page_entry),
                ));

            } else {
                //Points to PML1
                return Some(
                    Pml1Or2M::Pml1(
                        PhysLv1PageAddress::new_maskoff(page_entry)
                    )
                );

            }

        } else {
            return None;
        }

    }


    ///Returns (Accessed, Dirty&LARGE_PAGE_BIT)
    pub(in crate::hal::arch::paging) fn read_page_flags(page_level_base_address: PhysLv1PageAddress, index: u16) -> Option<(bool, bool)> {

        let page_entry: u64 = unsafe { 
            page_level_base_address.get_address().offset_unchecked::<u64>(index.into()).read_unchecked()
        };

        if page_entry.get_bit(VALID_BIT) {
            return Some((
                page_entry.get_bit(ACCESSED_BIT),
                page_entry.get_bit(DIRTY_BIT) & page_entry.get_bit(LARGE_PAGE_BIT),
            ));
        } else {
            return None;
        }
    }

    pub(in crate::hal::arch::paging) unsafe fn write_page_pml1(
        page_level_base_address: PhysLv1PageAddress, 
        index: u16,
        pml1_base_address: PhysLv1PageAddress,
    ) {

        let mut page_entry: u64 = 0;

        page_entry.set_bit(PRESENT_BIT, true);
        page_entry.set_bit(READ_WRITE_BIT, true);
        page_entry.set_bit(USER_SUPERVISOR_BIT, true);
        page_entry.set_bit(VALID_BIT, true);

        page_entry = page_entry | pml1_base_address.get_address().get_u64();

        page_level_base_address.get_address().offset_unchecked::<u64>(index.into()).write_unchecked::<u64>(&page_entry);

    }

    pub(in crate::hal::arch::paging) unsafe fn write_page_2mb(
        page_level_base_address: PhysLv1PageAddress, 
        index: u16,
        attributes: PageAttributes,
        phys_address: PhysLv2PageAddress,
    ) {

        let mut page_entry: u64 = 0;
        page_entry.set_bit(VALID_BIT, true);
        page_entry.set_bit(PRESENT_BIT, attributes.present);
        page_entry.set_bit(READ_WRITE_BIT, !attributes.readonly);
        page_entry.set_bit(NO_EXECUTE_BIT, !attributes.executable);
        page_entry.set_bit(USER_SUPERVISOR_BIT, !attributes.supervisor);
        page_entry.set_bit(GLOBAL_BIT, attributes.global);
        page_entry.set_bit(LARGE_PAGE_BIT, true);

        let caching: (bool, bool, bool) = encode_caching_bits(attributes.caching_mode);

        page_entry.set_bit(PAGE_LEVEL_WRITETHROUGH_BIT, caching.0);
        page_entry.set_bit(PAGE_LEVEL_CACHE_DISABLE_BIT, caching.1);
        page_entry.set_bit(PAGE_ATTRIBUTE_TABLE_BIT, caching.2);

        page_entry = page_entry | phys_address.get_address().get_u64();

        page_level_base_address.get_address().offset_unchecked::<u64>(index.into()).write_unchecked::<u64>(&page_entry);

    }

    ///Sets the page to 0
    ///Unmaps the page for the software paging system
    pub(in crate::hal::arch::paging) unsafe fn unmap_page(page_level_base_address: PhysLv1PageAddress, index: u16) {
        page_level_base_address.get_address().offset_unchecked::<u64>(index.into()).write_unchecked::<u64>(&0);
    }

}

mod pml1 {

    use super::{
        decode_caching_bits, encode_caching_bits, PhysLv1PageAddress, ACCESSED_BIT, DIRTY_BIT, GLOBAL_BIT, NO_EXECUTE_BIT, PAGE_LEVEL_CACHE_DISABLE_BIT, PAGE_LEVEL_WRITETHROUGH_BIT, PRESENT_BIT, READ_WRITE_BIT, USER_SUPERVISOR_BIT, VALID_BIT
    };
    use crate::hal::paging::PageAttributes;
    use bit_field::BitField;

    const PAGE_ATTRIBUTE_TABLE_BIT: usize = 7;

    pub(in crate::hal::arch::paging) fn read_page(
        page_level_base_address: PhysLv1PageAddress, index: u16
    ) -> Option<(PageAttributes, PhysLv1PageAddress)> {

        let page_entry: u64 = unsafe { 
            page_level_base_address.get_address().offset_unchecked::<u64>(index.into()).read_unchecked()
        };

        if page_entry.get_bit(VALID_BIT) {
            return Some((
                PageAttributes {
                    present: page_entry.get_bit(PRESENT_BIT),
                    readonly: !page_entry.get_bit(READ_WRITE_BIT),
                    executable: !page_entry.get_bit(NO_EXECUTE_BIT),
                    supervisor: !page_entry.get_bit(USER_SUPERVISOR_BIT),
                    global: page_entry.get_bit(GLOBAL_BIT),
                    caching_mode: decode_caching_bits(
                        page_entry.get_bit(PAGE_LEVEL_WRITETHROUGH_BIT),
                        page_entry.get_bit(PAGE_LEVEL_CACHE_DISABLE_BIT),
                        page_entry.get_bit(PAGE_ATTRIBUTE_TABLE_BIT),
                    ),
                },
                PhysLv1PageAddress::new_maskoff(page_entry),
            ));
        } else {
            return None;
        }
    }

    ///Returns (Accessed, Dirty)
    pub(in crate::hal::arch::paging) fn read_page_flags(page_level_base_address: PhysLv1PageAddress, index: u16) -> Option<(bool, bool)> {

        let page_entry: u64 = unsafe { 
            page_level_base_address.get_address().offset_unchecked::<u64>(index.into()).read_unchecked()
        };

        if page_entry.get_bit(VALID_BIT) {
            return Some((
                page_entry.get_bit(ACCESSED_BIT),
                page_entry.get_bit(DIRTY_BIT),
            ));
        } else {
            return None;
        }
    }

    pub(in crate::hal::arch::paging) unsafe fn write_page(
        page_level_base_address: PhysLv1PageAddress, 
        index: u16,
        attributes: PageAttributes,
        phys_address: PhysLv1PageAddress,
    ) {
        let mut page_entry: u64 = 0;
        page_entry.set_bit(VALID_BIT, true);
        page_entry.set_bit(PRESENT_BIT, attributes.present);
        page_entry.set_bit(READ_WRITE_BIT, !attributes.readonly);
        page_entry.set_bit(NO_EXECUTE_BIT, !attributes.executable);
        page_entry.set_bit(USER_SUPERVISOR_BIT, !attributes.supervisor);
        page_entry.set_bit(GLOBAL_BIT, attributes.global);

        let caching: (bool, bool, bool) = encode_caching_bits(attributes.caching_mode);

        page_entry.set_bit(PAGE_LEVEL_WRITETHROUGH_BIT, caching.0);
        page_entry.set_bit(PAGE_LEVEL_CACHE_DISABLE_BIT, caching.1);
        page_entry.set_bit(PAGE_ATTRIBUTE_TABLE_BIT, caching.2);

        page_entry = page_entry | phys_address.get_address().get_u64();

        page_level_base_address.get_address().offset_unchecked::<u64>(index.into()).write_unchecked::<u64>(&page_entry);

    }

    ///Sets the page to 0
    ///Unmaps the page for the software paging system
    pub(in crate::hal::arch::paging) unsafe fn unmap_page(page_level_base_address: PhysLv1PageAddress, index: u16) {
        page_level_base_address.get_address().offset_unchecked::<u64>(index.into()).write_unchecked::<u64>(&0);
    }
}
