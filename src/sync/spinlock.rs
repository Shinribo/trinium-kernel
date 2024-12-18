use crate::hal::interrupt::{bump_irq_level, set_irq_level, IrqLevel};
use core::{
    cell::UnsafeCell,
    fmt, hint,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicUsize, Ordering},
};

pub struct Spinlock<T> {
    ticket_queue: AtomicUsize,
    ticket_enter: AtomicUsize,
    required_irqlv: IrqLevel,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for Spinlock<T> {}
unsafe impl<T: Send> Send for Spinlock<T> {}

impl<T> Spinlock<T> {
    #[inline(always)]
    pub const fn new(data: T, new_irqlv: IrqLevel) -> Self {
        Self {
            ticket_queue: AtomicUsize::new(0),
            ticket_enter: AtomicUsize::new(0),
            required_irqlv: new_irqlv,
            data: UnsafeCell::new(data),
        }
    }

    #[inline(always)]
    pub fn queue_length(&self) -> usize {
        self.ticket_queue.load(Ordering::Acquire) - self.ticket_enter.load(Ordering::Acquire)
    }

    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        self.ticket_queue.load(Ordering::Acquire) != self.ticket_enter.load(Ordering::Acquire)
    }

    ///Safety: When nesting ensure that the Guards are released in the correct order
    #[inline(always)]
    pub unsafe fn lock(&self) -> SpinlockGuard<T> {
        let old_irqlv = unsafe { bump_irq_level(self.required_irqlv) };

        //enter queue
        let ticket = self.ticket_queue.fetch_add(1, Ordering::Relaxed);

        //wait for turn
        while ticket != self.ticket_enter.load(Ordering::Acquire) {
            hint::spin_loop();
        }

        SpinlockGuard {
            ticket_enter: &self.ticket_enter,
            old_irqlv,
            data: unsafe { &mut *self.data.get() },
        }
    }
}

pub struct SpinlockGuard<'a, T: 'a> {
    ticket_enter: &'a AtomicUsize,
    old_irqlv: IrqLevel,
    data: *mut T,
}

impl<'a, T: fmt::Debug> fmt::Debug for SpinlockGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<'a, T: fmt::Display> fmt::Display for SpinlockGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<'a, T> Deref for SpinlockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // We know statically that only we are referencing data
        unsafe { &*self.data }
    }
}

impl<'a, T> DerefMut for SpinlockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        // We know statically that only we are referencing data
        unsafe { &mut *self.data }
    }
}

impl<'a, T> Drop for SpinlockGuard<'a, T> {
    /// The dropping of the MutexGuard will release the lock it was created from.
    fn drop(&mut self) {
        unsafe { set_irq_level(self.old_irqlv) };
        self.ticket_enter.store(
            self.ticket_enter.load(Ordering::Acquire) + 1,
            Ordering::Release,
        );
    }
}
