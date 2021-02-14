
// Redo Kernel
// Synchronization
// Copyright (c) 2021 Johann Li <me@qinka.pro>


use core::cell::UnsafeCell;
use redo_cpu::lock::asm_try_lock;



/** Any object implementing this trait guarantees exclusive access to the data wrapped within the Mutex for the duration of the provided closure.
*/
pub trait Mutex {
    /// The type of the data that is wrapped by this mutex.
    type Data;

    /// Locks the mutex and grants the closure temporary mutable access to the wrapped data.
    fn lock<R>(&self, f: impl FnOnce(&mut Self::Data) -> R) -> R;
}


pub struct PseudoLock<T>
where
    T: ?Sized,
{
    lock_cell: UnsafeCell<usize>,
    data:      UnsafeCell<T>,
}


unsafe impl<T> Send for PseudoLock<T> where T: ?Sized + Send {}
unsafe impl<T> Sync for PseudoLock<T> where T: ?Sized + Send {}

impl<T> PseudoLock<T> {

    pub const fn new(data: T) -> Self {
        Self {
            lock_cell: UnsafeCell::new(1),
            data:      UnsafeCell::new(data),
        }
    }
}

impl<T> Mutex for PseudoLock<T>
{
    type Data = T;

    fn lock<R>(&self, f: impl FnOnce(&mut Self::Data) -> R) -> R {

        // wait for lock
        unsafe {
            while asm_try_lock(self.lock_cell.get()) == 0 { }
        }

        let data = unsafe { &mut *self.data.get() };
        let result = f(data);
        // after use
        unsafe {
            core::ptr::write_volatile(self.lock_cell.get(), 1);
        }
        return result;
    }
}
