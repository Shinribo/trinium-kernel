use lock_api::{GuardNoSend, RawMutex, RawRwLock};

///Used to bypass the lock_api so that native locking methods can be utilized
pub struct NullLock();

unsafe impl RawMutex for NullLock {
    
    const INIT: Self = NullLock();

    type GuardMarker = GuardNoSend;

    fn lock(&self) {}

    fn try_lock(&self) -> bool {
        true
    }

    unsafe fn unlock(&self) {}

}

pub struct NullRwLock();

unsafe impl RawRwLock for NullRwLock {
    
    const INIT: Self = NullRwLock();

    type GuardMarker = GuardNoSend;

    fn lock_shared(&self) {}

    fn try_lock_shared(&self) -> bool {
        true
    }

    unsafe fn unlock_shared(&self) {}

    fn lock_exclusive(&self) {}

    fn try_lock_exclusive(&self) -> bool {
        true
    }

    unsafe fn unlock_exclusive(&self) {}
    
}
