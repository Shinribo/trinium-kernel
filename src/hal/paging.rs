use super::{arch, memory::*};
use alloc::vec::Vec;

pub type PageRoot = arch::paging::ArchPageRoot;

pub struct PageAttributes {
    pub present: bool, //Indicates to the MMU that it can use the Page
    pub readonly: bool,
    pub executable: bool,
    pub supervisor: bool,
    pub global: bool,
    pub caching_mode: CachingMode,
}

pub enum CachingMode {
    Default,
    Framebuffer,
    MMIO,
    MmioPrefetch, //Indicates that the MMIO has no Read Sideeffects and can be cached
    DMA,
}

pub enum PagingErros {
    PageAlreadyPresent,
    PageAlreadyNotPresent,
    NumberOfPagesOutOfBounds,
    InsufficientPagesForPageTable,
}

pub enum Page {
    None,
    Lv1Page(Lv1Page),
    Lv2Page(Lv2Page),
    Lv3Page(Lv3Page),
}

pub struct Lv1Page {
    pub attributes: PageAttributes,
    pub phys_address: PhysLv1PageAddress,
    pub virt_address: VirtLv1PageAddress,
}

pub struct Lv2Page {
    pub attributes: PageAttributes,
    pub phys_address: PhysLv2PageAddress,
    pub virt_address: VirtLv2PageAddress,
}

pub struct Lv3Page {
    pub attributes: PageAttributes,
    pub phys_address: PhysLv3PageAddress,
    pub virt_address: VirtLv3PageAddress,
}

//these functions dont manage physical pages in any way
//Thin Wrapper to ensure that all code outside the hal mod never needs to touch the arch mod
//the slice variant is intended for when dynamic allocation is not availible (for example at boottime) or when the amount of pages is known at compiletime
//preserves the physical page ordering from the input list (Only for the mapped Pages NOT for the pages needed for the Page Table)

#[inline(always)]
pub unsafe fn map_slice_lv1_page(
    root: PageRoot,
    phys_pages_for_pt: &mut [PhysLv1PageAddress],
    phys_pages: &mut [PhysLv1PageAddress],
    virt_start_addr: VirtLv1PageAddress,
    attributes: PageAttributes,
) -> Result<(), PagingErros> {
    arch::paging::map_slice_lv1_page(
        root,
        phys_pages_for_pt,
        phys_pages,
        virt_start_addr,
        attributes,
    )
}

#[inline(always)]
pub unsafe fn map_slice_lv2_page(
    root: PageRoot,
    phys_pages_for_pt: &mut [PhysLv1PageAddress],
    phys_pages: &mut [PhysLv2PageAddress],
    virt_start_addr: VirtLv2PageAddress,
    attributes: PageAttributes,
) -> Result<(), PagingErros> {
    arch::paging::map_slice_lv2_page(
        root,
        phys_pages_for_pt,
        phys_pages,
        virt_start_addr,
        attributes,
    )
}

#[inline(always)]
pub unsafe fn map_slice_lv3_page(
    root: PageRoot,
    phys_pages_for_pt: &mut [PhysLv1PageAddress],
    phys_pages: &mut [PhysLv3PageAddress],
    virt_start_addr: VirtLv3PageAddress,
    attributes: PageAttributes,
) -> Result<(), PagingErros> {
    arch::paging::map_slice_lv3_page(
        root,
        phys_pages_for_pt,
        phys_pages,
        virt_start_addr,
        attributes,
    )
}

#[inline(always)]
pub unsafe fn map_vec_lv1_page(
    root: PageRoot,
    phys_pages_for_pt: Vec<PhysLv1PageAddress>,
    phys_pages: Vec<PhysLv1PageAddress>,
    virt_start_addr: VirtLv1PageAddress,
    attributes: PageAttributes,
) -> Result<(), PagingErros> {
    arch::paging::map_vec_lv1_page(
        root,
        phys_pages_for_pt,
        phys_pages,
        virt_start_addr,
        attributes,
    )
}

#[inline(always)]
pub unsafe fn map_vec_lv2_page(
    root: PageRoot,
    phys_pages_for_pt: Vec<PhysLv1PageAddress>,
    phys_pages: Vec<PhysLv2PageAddress>,
    virt_start_addr: VirtLv2PageAddress,
    attributes: PageAttributes,
) -> Result<(), PagingErros> {
    arch::paging::map_vec_lv2_page(
        root,
        phys_pages_for_pt,
        phys_pages,
        virt_start_addr,
        attributes,
    )
}

#[inline(always)]
pub unsafe fn map_vec_lv3_page(
    root: PageRoot,
    phys_pages_for_pt: Vec<PhysLv1PageAddress>,
    phys_pages: Vec<PhysLv3PageAddress>,
    virt_start_addr: VirtLv3PageAddress,
    attributes: PageAttributes,
) -> Result<(), PagingErros> {
    arch::paging::map_vec_lv3_page(
        root,
        phys_pages_for_pt,
        phys_pages,
        virt_start_addr,
        attributes,
    )
}

///Returns a Vec with pt pages that are not needed anymore
/// Performs necessary TLB Invalidations
#[inline(always)]
pub unsafe fn unmap_lv1_page(
    root: PageRoot,
    start: VirtLv1PageAddress,
    number_of_pages: u64,
) -> Result<Vec<PhysLv1PageAddress>, PagingErros> {
    arch::paging::unmap_lv1_page(root, start, number_of_pages)
}

///Returns a Vec with pt pages that are not needed anymore
/// Performs necessary TLB Invalidations
#[inline(always)]
pub unsafe fn unmap_lv2_page(
    root: PageRoot,
    start: VirtLv2PageAddress,
    number_of_pages: u64,
) -> Result<Vec<PhysLv1PageAddress>, PagingErros> {
    arch::paging::unmap_lv2_page(root, start, number_of_pages)
}

///Returns a Vec with pt pages that are not needed anymore
/// Performs necessary TLB Invalidations
#[inline(always)]
pub unsafe fn unmap_lv3_page(
    root: PageRoot,
    start: VirtLv3PageAddress,
    number_of_pages: u64,
) -> Result<Vec<PhysLv1PageAddress>, PagingErros> {
    arch::paging::unmap_lv3_page(root, start, number_of_pages)
}

///Updates the Attributes of the give Range
/// Performs necessary TLB Invalidations
#[inline(always)]
pub unsafe fn update_page_attributes(
    root: PageRoot,
    virt_start_addr: VirtAddress,
    virt_end_addr: VirtAddress,
    attributes: PageAttributes,
) {
    arch::paging::update_page_attributes(root, virt_start_addr, virt_end_addr, attributes)
}

#[inline(always)]
pub fn get_single_page(root: PageRoot, virt_start_addr: VirtAddress) -> Option<Page> {
    arch::paging::get_single_page(root, virt_start_addr)
}

#[inline(always)]
pub fn get_vec_page(
    root: PageRoot,
    virt_start_addr: VirtAddress,
    virt_end_addr: VirtAddress,
) -> Vec<Option<Page>> {
    arch::paging::get_vec_page(root, virt_start_addr, virt_end_addr)
}

#[inline(always)]
pub fn needed_pt_pages_lv1(root: PageRoot, start: VirtLv1PageAddress, number_of_pages: u64) -> u64 {
    arch::paging::needed_pt_pages_lv1(root, start, number_of_pages)
}

#[inline(always)]
pub fn needed_pt_pages_lv2(root: PageRoot, start: VirtLv2PageAddress, number_of_pages: u64) -> u64 {
    arch::paging::needed_pt_pages_lv2(root, start, number_of_pages)
}

#[inline(always)]
pub fn needed_pt_pages_lv3(root: PageRoot, start: VirtLv3PageAddress, number_of_pages: u64) -> u64 {
    arch::paging::needed_pt_pages_lv3(root, start, number_of_pages)
}
