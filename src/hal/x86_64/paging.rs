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
    PMLEntryIndex,
    PMLEntryIndex,
    PMLEntryIndex,
    PMLEntryIndex,
    PMLEntryIndex,
) {
    (
        PMLEntryIndex {
            index: ((address.get_u64() & PML5ENTRYINDEXMASK) >> 48),
        },
        PMLEntryIndex {
            index: ((address.get_u64() & PML4ENTRYINDEXMASK) >> 39),
        },
        PMLEntryIndex {
            index: ((address.get_u64() & PML3ENTRYINDEXMASK) >> 30),
        },
        PMLEntryIndex {
            index: ((address.get_u64() & PML2ENTRYINDEXMASK) >> 21),
        },
        PMLEntryIndex {
            index: ((address.get_u64() & PML1ENTRYINDEXMASK) >> 12),
        },
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

mod pml_root {}

mod pml5 {

    
    compile_error!("TODO");

    //getpml3(index: EntryIndex)
    //setpml3(index: EntryIndex)
}

mod pml4 {

    

    compile_error!("TODO");

    //getpml3(index: EntryIndex)
    //setpml3(index: EntryIndex)
}

mod pml3 {

    

    compile_error!("TODO");

    //getpml2orlv3page(index: EntryIndex)
    //setpml2(index: EntryIndex)
    //setlv3page(index: EntryIndex)
}

mod pml2 {
    use core::ptr::NonNull;

    use volatile::VolatilePtr;

    use crate::hal::paging::PageAttributes;

    use super::{pml1::Pml1Page, PhysLv1PageAddress, VirtLv1PageAddress};

    pub struct Pml2Page {
        pub start: VirtLv1PageAddress,
    }

    pub enum Pml1Or2M {
        None,
        Pml1(Pml1Page),
        MB2(PageAttributes, PhysLv1PageAddress),
    }

    compile_error!("TODO");

    ///Unsafe: dont use a invalid EntryIndex
    pub unsafe fn getpml1orlv2page(ptr: Pml2Page, index: EntryIndex) -> Pml1Or2M {
        let entry: u64 = unsafe {
            VolatilePtr::new(NonNull::new_unchecked(
                (ptr.start.get_address().get_u64() as *mut u64).offset(index.index as isize),
            ))
            .read()
        };
    }
    //setpml1(index: EntryIndex)
    //setlv2page(index: EntryIndex)
}

mod pml1 {

    use core::mem::size_of;

    use super::{
        decode_caching_bits, encode_caching_bits, PMLEntryIndex, PhysLv1PageAddress, VirtAddress, ACCESSED_BIT, DEFAULTPHYSADDRESSMASK, DIRTY_BIT, GLOBAL_BIT, NO_EXECUTE_BIT, PAGE_LEVEL_CACHE_DISABLE_BIT, PAGE_LEVEL_WRITETHROUGH_BIT, PRESENT_BIT, READ_WRITE_BIT, USER_SUPERVISOR_BIT, VALID_BIT
    };
    use crate::hal::paging::PageAttributes;
    use bit_field::BitField;

    const PAGE_ATTRIBUTE_TABLE_BIT: usize = 7;

    pub struct Pml1Page {
        pub base_address: PhysLv1PageAddress,
    }

    pub(in crate::hal::arch::paging) fn read_page(
        ptr: Pml1Page, index: PMLEntryIndex
    ) -> Option<(PageAttributes, PhysLv1PageAddress)> {
        let page_entry: u64 = unsafe { 
            
            VirtAddress::new_unchecked(
                ptr.base_address.get_address().get_u64() + (size_of::<u64>() as u64 * index.get_index())
            ).read::<u64>()

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
                unsafe { PhysLv1PageAddress::new_unchecked(page_entry & DEFAULTPHYSADDRESSMASK) },
            ));
        } else {
            return None;
        }
    }

    ///Return (Accessed, Dirty)
    pub(in crate::hal::arch::paging) fn read_page_flags(ptr: Pml1Page) -> Option<(bool, bool)> {
        let page_entry: u64 = unsafe { ptr.start.read::<u64>() };

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
        ptr: Pml1Page,
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

        unsafe { ptr.start.write::<u64>(&page_entry) }
    }

    ///Sets the page to 0
    pub(in crate::hal::arch::paging) unsafe fn unmap_page(ptr: Pml1Page) {
        ptr.start.write::<u64>(&0)
    }
}
