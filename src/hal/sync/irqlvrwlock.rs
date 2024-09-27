
use core::{cell::UnsafeCell, fmt, ops::{Deref, DerefMut}, sync::atomic::{AtomicUsize, Ordering}};

use super::{super::interrupt::{bump_irq_level, set_irq_level, IrqLevel}, WaitStrategy};

pub struct IrqLvRwLock<T> {
    lock: AtomicUsize, //0 - Write, 1 - Unlocked, >=2 - Read
    required_irqlv: IrqLevel,
    wait_strategy: WaitStrategy,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for IrqLvRwLock<T> {}
unsafe impl<T: Send> Send for IrqLvRwLock<T> {}

impl<T> IrqLvRwLock<T> {
    
    #[inline(always)]
    pub const fn new(data: T, new_irqlv: IrqLevel, strategy: WaitStrategy) -> Self {
        Self {
            lock: AtomicUsize::new(1),
            required_irqlv: new_irqlv,
            wait_strategy: strategy,
            data: UnsafeCell::new(data),
        }
    }

    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        self.lock.load(Ordering::Relaxed) != 1
    }

    #[inline(always)]
    pub fn is_locked_read(&self) -> bool {
        self.lock.load(Ordering::Relaxed) > 1
    }

    #[inline(always)]
    pub fn is_locked_mut(&self) -> bool {
        self.lock.load(Ordering::Relaxed) == 0
    }

    ///Safety: When nesting ensure that the Guards are released in the correct order
    #[inline(always)]
    pub unsafe fn lock(&self) -> ReadGuard<T> {
        
        if let Some(read_lock) = self.try_lock() {
            return read_lock;
        }

        match self.wait_strategy {
            WaitStrategy::Spin => {
                loop {
                    
                    if let Some(read_lock) = self.try_lock() {
                        return read_lock;
                    }

                    core::hint::spin_loop();

                }
            },

            WaitStrategy::Yield => todo!(),
        }

    }

    ///Safety: When nesting ensure that the Guards are released in the correct order
    #[inline(always)]
    pub unsafe fn lock_mut(&self) -> WriteGuard<T> {
        
        if let Some(write_lock) = self.try_lock_mut() {
            return write_lock;
        }

        match self.wait_strategy {
            WaitStrategy::Spin => {
                loop {
                    
                    if let Some(write_lock) = self.try_lock_mut() {
                        return write_lock;
                    }

                    core::hint::spin_loop();

                }
            },

            WaitStrategy::Yield => todo!(),
        }

    }

    ///Safety: When nesting ensure that the Guards are released in the correct order
    #[inline(always)]
    pub fn try_lock(&self) -> Option<ReadGuard<T>> {

        let current = self.lock.load(Ordering::Release);

        if let Some(new) = current.checked_add(1) {
            
            let old_irqlv = unsafe { bump_irq_level(self.required_irqlv) };

            if self
                .lock
                .compare_exchange(current, new, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                return Some(ReadGuard {
                    lock: &self.lock,
                    old_irqlv,
                    data: unsafe { &mut *self.data.get() },
                });
            }

            unsafe { set_irq_level(old_irqlv) };
        }
 
        None

    }

    ///Safety: When nesting ensure that the Guards are released in the correct order
    /// This function is allowed to spuriously fail even when the mutex is unlocked,
    /// which can result in more efficient code on some platforms.
    #[inline(always)]
    pub fn try_lock_weak(&self) -> Option<ReadGuard<T>> {

        let current = self.lock.load(Ordering::Release);

        if let Some(new) = current.checked_add(1) {
            
            let old_irqlv = unsafe { bump_irq_level(self.required_irqlv) };

            if self
                .lock
                .compare_exchange_weak(current, new, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                return Some(ReadGuard {
                    lock: &self.lock,
                    old_irqlv,
                    data: unsafe { &mut *self.data.get() },
                });
            }

            unsafe { set_irq_level(old_irqlv) };
        }
 
        None
    }

    ///Safety: When nesting ensure that the Guards are released in the correct order
    #[inline(always)]
    pub fn try_lock_mut(&self) -> Option<WriteGuard<T>> {

        let old_irqlv = unsafe { bump_irq_level(self.required_irqlv) };

        if self
            .lock
            .compare_exchange(1, 0, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(WriteGuard {
                lock: &self.lock,
                old_irqlv,
                data: unsafe { &mut *self.data.get() },
            })
        } else {
            unsafe { set_irq_level(old_irqlv) };
            None
        }

    }

    ///Safety: When nesting ensure that the Guards are released in the correct order
    /// This function is allowed to spuriously fail even when the mutex is unlocked,
    /// which can result in more efficient code on some platforms.
    #[inline(always)]
    pub fn try_lock_mut_weak(&self) -> Option<WriteGuard<T>> {

        let old_irqlv = unsafe { bump_irq_level(self.required_irqlv) };

        if self
            .lock
            .compare_exchange_weak(1, 0, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(WriteGuard {
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

pub struct ReadGuard<'a, T: 'a> {
    lock: &'a AtomicUsize,
    old_irqlv: IrqLevel,
    data: *const T,
}

impl<'a, T: fmt::Debug> fmt::Debug for ReadGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(& **self, f)
    }
}

impl<'a, T: fmt::Display> fmt::Display for ReadGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(& **self, f)
    }
}

impl<'a, T> Deref for ReadGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // We know statically that only we are referencing data
        unsafe { &*self.data }
    }
}

impl<'a, T> Drop for ReadGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.fetch_sub(1, Ordering::Release);
    }
}

pub struct WriteGuard<'a, T: 'a> {
    lock: &'a AtomicUsize,
    old_irqlv: IrqLevel,
    data: *mut T,
}

impl<'a, T: fmt::Debug> fmt::Debug for WriteGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(& **self, f)
    }
}

impl<'a, T: fmt::Display> fmt::Display for WriteGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(& **self, f)
    }
}

impl<'a, T> Deref for WriteGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // We know statically that only we are referencing data
        unsafe { & *self.data }
    }
}

impl<'a, T> DerefMut for WriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        // We know statically that only we are referencing data
        unsafe { &mut *self.data }
    }
}

impl<'a, T> Drop for WriteGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.store(1, Ordering::Relaxed);
    }
}
