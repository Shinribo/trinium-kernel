use lazy_static::lazy_static;
use raw_cpuid::CpuIdReaderNative;

lazy_static! {
    pub static ref CPUID_INSTANCE: raw_cpuid::CpuId<CpuIdReaderNative> = raw_cpuid::CpuId::new();
}

pub fn physical_address_bit_size() -> u8 {
    if let Some(processor_capacity_feature_info) =
        (*CPUID_INSTANCE).get_processor_capacity_feature_info()
    {
        return processor_capacity_feature_info.physical_address_bits();
    }
    panic!("CPUID ERROR: LEAF 0x8000 0008 NOT SUPPORTED"); //Sollte nie passieren
}

pub fn virtual_address_bit_size() -> u8 {
    if let Some(processor_capacity_feature_info) =
        (*CPUID_INSTANCE).get_processor_capacity_feature_info()
    {
        return processor_capacity_feature_info.linear_address_bits();
    }
    panic!("CPUID ERROR: LEAF 0x8000 0008 NOT SUPPORTED"); //Sollte nie passieren
}

pub fn pcid_supported() -> bool {
    //Es wird nie ein Panic erzeugt, da Leaf 0x01 bereits von CPUs unterstützt wird die noch gar kein 64bit können
    (*CPUID_INSTANCE)
        .get_feature_info()
        .expect("CPUID ERROR: LEAF 0x01 NOT SUPPORTED")
        .has_pcid()
}
