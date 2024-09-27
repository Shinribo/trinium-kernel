
pub(crate) type IrqLevel = super::arch::interrupt::ArchIrqLevel;

pub(crate) const MASK_ALL: IrqLevel = super::arch::interrupt::MASK_ALL;




///Is reentrant safe as the IrqLevel is a core local hardware mechanism
pub(super) unsafe fn set_irq_level(value: IrqLevel){
    super::arch::interrupt::set_irq_level(value);
}

///Is reentrant safe as the IrqLevel is a core local hardware mechanism \
///Increases the IrqLevel to the requested level if the current Level is lower \
///returns the old IrqLevel
pub(super) unsafe  fn bump_irq_level(value: IrqLevel) -> IrqLevel{
    super::arch::interrupt::bump_irq_level(value)
}