//wrapper around cpu specific details for needed function

use core::arch::asm;

use crate::hal::interrupt::IrqLevel;

#[derive(Clone, Copy)]
pub struct ArchIrqLevel(u8);


pub(in crate::hal) const MASK_ALL: IrqLevel = ArchIrqLevel(15);


pub(in crate::hal) unsafe fn set_irq_level(value: IrqLevel) {
    asm!(
        " mov cr8, {}",
        in(reg_byte) value.0,
    )
}

pub(in crate::hal) unsafe fn bump_irq_level(value: IrqLevel) -> IrqLevel {
    
    let mut value_new: u8;
    
    asm!(
        " mov {0}, cr8",
        " movzbq cr8, {1}",
        out(reg_byte) value_new,
        in(reg_byte) value.0,
    );

    ArchIrqLevel(value_new)

}