use core::{
    mem::{size_of, transmute_copy},
    num::NonZeroU64,
    ptr::{read_volatile, write_volatile},
};

use bit_field::BitField;
use lazy_static::lazy_static;

use crate::bal::hhdm::HHDM_OFFSET;

#[derive(Debug)]
pub enum MemoryAddressErrors {
    NonCannonical,
    InvalidBits,
    NULLPTR,
    Invalid,
    OutOfBounds,
}

//Set Bits are allowed to be used
//CANNONICAL_BIT has to be the highes valid address bit
lazy_static! {
    static ref BIT_MASK_PHYS_ADDRESS: u64 = super::arch::memory::get_max_supported_phy_address_as_bit_mask();
    static ref BIT_MASK_VIRT_ADDRESS: u64 = super::arch::memory::get_max_supported_virt_address_as_bit_mask();
    static ref CANNONICAL_BIT: Option<u8> = super::arch::memory::get_cannonical_bit_number(); //Bitnumber that is used for sign extension
}

///Ensures that plattform Constrains are fullfilled (for example max 52bit on x86_64)
/// NOTE: This Ptr can be NULL as Physical Address Space doesnt have a meaning for NULL
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PhysAddress {
    address: u64,
}

impl PhysAddress {
    #[inline]
    pub fn new(value: u64) -> Result<Self, MemoryAddressErrors> {
        if value & (!*BIT_MASK_PHYS_ADDRESS) != 0 {
            return Err(MemoryAddressErrors::InvalidBits);
        }

        Ok(Self { address: value })
    }

    ///masks of unsupported bits
    #[inline]
    pub fn new_maskoff(value: u64) -> Self {
        Self {
            address: value & *BIT_MASK_PHYS_ADDRESS,
        }
    }

    ///Caller has to ensure that >value< fullfills platform address space constrains
    #[inline]
    pub unsafe fn new_unchecked(value: u64) -> Self {
        Self { address: value }
    }

    #[inline]
    pub fn get_u64(&self) -> u64 {
        self.address
    }

    #[inline]
    pub fn offset<T>(&self, count: i64) -> Result<Self, MemoryAddressErrors> {
        Self::new(
            self.get_u64()
                .checked_add_signed(size_of::<T>() as i64 * count)
                .ok_or(MemoryAddressErrors::OutOfBounds)?,
        )
    }

    #[inline]
    pub fn offset_maskoff<T>(&self, count: i64) -> Result<Self, MemoryAddressErrors> {
        Ok(Self::new_maskoff(
            self.get_u64()
                .checked_add_signed(size_of::<T>() as i64 * count)
                .ok_or(MemoryAddressErrors::OutOfBounds)?,
        ))
    }

    ///Caller has to ensure that the new address fullfills platform address space constrains
    #[inline]
    pub unsafe fn offset_unchecked<T>(&self, count: i64) -> Self {
        Self::new_unchecked(
            self.get_u64()
                .wrapping_add_signed(size_of::<T>() as i64 * count),
        )
    }

    ///Converts the physical Address into a virtual Address via the HHDM
    #[inline]
    pub fn to_virt(&self) -> Result<VirtAddress, MemoryAddressErrors> {
        VirtAddress::new(self.get_u64() + (*HHDM_OFFSET).address.get_u64())
    }

    ///Converts the physical Address into a virtual Address via the HHDM \
    ///masks of unsupported bits
    #[inline]
    pub fn to_virt_maskoff(&self) -> Result<VirtAddress, MemoryAddressErrors> {
        VirtAddress::new_maskoff(self.get_u64() + (*HHDM_OFFSET).address.get_u64())
    }

    ///Converts the physical Address into a virtual Address via the HHDM \
    ///Caller has to ensure that >value< fullfills platform address space constrains
    #[inline]
    pub unsafe fn to_virt_unchecked(&self) -> VirtAddress {
        VirtAddress::new_unchecked(self.get_u64() + (*HHDM_OFFSET).address.get_u64())
    }

    ///Performs a volatile Read from physical Memory via the HHDM \
    ///The caller has to ensure that the target page is mapped with the needed permissions \
    ///The caller has to properly handle invalid instances of T returned by this function
    #[inline(always)]
    pub unsafe fn read<T>(&self) -> Result<T, MemoryAddressErrors>
    where
        [(); size_of::<T>()]:,
    {
        Ok(self.to_virt()?.read::<T>())
    }

    ///Performs a volatile Read from physical Memory via the HHDM \
    ///masks of unsupported bits \
    ///The caller has to ensure that the target page is mapped with the needed permissions \
    ///The caller has to properly handle invalid instances of T returned by this function \
    #[inline(always)]
    pub unsafe fn read_maskoff<T>(&self) -> Result<T, MemoryAddressErrors>
    where
        [(); size_of::<T>()]:,
    {
        Ok(self.to_virt_maskoff()?.read::<T>())
    }

    ///Performs a volatile Read from physical Memory via the HHDM \
    ///Caller has to ensure that >value< fullfills platform address space constrains \
    ///The caller has to ensure that the target page is mapped with the needed permissions \
    ///The caller has to properly handle invalid instances of T returned by this function
    #[inline(always)]
    pub unsafe fn read_unchecked<T>(&self) -> T
    where
        [(); size_of::<T>()]:,
    {
        self.to_virt_unchecked().read::<T>()
    }

    ///Performs a volatile Write to physical Memory via the HHDM \
    ///The caller has to ensure that the target page is mapped with the needed permissions
    #[inline(always)]
    pub unsafe fn write<T>(&self, value: &T) -> Result<(), MemoryAddressErrors>
    where
        [(); size_of::<T>()]:,
    {
        Ok(self.to_virt()?.write::<T>(value))
    }

    ///Performs a volatile Write to physical Memory via the HHDM \
    ///masks of unsupported bits \
    ///The caller has to ensure that the target page is mapped with the needed permissions
    #[inline(always)]
    pub unsafe fn write_maskoff<T>(&self, value: &T) -> Result<(), MemoryAddressErrors>
    where
        [(); size_of::<T>()]:,
    {
        Ok(self.to_virt_maskoff()?.write::<T>(value))
    }

    ///Performs a volatile Write to physical Memory via the HHDM \
    ///Caller has to ensure that >value< fullfills platform address space constrains \
    ///The caller has to ensure that the target page is mapped with the needed permissions
    #[inline(always)]
    pub unsafe fn write_unchecked<T>(&self, value: &T)
    where
        [(); size_of::<T>()]:,
    {
        self.to_virt_unchecked().write::<T>(value)
    }
}

////Wrapper around a u64 that ensures that plattform constrains for the address size are met
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VirtAddress {
    address: NonZeroU64,
}

impl VirtAddress {
    #[inline]
    pub fn new(value: u64) -> Result<Self, MemoryAddressErrors> {
        let masked_value: u64 = value & (!*BIT_MASK_VIRT_ADDRESS);

        //Checks if a cannonical bit is used and set
        //Note: Reodering the if branches doesnt decrease the amount of branches on average (currently 2-3-3)
        if let Some(cannonical_bit_value) = *CANNONICAL_BIT
            && value.get_bit(cannonical_bit_value as usize)
        {
            if masked_value != !*BIT_MASK_VIRT_ADDRESS {
                return Err(MemoryAddressErrors::NonCannonical);
            }
        } else {
            if masked_value != 0 {
                if (*CANNONICAL_BIT).is_none() {
                    return Err(MemoryAddressErrors::InvalidBits);
                }

                return Err(MemoryAddressErrors::NonCannonical);
            }
        }

        Ok(Self {
            address: NonZeroU64::new(value).ok_or(MemoryAddressErrors::NULLPTR)?,
        })
    }

    ///masks of unsupported bits
    #[inline]
    pub fn new_maskoff(value: u64) -> Result<Self, MemoryAddressErrors> {
        //Checks if a cannonical bit is used and set
        if let Some(cannonical_bit_value) = *CANNONICAL_BIT
            && (value as u64).get_bit(cannonical_bit_value as usize)
        {
            return Ok(Self {
                address: NonZeroU64::new(
                    (value & *BIT_MASK_VIRT_ADDRESS) | !*BIT_MASK_VIRT_ADDRESS,
                )
                .ok_or(MemoryAddressErrors::NULLPTR)?,
            });
        }

        Ok(Self {
            address: NonZeroU64::new(value & *BIT_MASK_VIRT_ADDRESS)
                .ok_or(MemoryAddressErrors::NULLPTR)?,
        })
    }

    ///Caller has to ensure that >value< fullfills platform address space constrains
    #[inline]
    pub unsafe fn new_unchecked(value: u64) -> Self {
        Self {
            address: NonZeroU64::new_unchecked(value),
        }
    }

    #[inline]
    pub fn get_u64(&self) -> u64 {
        self.address.into()
    }

    #[inline]
    pub fn get_non_zero_u64(&self) -> NonZeroU64 {
        self.address.into()
    }

    #[inline]
    pub fn offset<T>(&self, count: i64) -> Result<Self, MemoryAddressErrors> {
        Self::new(
            self.get_u64()
                .checked_add_signed(size_of::<T>() as i64 * count)
                .ok_or(MemoryAddressErrors::OutOfBounds)?,
        )
    }

    #[inline]
    pub fn offset_maskoff<T>(&self, count: i64) -> Result<Self, MemoryAddressErrors> {
        Self::new_maskoff(
            self.get_u64()
                .checked_add_signed(size_of::<T>() as i64 * count)
                .ok_or(MemoryAddressErrors::OutOfBounds)?,
        )
    }

    ///Caller has to ensure that the new address fullfills platform address space constrains
    #[inline]
    pub unsafe fn offset_unchecked<T>(&self, count: i64) -> Self {
        Self::new_unchecked(
            self.get_u64()
                .wrapping_add_signed(size_of::<T>() as i64 * count),
        )
    }

    ///Converts the virtual Address into a physical Address via the HHDM
    #[inline]
    pub fn to_phys(&self) -> Result<PhysAddress, MemoryAddressErrors> {
        if self.get_u64() < (*HHDM_OFFSET).address.get_u64() {
            return Err(MemoryAddressErrors::Invalid);
        }

        PhysAddress::new(self.get_u64() - (*HHDM_OFFSET).address.get_u64())
    }

    ///Converts the virtual Address into a physical Address via the HHDM \
    ///masks of unsupported bits
    #[inline]
    pub fn to_phys_maskoff(&self) -> PhysAddress {
        if self.get_u64() < (*HHDM_OFFSET).address.get_u64() {
            return PhysAddress::new_maskoff(0);
        }

        PhysAddress::new_maskoff(self.get_u64() - (*HHDM_OFFSET).address.get_u64())
    }

    ///Converts the virtual Address into a physical Address via the HHDM \
    ///Caller has to ensure that >value< fullfills platform address space constrains
    #[inline]
    pub unsafe fn to_phys_unchecked(&self) -> PhysAddress {
        PhysAddress::new_unchecked(self.get_u64() - (*HHDM_OFFSET).address.get_u64())
    }

    ///Performs a volatile Read to virtual Memory \
    ///The caller has to ensure that the target page is mapped with the needed permissions \
    ///The caller has to properly handle invalid instances of T returned by this function
    #[inline(always)]
    pub unsafe fn read<T>(&self) -> T
    where
        [(); size_of::<T>()]:,
    {
        transmute_copy::<[u8; size_of::<T>()], T>(&read_volatile(
            (self.get_u64()) as *const [u8; size_of::<T>()],
        ))
    }

    ///Performs a volatile Write to virtual Memory \
    ///The caller has to ensure that the target page is mapped with the needed permissions
    #[inline(always)]
    pub unsafe fn write<T>(&self, value: &T)
    where
        [(); size_of::<T>()]:,
    {
        write_volatile(
            self.get_u64() as *mut [u8; size_of::<T>()],
            transmute_copy::<T, [u8; size_of::<T>()]>(value),
        )
    }
}

//Set Bits are allowed to be used including those that have a canonical constrain (x86_64)
//Basicly the mask ony ensure that the address is valid for the given page size, general address constrains (max bits/canonical) are handled with the inner type
//This layer of the wrapper assumes full 64bit virt address space with no other constrains, further constrains are handled with the inner type (PhysAddress/VirtAddress)
//A value of 0 indicates no support for a given pagesize
//if LV2 is not supported LV3 is also not supported
lazy_static! {
    pub static ref LV1_PAGE_MASK: u64 = super::arch::memory::get_lv1_page_size_mask();
    pub static ref LV2_PAGE_MASK: u64 = super::arch::memory::get_lv2_page_size_mask();
    pub static ref LV3_PAGE_MASK: u64 = super::arch::memory::get_lv3_page_size_mask();
    pub static ref LV1_PAGE_SIZE: u64 = (!(*LV1_PAGE_MASK)) + 1;
    pub static ref LV2_PAGE_SIZE: u64 = (!(*LV2_PAGE_MASK)).wrapping_add(1); //If Pagesize isnt supported the u64 would overflow to 0, this prevent a panic
    pub static ref LV3_PAGE_SIZE: u64 = (!(*LV3_PAGE_MASK)).wrapping_add(1); //If Pagesize isnt supported the u64 would overflow to 0, this prevent a panic
    pub static ref LV2_PAGE_SUPPORTED: bool = *LV2_PAGE_MASK > 0;
    pub static ref LV3_PAGE_SUPPORTED: bool = (*LV3_PAGE_MASK > 0) && *LV2_PAGE_SUPPORTED;
}

//Wrapper that ensures that a address is always aligned to the base page size and a valid address
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PhysLv1PageAddress {
    address: PhysAddress,
}

impl PhysLv1PageAddress {
    #[inline]
    pub fn new(value: u64) -> Result<Self, MemoryAddressErrors> {
        if value & (!*LV1_PAGE_MASK) != 0 {
            return Err(MemoryAddressErrors::InvalidBits);
        }

        Ok(Self {
            address: PhysAddress::new(value)?,
        })
    }

    //masks of unsupported bits
    #[inline]
    pub fn new_maskoff(value: u64) -> Self {
        Self {
            address: PhysAddress::new_maskoff(value & *LV1_PAGE_MASK),
        }
    }

    ///Caller has to ensure that >value< fullfills platform address space constrains
    #[inline]
    pub unsafe fn new_unchecked(value: u64) -> Self {
        Self {
            address: PhysAddress::new_unchecked(value),
        }
    }

    #[inline]
    pub fn get_address(&self) -> PhysAddress {
        self.address
    }

    #[inline]
    pub fn offset(&self, count: i64) -> Result<Self, MemoryAddressErrors> {
        Self::new(
            self.get_address()
                .get_u64()
                .checked_add_signed(*LV1_PAGE_SIZE as i64 * count)
                .ok_or(MemoryAddressErrors::OutOfBounds)?,
        )
    }

    #[inline]
    pub fn offset_maskoff(&self, count: i64) -> Result<Self, MemoryAddressErrors> {
        Ok(Self::new_maskoff(
            self.get_address()
                .get_u64()
                .checked_add_signed(*LV1_PAGE_SIZE as i64 * count)
                .ok_or(MemoryAddressErrors::OutOfBounds)?,
        ))
    }

    ///Caller has to ensure that the new address fullfills platform address space constrains
    #[inline]
    pub unsafe fn offset_unchecked(&self, count: i64) -> Self {
        Self::new_unchecked(
            self.get_address()
                .get_u64()
                .wrapping_add_signed(*LV1_PAGE_SIZE as i64 * count),
        )
    }

    ///Converts the physical Address into a virtual Address via the HHDM
    #[inline]
    pub fn to_virt(&self) -> Result<VirtLv1PageAddress, MemoryAddressErrors> {
        VirtLv1PageAddress::new(self.get_address().get_u64() + (*HHDM_OFFSET).address.get_u64())
    }

    ///Converts the physical Address into a virtual Address via the HHDM \
    ///masks of unsupported bits
    #[inline]
    pub fn to_virt_maskoff(&self) -> Result<VirtLv1PageAddress, MemoryAddressErrors> {
        VirtLv1PageAddress::new_maskoff(
            self.get_address().get_u64() + (*HHDM_OFFSET).address.get_u64(),
        )
    }

    ///Converts the physical Address into a virtual Address via the HHDM \
    ///Caller has to ensure that >value< fullfills platform address space constrains
    #[inline]
    pub unsafe fn to_virt_unchecked(&self) -> VirtLv1PageAddress {
        VirtLv1PageAddress::new_unchecked(
            self.get_address().get_u64() + (*HHDM_OFFSET).address.get_u64(),
        )
    }

    ///Performs a volatile Read to physical Memory via the HHDM \
    ///The caller has to ensure that the target page is mapped with the needed permissions \
    ///The caller has to properly handle invalid instances of T returned by this function
    #[inline(always)]
    pub unsafe fn read<T>(&self) -> Result<T, MemoryAddressErrors>
    where
        [(); size_of::<T>()]:,
    {
        Ok(self.to_virt()?.read::<T>())
    }

    ///Performs a volatile Read to physical Memory via the HHDM \
    ///masks of unsupported bits \
    ///The caller has to ensure that the target page is mapped with the needed permissions \
    ///The caller has to properly handle invalid instances of T returned by this function \
    #[inline(always)]
    pub unsafe fn read_maskoff<T>(&self) -> Result<T, MemoryAddressErrors>
    where
        [(); size_of::<T>()]:,
    {
        Ok(self.to_virt_maskoff()?.read::<T>())
    }

    ///Performs a volatile Read to physical Memory via the HHDM \
    ///Caller has to ensure that >value< fullfills platform address space constrains \
    ///The caller has to ensure that the target page is mapped with the needed permissions \
    ///The caller has to properly handle invalid instances of T returned by this function
    #[inline(always)]
    pub unsafe fn read_unchecked<T>(&self) -> T
    where
        [(); size_of::<T>()]:,
    {
        self.to_virt_unchecked().read::<T>()
    }

    ///Performs a volatile Write to physical Memory via the HHDM \
    ///The caller has to ensure that the target page is mapped with the needed permissions
    #[inline(always)]
    pub unsafe fn write<T>(&self, value: &T) -> Result<(), MemoryAddressErrors>
    where
        [(); size_of::<T>()]:,
    {
        Ok(self.to_virt()?.write::<T>(value))
    }

    ///Performs a volatile Write to physical Memory via the HHDM \
    ///masks of unsupported bits \
    ///The caller has to ensure that the target page is mapped with the needed permissions
    #[inline(always)]
    pub unsafe fn write_maskoff<T>(&self, value: &T) -> Result<(), MemoryAddressErrors>
    where
        [(); size_of::<T>()]:,
    {
        Ok(self.to_virt_maskoff()?.write::<T>(value))
    }

    ///Performs a volatile Write to physical Memory via the HHDM \
    ///Caller has to ensure that >value< fullfills platform address space constrains \
    ///The caller has to ensure that the target page is mapped with the needed permissions
    #[inline(always)]
    pub unsafe fn write_unchecked<T>(&self, value: &T)
    where
        [(); size_of::<T>()]:,
    {
        self.to_virt_unchecked().write::<T>(value)
    }
}

//Wrapper that ensures that a address is always aligned to the page size one level above the base page size
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PhysLv2PageAddress {
    address: PhysAddress,
}

impl PhysLv2PageAddress {
    #[inline]
    pub fn new(value: u64) -> Result<Self, MemoryAddressErrors> {
        if value & (!*LV2_PAGE_MASK) != 0 {
            return Err(MemoryAddressErrors::InvalidBits);
        }

        Ok(Self {
            address: PhysAddress::new(value)?,
        })
    }

    ///masks of unsupported bits
    #[inline]
    pub fn new_maskoff(value: u64) -> Self {
        Self {
            address: PhysAddress::new_maskoff(value & *LV2_PAGE_MASK),
        }
    }

    ///Caller has to ensure that >value< fullfills platform address space constrains
    #[inline]
    pub unsafe fn new_unchecked(value: u64) -> Self {
        Self {
            address: PhysAddress::new_unchecked(value),
        }
    }

    #[inline]
    pub fn get_address(&self) -> PhysAddress {
        self.address
    }

    #[inline]
    pub fn offset(&self, count: i64) -> Result<Self, MemoryAddressErrors> {
        Self::new(
            self.get_address()
                .get_u64()
                .checked_add_signed(*LV2_PAGE_SIZE as i64 * count)
                .ok_or(MemoryAddressErrors::OutOfBounds)?,
        )
    }

    #[inline]
    pub fn offset_maskoff(&self, count: i64) -> Result<Self, MemoryAddressErrors> {
        Ok(Self::new_maskoff(
            self.get_address()
                .get_u64()
                .checked_add_signed(*LV2_PAGE_SIZE as i64 * count)
                .ok_or(MemoryAddressErrors::OutOfBounds)?,
        ))
    }

    ///Caller has to ensure that the new address fullfills platform address space constrains
    #[inline]
    pub unsafe fn offset_unchecked(&self, count: i64) -> Self {
        Self::new_unchecked(
            self.get_address()
                .get_u64()
                .wrapping_add_signed(*LV2_PAGE_SIZE as i64 * count),
        )
    }

    ///Converts the physical Address into a virtual Address via the HHDM
    #[inline]
    pub fn to_virt(&self) -> Result<VirtLv2PageAddress, MemoryAddressErrors> {
        VirtLv2PageAddress::new(self.get_address().get_u64() + (*HHDM_OFFSET).address.get_u64())
    }

    ///Converts the physical Address into a virtual Address via the HHDM \
    ///masks of unsupported bits
    #[inline]
    pub fn to_virt_maskoff(&self) -> Result<VirtLv2PageAddress, MemoryAddressErrors> {
        VirtLv2PageAddress::new_maskoff(
            self.get_address().get_u64() + (*HHDM_OFFSET).address.get_u64(),
        )
    }

    ///Converts the physical Address into a virtual Address via the HHDM \
    ///Caller has to ensure that >value< fullfills platform address space constrains
    #[inline]
    pub unsafe fn to_virt_unchecked(&self) -> VirtLv2PageAddress {
        VirtLv2PageAddress::new_unchecked(
            self.get_address().get_u64() + (*HHDM_OFFSET).address.get_u64(),
        )
    }

    ///Performs a volatile Read to physical Memory via the HHDM \
    ///The caller has to ensure that the target page is mapped with the needed permissions \
    ///The caller has to properly handle invalid instances of T returned by this function
    #[inline(always)]
    pub unsafe fn read<T>(&self) -> Result<T, MemoryAddressErrors>
    where
        [(); size_of::<T>()]:,
    {
        Ok(self.to_virt()?.read::<T>())
    }

    ///Performs a volatile Read to physical Memory via the HHDM \
    ///masks of unsupported bits \
    ///The caller has to ensure that the target page is mapped with the needed permissions \
    ///The caller has to properly handle invalid instances of T returned by this function \
    #[inline(always)]
    pub unsafe fn read_maskoff<T>(&self) -> Result<T, MemoryAddressErrors>
    where
        [(); size_of::<T>()]:,
    {
        Ok(self.to_virt_maskoff()?.read::<T>())
    }

    ///Performs a volatile Read to physical Memory via the HHDM \
    ///Caller has to ensure that >value< fullfills platform address space constrains \
    ///The caller has to ensure that the target page is mapped with the needed permissions \
    ///The caller has to properly handle invalid instances of T returned by this function
    #[inline(always)]
    pub unsafe fn read_unchecked<T>(&self) -> T
    where
        [(); size_of::<T>()]:,
    {
        self.to_virt_unchecked().read::<T>()
    }

    ///Performs a volatile Write to physical Memory via the HHDM \
    ///The caller has to ensure that the target page is mapped with the needed permissions
    #[inline(always)]
    pub unsafe fn write<T>(&self, value: &T) -> Result<(), MemoryAddressErrors>
    where
        [(); size_of::<T>()]:,
    {
        Ok(self.to_virt()?.write::<T>(value))
    }

    ///Performs a volatile Write to physical Memory via the HHDM \
    ///masks of unsupported bits \
    ///The caller has to ensure that the target page is mapped with the needed permissions
    #[inline(always)]
    pub unsafe fn write_maskoff<T>(&self, value: &T) -> Result<(), MemoryAddressErrors>
    where
        [(); size_of::<T>()]:,
    {
        Ok(self.to_virt_maskoff()?.write::<T>(value))
    }

    ///Performs a volatile Write to physical Memory via the HHDM \
    ///Caller has to ensure that >value< fullfills platform address space constrains \
    ///The caller has to ensure that the target page is mapped with the needed permissions
    #[inline(always)]
    pub unsafe fn write_unchecked<T>(&self, value: &T)
    where
        [(); size_of::<T>()]:,
    {
        self.to_virt_unchecked().write::<T>(value)
    }
}

//Wrapper that ensures that a address is always aligned to the page size two level above the base page size
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PhysLv3PageAddress {
    address: PhysAddress,
}

impl PhysLv3PageAddress {
    #[inline]
    pub fn new(value: u64) -> Result<Self, MemoryAddressErrors> {
        if value & (!*LV3_PAGE_MASK) != 0 {
            return Err(MemoryAddressErrors::InvalidBits);
        }

        Ok(Self {
            address: PhysAddress::new(value)?,
        })
    }

    //masks of unsupported bits
    #[inline]
    pub fn new_maskoff(value: u64) -> Self {
        Self {
            address: PhysAddress::new_maskoff(value & *LV3_PAGE_MASK),
        }
    }

    ///Caller has to ensure that >value< fullfills platform address space constrains
    #[inline]
    pub unsafe fn new_unchecked(value: u64) -> Self {
        Self {
            address: PhysAddress::new_unchecked(value),
        }
    }

    #[inline]
    pub fn get_address(&self) -> PhysAddress {
        self.address
    }

    #[inline]
    pub fn offset(&self, count: i64) -> Result<Self, MemoryAddressErrors> {
        Self::new(
            self.get_address()
                .get_u64()
                .checked_add_signed(*LV3_PAGE_SIZE as i64 * count)
                .ok_or(MemoryAddressErrors::OutOfBounds)?,
        )
    }

    #[inline]
    pub fn offset_maskoff(&self, count: i64) -> Result<Self, MemoryAddressErrors> {
        Ok(Self::new_maskoff(
            self.get_address()
                .get_u64()
                .checked_add_signed(*LV3_PAGE_SIZE as i64 * count)
                .ok_or(MemoryAddressErrors::OutOfBounds)?,
        ))
    }

    ///Caller has to ensure that the new address fullfills platform address space constrains
    #[inline]
    pub unsafe fn offset_unchecked(&self, count: i64) -> Self {
        Self::new_unchecked(
            self.get_address()
                .get_u64()
                .wrapping_add_signed(*LV3_PAGE_SIZE as i64 * count),
        )
    }

    ///Converts the physical Address into a virtual Address via the HHDM
    #[inline]
    pub fn to_virt(&self) -> Result<VirtLv3PageAddress, MemoryAddressErrors> {
        VirtLv3PageAddress::new(self.get_address().get_u64() + (*HHDM_OFFSET).address.get_u64())
    }

    ///Converts the physical Address into a virtual Address via the HHDM \
    ///masks of unsupported bits
    #[inline]
    pub fn to_virt_maskoff(&self) -> Result<VirtLv3PageAddress, MemoryAddressErrors> {
        VirtLv3PageAddress::new_maskoff(
            self.get_address().get_u64() + (*HHDM_OFFSET).address.get_u64(),
        )
    }

    ///Converts the physical Address into a virtual Address via the HHDM \
    ///Caller has to ensure that >value< fullfills platform address space constrains
    #[inline]
    pub unsafe fn to_virt_unchecked(&self) -> VirtLv3PageAddress {
        VirtLv3PageAddress::new_unchecked(
            self.get_address().get_u64() + (*HHDM_OFFSET).address.get_u64(),
        )
    }

    ///Performs a volatile Read to physical Memory via the HHDM \
    ///The caller has to ensure that the target page is mapped with the needed permissions \
    ///The caller has to properly handle invalid instances of T returned by this function
    #[inline(always)]
    pub unsafe fn read<T>(&self) -> Result<T, MemoryAddressErrors>
    where
        [(); size_of::<T>()]:,
    {
        Ok(self.to_virt()?.read::<T>())
    }

    ///Performs a volatile Read to physical Memory via the HHDM \
    ///masks of unsupported bits \
    ///The caller has to ensure that the target page is mapped with the needed permissions \
    ///The caller has to properly handle invalid instances of T returned by this function \
    #[inline(always)]
    pub unsafe fn read_maskoff<T>(&self) -> Result<T, MemoryAddressErrors>
    where
        [(); size_of::<T>()]:,
    {
        Ok(self.to_virt_maskoff()?.read::<T>())
    }

    ///Performs a volatile Read to physical Memory via the HHDM \
    ///Caller has to ensure that >value< fullfills platform address space constrains \
    ///The caller has to ensure that the target page is mapped with the needed permissions \
    ///The caller has to properly handle invalid instances of T returned by this function
    #[inline(always)]
    pub unsafe fn read_unchecked<T>(&self) -> T
    where
        [(); size_of::<T>()]:,
    {
        self.to_virt_unchecked().read::<T>()
    }

    ///Performs a volatile Write to physical Memory via the HHDM \
    ///The caller has to ensure that the target page is mapped with the needed permissions
    #[inline(always)]
    pub unsafe fn write<T>(&self, value: &T) -> Result<(), MemoryAddressErrors>
    where
        [(); size_of::<T>()]:,
    {
        Ok(self.to_virt()?.write::<T>(value))
    }

    ///Performs a volatile Write to physical Memory via the HHDM \
    ///masks of unsupported bits \
    ///The caller has to ensure that the target page is mapped with the needed permissions
    #[inline(always)]
    pub unsafe fn write_maskoff<T>(&self, value: &T) -> Result<(), MemoryAddressErrors>
    where
        [(); size_of::<T>()]:,
    {
        Ok(self.to_virt_maskoff()?.write::<T>(value))
    }

    ///Performs a volatile Write to physical Memory via the HHDM \
    ///Caller has to ensure that >value< fullfills platform address space constrains \
    ///The caller has to ensure that the target page is mapped with the needed permissions
    #[inline(always)]
    pub unsafe fn write_unchecked<T>(&self, value: &T)
    where
        [(); size_of::<T>()]:,
    {
        self.to_virt_unchecked().write::<T>(value)
    }
}

//Wrapper that ensures that a address is always aligned to the base page size
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VirtLv1PageAddress {
    address: VirtAddress,
}

impl VirtLv1PageAddress {
    #[inline]
    pub fn new(value: u64) -> Result<Self, MemoryAddressErrors> {
        if value & (!*LV1_PAGE_MASK) != 0 {
            return Err(MemoryAddressErrors::InvalidBits);
        }

        Ok(Self {
            address: VirtAddress::new(value)?,
        })
    }

    //masks of unsupported bits
    #[inline]
    pub fn new_maskoff(value: u64) -> Result<Self, MemoryAddressErrors> {
        Ok(Self {
            address: VirtAddress::new_maskoff(value & *LV1_PAGE_MASK)?,
        })
    }

    ///Caller has to ensure that >value< fullfills platform address space constrains
    #[inline]
    pub unsafe fn new_unchecked(value: u64) -> Self {
        Self {
            address: VirtAddress::new_unchecked(value),
        }
    }

    #[inline]
    pub fn get_address(&self) -> VirtAddress {
        self.address
    }

    #[inline]
    pub fn offset(&self, count: i64) -> Result<Self, MemoryAddressErrors> {
        Self::new(
            self.get_address()
                .get_u64()
                .checked_add_signed(*LV1_PAGE_SIZE as i64 * count)
                .ok_or(MemoryAddressErrors::OutOfBounds)?,
        )
    }

    #[inline]
    pub fn offset_maskoff(&self, count: i64) -> Result<Self, MemoryAddressErrors> {
        Self::new_maskoff(
            self.get_address()
                .get_u64()
                .checked_add_signed(*LV1_PAGE_SIZE as i64 * count)
                .ok_or(MemoryAddressErrors::OutOfBounds)?,
        )
    }

    ///Caller has to ensure that the new address fullfills platform address space constrains
    #[inline]
    pub unsafe fn offset_unchecked(&self, count: i64) -> Self {
        Self::new_unchecked(
            self.get_address()
                .get_u64()
                .wrapping_add_signed(*LV1_PAGE_SIZE as i64 * count),
        )
    }

    ///Converts the virtual Address into a physical Address via the HHDM
    #[inline]
    pub fn to_phys(&self) -> Result<PhysLv1PageAddress, MemoryAddressErrors> {
        if self.get_address().get_u64() < (*HHDM_OFFSET).address.get_u64() {
            return Err(MemoryAddressErrors::Invalid);
        }

        PhysLv1PageAddress::new(self.get_address().get_u64() - (*HHDM_OFFSET).address.get_u64())
    }

    ///Converts the virtual Address into a physical Address via the HHDM \
    ///masks of unsupported bits
    #[inline]
    pub fn to_phys_maskoff(&self) -> PhysLv1PageAddress {
        if self.get_address().get_u64() < (*HHDM_OFFSET).address.get_u64() {
            return PhysLv1PageAddress::new_maskoff(0);
        }

        PhysLv1PageAddress::new_maskoff(
            self.get_address().get_u64() - (*HHDM_OFFSET).address.get_u64(),
        )
    }

    ///Converts the virtual Address into a physical Address via the HHDM \
    ///Caller has to ensure that >value< fullfills platform address space constrains
    #[inline]
    pub unsafe fn to_phys_unchecked(&self) -> PhysLv1PageAddress {
        PhysLv1PageAddress::new_unchecked(
            self.get_address().get_u64() - (*HHDM_OFFSET).address.get_u64(),
        )
    }

    ///Performs a volatile Read from virtual Memory \
    ///The caller has to ensure that the target page is mapped with the needed permissions \
    ///The caller has to properly handle invalid instances of T returned by this function
    #[inline(always)]
    pub unsafe fn read<T>(&self) -> T
    where
        [(); size_of::<T>()]:,
    {
        self.address.read::<T>()
    }

    ///Performs a volatile Write to virtual Memory \
    ///The caller has to ensure that the target page is mapped with the needed permissions
    #[inline(always)]
    pub unsafe fn write<T>(&self, value: &T)
    where
        [(); size_of::<T>()]:,
    {
        self.address.write::<T>(value)
    }
}

//Wrapper that ensures that a address is always aligned to the page size one level above the base page size
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VirtLv2PageAddress {
    address: VirtAddress,
}

impl VirtLv2PageAddress {
    #[inline]
    pub fn new(value: u64) -> Result<Self, MemoryAddressErrors> {
        if value & (!*LV2_PAGE_MASK) != 0 {
            return Err(MemoryAddressErrors::InvalidBits);
        }

        Ok(Self {
            address: VirtAddress::new(value)?,
        })
    }

    #[inline]
    //masks of unsupported bits
    pub fn new_maskoff(value: u64) -> Result<Self, MemoryAddressErrors> {
        Ok(Self {
            address: VirtAddress::new_maskoff(value & *LV2_PAGE_MASK)?,
        })
    }

    #[inline]
    ///Caller has to ensure that >value< fullfills platform address space constrains
    pub unsafe fn new_unchecked(value: u64) -> Self {
        Self {
            address: VirtAddress::new_unchecked(value),
        }
    }

    #[inline]
    pub fn get_address(&self) -> VirtAddress {
        self.address
    }

    #[inline]
    pub fn offset(&self, count: i64) -> Result<Self, MemoryAddressErrors> {
        Self::new(
            self.get_address()
                .get_u64()
                .checked_add_signed(*LV2_PAGE_SIZE as i64 * count)
                .ok_or(MemoryAddressErrors::OutOfBounds)?,
        )
    }

    #[inline]
    pub fn offset_maskoff(&self, count: i64) -> Result<Self, MemoryAddressErrors> {
        Self::new_maskoff(
            self.get_address()
                .get_u64()
                .checked_add_signed(*LV2_PAGE_SIZE as i64 * count)
                .ok_or(MemoryAddressErrors::OutOfBounds)?,
        )
    }

    ///Caller has to ensure that the new address fullfills platform address space constrains
    #[inline]
    pub unsafe fn offset_unchecked(&self, count: i64) -> Self {
        Self::new_unchecked(
            self.get_address()
                .get_u64()
                .wrapping_add_signed(*LV2_PAGE_SIZE as i64 * count),
        )
    }

    #[inline]
    pub fn to_phys(&self) -> Result<PhysLv2PageAddress, MemoryAddressErrors> {
        if self.get_address().get_u64() < (*HHDM_OFFSET).address.get_u64() {
            return Err(MemoryAddressErrors::Invalid);
        }

        PhysLv2PageAddress::new(self.get_address().get_u64() - (*HHDM_OFFSET).address.get_u64())
    }

    ///Converts the virtual Address into a physical Address via the HHDM \
    ///masks of unsupported bits
    #[inline]
    pub fn to_phys_maskoff(&self) -> PhysLv2PageAddress {
        if self.get_address().get_u64() < (*HHDM_OFFSET).address.get_u64() {
            return PhysLv2PageAddress::new_maskoff(0);
        }

        PhysLv2PageAddress::new_maskoff(
            self.get_address().get_u64() - (*HHDM_OFFSET).address.get_u64(),
        )
    }

    ///Converts the virtual Address into a physical Address via the HHDM \
    ///Caller has to ensure that >value< fullfills platform address space constrains
    #[inline]
    pub unsafe fn to_phys_unchecked(&self) -> PhysLv2PageAddress {
        PhysLv2PageAddress::new_unchecked(
            self.get_address().get_u64() - (*HHDM_OFFSET).address.get_u64(),
        )
    }

    ///Performs a volatile Read to virtual Memory \
    ///The caller has to ensure that the target page is mapped with the needed permissions \
    ///The caller has to properly handle invalid instances of T returned by this function
    #[inline(always)]
    pub unsafe fn read<T>(&self) -> T
    where
        [(); size_of::<T>()]:,
    {
        self.address.read::<T>()
    }

    ///Performs a volatile Write to virtual Memory \
    ///The caller has to ensure that the target page is mapped with the needed permissions
    #[inline(always)]
    pub unsafe fn write<T>(&self, value: &T)
    where
        [(); size_of::<T>()]:,
    {
        self.address.write::<T>(value)
    }
}

//Wrapper that ensures that a address is always aligned to the page size two level above the base page size
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VirtLv3PageAddress {
    address: VirtAddress,
}

impl VirtLv3PageAddress {
    #[inline]
    pub fn new(value: u64) -> Result<Self, MemoryAddressErrors> {
        if value & (!*LV3_PAGE_MASK) != 0 {
            return Err(MemoryAddressErrors::InvalidBits);
        }

        Ok(Self {
            address: VirtAddress::new(value)?,
        })
    }

    ///masks of unsupported bits
    #[inline]
    pub fn new_maskoff(value: u64) -> Result<Self, MemoryAddressErrors> {
        Ok(Self {
            address: VirtAddress::new_maskoff(value & *LV3_PAGE_MASK)?,
        })
    }

    ///Caller has to ensure that >value< fullfills platform address space constrains
    #[inline]
    pub unsafe fn new_unchecked(value: u64) -> Self {
        Self {
            address: VirtAddress::new_unchecked(value),
        }
    }

    #[inline]
    pub fn get_address(&self) -> VirtAddress {
        self.address
    }

    #[inline]
    pub fn offset(&self, count: i64) -> Result<Self, MemoryAddressErrors> {
        Self::new(
            self.get_address()
                .get_u64()
                .checked_add_signed(*LV3_PAGE_SIZE as i64 * count)
                .ok_or(MemoryAddressErrors::OutOfBounds)?,
        )
    }

    #[inline]
    pub fn offset_maskoff(&self, count: i64) -> Result<Self, MemoryAddressErrors> {
        Self::new_maskoff(
            self.get_address()
                .get_u64()
                .checked_add_signed(*LV3_PAGE_SIZE as i64 * count)
                .ok_or(MemoryAddressErrors::OutOfBounds)?,
        )
    }

    ///Caller has to ensure that the new address fullfills platform address space constrains
    #[inline]
    pub unsafe fn offset_unchecked(&self, count: i64) -> Self {
        Self::new_unchecked(
            self.get_address()
                .get_u64()
                .wrapping_add_signed(*LV3_PAGE_SIZE as i64 * count),
        )
    }

    ///Converts the virtual Address into a physical Address via the HHDM
    #[inline]
    pub fn to_phys(&self) -> Result<PhysLv3PageAddress, MemoryAddressErrors> {
        if self.get_address().get_u64() < (*HHDM_OFFSET).address.get_u64() {
            return Err(MemoryAddressErrors::Invalid);
        }

        PhysLv3PageAddress::new(self.get_address().get_u64() - (*HHDM_OFFSET).address.get_u64())
    }

    ///Converts the virtual Address into a physical Address via the HHDM \
    ///masks of unsupported bits
    #[inline]
    pub fn to_phys_maskoff(&self) -> PhysLv3PageAddress {
        if self.get_address().get_u64() < (*HHDM_OFFSET).address.get_u64() {
            return PhysLv3PageAddress::new_maskoff(0);
        }

        PhysLv3PageAddress::new_maskoff(
            self.get_address().get_u64() - (*HHDM_OFFSET).address.get_u64(),
        )
    }

    ///Converts the virtual Address into a physical Address via the HHDM \
    ///Caller has to ensure that >value< fullfills platform address space constrains
    #[inline]
    pub unsafe fn to_phys_unchecked(&self) -> PhysLv3PageAddress {
        PhysLv3PageAddress::new_unchecked(
            self.get_address().get_u64() - (*HHDM_OFFSET).address.get_u64(),
        )
    }

    ///Performs a volatile Read to virtual Memory \
    ///The caller has to ensure that the target page is mapped with the needed permissions \
    ///The caller has to properly handle invalid instances of T returned by this function
    #[inline(always)]
    pub unsafe fn read<T>(&self) -> T
    where
        [(); size_of::<T>()]:,
    {
        self.address.read::<T>()
    }

    ///Performs a volatile Write to virtual Memory \
    ///The caller has to ensure that the target page is mapped with the needed permissions
    #[inline(always)]
    pub unsafe fn write<T>(&self, value: &T)
    where
        [(); size_of::<T>()]:,
    {
        self.address.write::<T>(value)
    }
}

/*
///General Purpose Optimized Memory Functions for larger Memory Operations

pub mod utility {
    use core::mem::size_of;

    use alloc::boxed::Box;
    use x86_64::VirtAddr;

    #[inline(always)]
    pub fn memset_1(ptr: VirtAddr, value: bool, size: usize) {
        unsafe { memset_core(ptr.as_u64() as *mut u8, value, size) }
    }

    #[inline(always)]
    pub fn memset_2<T>(ptr: &mut Box<T>, value: bool) {
        unsafe {
            memset_core(
                core::ptr::addr_of_mut!(*ptr) as *mut u8,
                value,
                size_of::<T>(),
            )
        }
    }

    #[inline(always)]
    unsafe fn memcpy_1(ptr_src: VirtAddr, ptr_dst: VirtAddr, size: usize) {
        memcpy_core(
            ptr_src.as_u64() as *mut u8,
            ptr_dst.as_u64() as *mut u8,
            size,
        )
    }

    compile_error!("TODO");

    #[inline(always)]
    unsafe fn memset_core(ptr: *mut u8, value: bool, size: usize) {
        //arch::memory::memset()
    }

    #[inline(always)]
    unsafe fn memcpy_core(ptr_src: *mut u8, ptr_dst: *mut u8, size: usize) {
        //arch::memory::memcpy()
    }
}
    */
