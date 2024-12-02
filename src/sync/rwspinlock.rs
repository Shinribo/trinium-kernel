
use core::{cell::UnsafeCell, fmt, hint, ops::{Deref, DerefMut}, sync::atomic::{AtomicUsize, Ordering}};
use crate::hal::interrupt::{bump_irq_level, set_irq_level, IrqLevel};

pub struct RwSpinlock<T> {
    ticket_queue: AtomicUsize,
    ticket_enter: AtomicUsize,
    ticket_exit: AtomicUsize,
    required_irqlv: IrqLevel,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for RwSpinlock<T> {}
unsafe impl<T: Send> Send for RwSpinlock<T> {}

impl<T> RwSpinlock<T> {
    
    #[inline(always)]
    pub const fn new(data: T, new_irqlv: IrqLevel) -> Self {
        Self {
            ticket_queue: AtomicUsize::new(0),
            ticket_enter: AtomicUsize::new(0),
            ticket_exit: AtomicUsize::new(0),
            required_irqlv: new_irqlv,
            data: UnsafeCell::new(data),
        }
    }

    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        self.is_locked_read() || self.is_locked_mut()
    }

    #[inline(always)]
    pub fn is_locked_read(&self) -> bool {

        //tests if locking is in progress queue > enter = exit
        self.ticket_queue.load(Ordering::Acquire) != self.ticket_enter.load(Ordering::Acquire) 
        &&
        self.ticket_queue.load(Ordering::Acquire) != self.ticket_exit.load(Ordering::Acquire)

        ||

        //tests for a read lock => queue = enter > exit
        self.ticket_queue.load(Ordering::Acquire) == self.ticket_enter.load(Ordering::Acquire) 
        &&
        self.ticket_queue.load(Ordering::Acquire) != self.ticket_exit.load(Ordering::Acquire)
    }

    #[inline(always)]
    pub fn is_locked_mut(&self) -> bool {

        //tests if locking is in progress queue > enter = exit
        self.ticket_queue.load(Ordering::Acquire) != self.ticket_enter.load(Ordering::Acquire) 
        &&
        self.ticket_queue.load(Ordering::Acquire) != self.ticket_exit.load(Ordering::Acquire)

        ||

        //tests for a write lock queue = exit > enter
        self.ticket_queue.load(Ordering::Acquire) != self.ticket_enter.load(Ordering::Acquire) 
        &&
        self.ticket_queue.load(Ordering::Acquire) == self.ticket_exit.load(Ordering::Acquire)
    }

    ///Safety: When nesting ensure that the Guards are released in the correct order
    #[inline(always)]
    pub unsafe fn lock(&self) -> ReadGuard<T> {
        
        let old_irqlv = unsafe { bump_irq_level(self.required_irqlv) };

        //enter queue
        let ticket = self.ticket_queue.fetch_add(1, Ordering::Relaxed);

        //wait for read enter
        while ticket != self.ticket_enter.load(Ordering::Acquire) {
            hint::spin_loop();
        }

        //as this is a read access the next thread can enter this imediatly
        self.ticket_enter.fetch_add(1, Ordering::AcqRel);

        ReadGuard { 
            ticket_exit: &self.ticket_exit, 
            old_irqlv: old_irqlv, 
            data: unsafe { & *self.data.get() } 
        }
        
    }

    ///Safety: When nesting ensure that the Guards are released in the correct order
    #[inline(always)]
    pub unsafe fn lock_mut(&self) -> WriteGuard<T> {
    
        let old_irqlv = unsafe { bump_irq_level(self.required_irqlv) };

        //enter queue
        let ticket = self.ticket_queue.fetch_add(1, Ordering::Relaxed);
        
        //wait for read enter
        while ticket != self.ticket_enter.load(Ordering::Acquire) {
            hint::spin_loop();
        }

        //Waits until all Readers have left the critical section
        while self.ticket_enter.load(Ordering::Acquire) != self.ticket_exit.load(Ordering::Acquire) {
            hint::spin_loop();
        }        

        self.ticket_exit.fetch_add(1, Ordering::Release);

        WriteGuard { 
            ticket_enter: &self.ticket_enter, 
            old_irqlv, 
            data: unsafe { &mut *self.data.get() } 
        }


    }


}




pub struct ReadGuard<'a, T: 'a> {
    ticket_exit: &'a AtomicUsize,
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
        unsafe { set_irq_level(self.old_irqlv) };
        self.ticket_exit.fetch_add(1, Ordering::AcqRel);
    }
}

pub struct WriteGuard<'a, T: 'a> {
    ticket_enter: &'a AtomicUsize,
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
        unsafe { set_irq_level(self.old_irqlv) };
        self.ticket_enter.store(self.ticket_enter.load(Ordering::Acquire) + 1, Ordering::Release);
    }
}
