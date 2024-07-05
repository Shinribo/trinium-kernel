//use x86_64::registers::segmentation::{Segment, CS, SS};
//use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
//use x86_64::structures::tss::TaskStateSegment;

//static GDT: x86_64::structures::gdt::GlobalDescriptorTable = {
//    let mut gdt = GlobalDescriptorTable::new();
//    gdt.append(Descriptor::kernel_code_segment());
//    gdt.append(Descriptor::kernel_data_segment());
//    gdt.append(Descriptor::user_code_segment());
//    gdt.append(Descriptor::user_data_segment());
//    gdt.append(Descriptor::tss_segment(unsafe{&TSS}));
//    gdt
//};

//static mut TSS: TaskStateSegment = TaskStateSegment::new();

//pub(in crate::hal::arch) fn init_gdt() {
//    GDT.load();
//    unsafe{
//        CS::set_reg(SegmentSelector(1));
//        SS::set_reg(SegmentSelector(2));
//    }

//}
