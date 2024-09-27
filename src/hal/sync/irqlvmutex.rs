
use core::{cell::UnsafeCell, fmt, ops::{Deref, DerefMut}, sync::atomic::{AtomicBool, Ordering}};

use crate::hal::interrupt::{IrqLevel, set_irq_level,bump_irq_level};

use super::WaitStrategy;

pub struct IrqLvMutex<T> {
    lock: AtomicBool,
    required_irqlv: IrqLevel,
    wait_strategy: WaitStrategy,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for IrqLvMutex<T> {}
unsafe impl<T: Send> Send for IrqLvMutex<T> {}


impl<T> IrqLvMutex<T> {
    
    #[inline(always)]
    pub const fn new(data: T, new_irqlv: IrqLevel, strategy: WaitStrategy) -> Self {
        Self {
            lock: AtomicBool::new(false),
            required_irqlv: new_irqlv,
            wait_strategy: strategy,
            data: UnsafeCell::new(data),
        }
    }

    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        self.lock.load(Ordering::Relaxed)
    }

    ///Safety: When nesting ensure that the Guards are released in the correct order
    #[inline(always)]
    pub unsafe fn lock(&self) -> MutexGuard<T> {
        
        if let Some(mutex) = self.try_lock() {
            return mutex;
        }

        match self.wait_strategy {
            WaitStrategy::Spin => {
                loop {
                    
                    if let Some(mutex) = self.try_lock() {
                        return mutex;
                    }

                    core::hint::spin_loop();

                }
            },

            WaitStrategy::Yield => todo!(),
        }

    }


    #[inline(always)]
    pub fn try_lock(&self) -> Option<MutexGuard<T>> {

        let old_irqlv = unsafe { bump_irq_level(self.required_irqlv) };

        if self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(MutexGuard {
                lock: &self.lock,
                old_irqlv,
                data: unsafe { &mut *self.data.get() },
            })
        } else {
            unsafe { set_irq_level(old_irqlv) };
            None
        }
    }

    /// This function is allowed to spuriously fail even when the mutex is unlocked,
    /// which can result in more efficient code on some platforms.
    #[inline(always)]
    pub fn try_lock_weak(&self) -> Option<MutexGuard<T>> {

        let old_irqlv = unsafe { bump_irq_level(self.required_irqlv) };

        if self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(MutexGuard {
                lock: &self.lock,
                old_irqlv,
                data: unsafe { &mut *self.data.get() },
            })
        } else {
            unsafe { set_irq_level(old_irqlv) };
            None
        }
    }

}

pub struct MutexGuard<'a, T: 'a> {
    lock: &'a AtomicBool,
    old_irqlv: IrqLevel,
    data: *mut T,
}

impl<'a, T: fmt::Debug> fmt::Debug for MutexGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<'a, T: fmt::Display> fmt::Display for MutexGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // We know statically that only we are referencing data
        unsafe { &*self.data }
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        // We know statically that only we are referencing data
        unsafe { &mut *self.data }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    /// The dropping of the MutexGuard will release the lock it was created from.
    fn drop(&mut self) {
        unsafe { set_irq_level(self.old_irqlv) };
        self.lock.store(false, Ordering::Release);
    }
}


