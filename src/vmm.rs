//high level segmentation based memory management over low level paging
//cs - code segment -> readonly
//ds - data segment -> readonly/rw + nx
//ss - stack segment -> rw + nx
//mmios - mmio segment -> r/rw + nx

use crate::hal::memory::*;
use alloc::{boxed::Box, vec::Vec};

compile_error!("Box Size is Runtime Constant depending on Paging config: on x86_64 its 512 for 2M and 4K Page Arrays");
compile_error!("maybe allow a trailing box to be smaller");
compile_error!(
    "implement demand paging so that pages are attempted to be allocated Lv2 > Lv1 > Lv3"
);
compile_error!("maybe make it so that lv3 cannot be allocated but only merged when a lv3 page is filled with smaller pages");

enum Lv3PageBlock {
    Lv3(Lv3Page),
    Lv2(Box<[Option<Lv2PageBlock>]>),
}

enum Lv2PageBlock {
    Lv2(Lv2Page),
    Lv1(Box<[Option<Lv1Page>]>),
}

struct Lv1Page {
    phys_page_index: PhysLv1PageAddress,
}

struct Lv2Page {
    phys_page_index: PhysLv2PageAddress,
}

struct Lv3Page {
    phys_page_index: PhysLv3PageAddress,
}

pub enum SegmentsTypes {
    //Size,     RWX,    Caching,    Demand
    CodeSegment,         //Fixed,    ROX,    Default,    yes
    DataSegment,         //Dynamic,  RW,     Default,    yes
    DataROSegment,       //Fixed,    RO,     Default,    yes
    StackSegment,        //Fixed,    RW,     Default,    yes (custom algorithm)
    DeviceMMIOSegment,   //Fixed,    RW,     MMIO,       no
    DeviceMMIOROSegment, //Fixed,    RO,     MMIO,       no
    DMASegment,          //Fixed,    RW,     DMA,        no
    IPCSegment,          //Dynamic,  RW,     Default,    no
    IPCROSegment,        //Dynamic,  RO,     Default,    no
    HHDMSegment,         //Fixed,    RW,     Default,    no
}

pub enum SegmentError {
    FixedSizeSegmentCantHaveReservedSpace,
    InvalidBaseAddress,
    OutOfBounds,
    InvalidSegmentTypeForConstructor, //
}

enum HighestPageSize {
    Lv3(Vec<Lv3PageBlock>),
    Lv2(Vec<Lv2PageBlock>),
    Lv1(Vec<Lv1Page>),
}

enum Page {
    None,
    Lv1(PhysLv1PageAddress),
    Lv2(PhysLv2PageAddress),
    Lv3(PhysLv3PageAddress),
}

enum SegmentBehavior {
    Normal,
    Cow,
    Demand,
}

//let the specialized segments listed in the enum wrap a unspecialized segment to reduce code duplication and complexity
//maybe just have the segment type with subtypes that enforce specific behavior
compile_error!("maybe choose nested trees");
compile_error!("Add a in Progrss bit to indicate that the current segment is beeing modified and the #pf may be a false positive");
compile_error!("maybe add page attributes into the segment struct");
pub struct Segment {
    segment_type: SegmentsTypes,
    kernel_mode: bool, //if the segment should require privileged access (ring0)
    global: bool,
    base_address: VirtLv1PageAddress,
    phys_pages: HighestPageSize,
    allocated_size: u64, //Size of the memory region that is demand backed with pages (some segment types dont support demand paging)
    reserved_size: u64, //Size of the memory region (starting after allocated_size) that is guraanted to not to intersect with other segments
    segment_behavior: SegmentBehavior,
}

impl Segment {
    ///Creates the specified mappings and allocates the specified amount of memory (NOTE: will round up the segment size to a lv1 page boundary)
    ///Limited to:
    ///    DataSegment
    ///    StackSegment
    ///    DMASegment
    fn new(
        p_segment_type: SegmentsTypes,
        p_kernel: bool,
        p_global: bool,
        p_base_address: VirtLv1PageAddress,
        p_allocated_size: u64,
        p_reserved_size: u64,
    ) -> Result<Segment, SegmentError> {
        if p_base_address.get_address().get_u64() == 0 {
            return Err(SegmentError::InvalidBaseAddress);
        }

        match p_segment_type {
            SegmentsTypes::DataSegment
            | SegmentsTypes::StackSegment
            | SegmentsTypes::DMASegment => {}
            _ => return Err(SegmentError::InvalidSegmentTypeForConstructor),
        }

        if p_reserved_size > 0 {
            match p_segment_type {
                SegmentsTypes::StackSegment | SegmentsTypes::DMASegment => {
                    return Err(SegmentError::FixedSizeSegmentCantHaveReservedSpace)
                }
                _ => {}
            }
        }

        compile_error!("TODO")
    }

    ///Creates a segment on the given phys pages and multi allocs them if the segment type indicates usable memory (NOTE: HHDMSegment doesnt multi allocs memory)
    ///Limited to:
    ///    CodeSegment
    ///    DataROSegment
    ///    DeviceMMIOSegment
    ///    DeviceMMIOROSegment
    ///    IPCSegment
    ///    IPCROSegment
    ///    HHDMSegment
    fn new_with_memory() -> Result<Segment, SegmentError> {
        compile_error!("TODO")
    }

    ///Creates a segment on the given phys pages with RO permissions and does cow
    ///Limited to:
    ///    DataSegment
    fn new_cow() -> Result<Segment, SegmentError> {
        compile_error!("TODO")
    }

    ///Creates the specified mappings and demand allocates the specified amount of zeroed memory (NOTE: will round up the segment size to a lv1 page boundary)
    ///Limited to:
    ///    CodeSegment
    ///    DataSegment
    ///    DataROSegment
    ///    StackSegment
    fn new_demand() -> Result<Segment, SegmentError> {
        compile_error!("TODO")
    }

    ///Used to create certain Segments without using demand paging
    ///Limited to:
    ///    DataSegment -> CodeSegment
    ///    DataSegment -> DataROSegment
    fn change_segment_type() -> Result<Segment, SegmentError> {
        compile_error!("TODO")
    }

    pub fn get_segment_type(&self) -> SegmentsTypes {
        self.segment_type
    }

    pub fn is_kernel(&self) -> bool {
        self.kernel_mode
    }

    pub fn is_global(&self) -> bool {
        self.global
    }

    pub fn get_segment_base_address(&self) -> VirtLv1PageAddress {
        self.base_address
    }

    pub fn virt_to_phys_translation(&self, address: VirtAddress) -> Result<Page, SegmentError> {
        if address < self.base_address.get_address()
            || address.get_u64() > (self.base_address.get_address().get_u64() + self.allocated_size)
        {
            return Err(SegmentError::OutOfBounds);
        }

        compile_error!("TODO")
    }
}

struct Addressspace {
    //PageRoot //Opaque Struct that contains arch dependant stuff
    segment_list: alloc::vec::Vec<Segment>,
}
