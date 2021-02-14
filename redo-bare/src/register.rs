
use crate::sync::Mutex;
use core::cell::UnsafeCell;
use core::marker::PhantomData;
use redo_cpu::lock::asm_try_lock;

pub trait Register: Sized {
    unsafe fn bare<'a>(address: usize) -> &'a mut Self {
        &mut *(address as *mut Self)
    }
}


pub const fn regi<T>(address: usize) -> &'static mut T {
    unsafe {
        &mut *(address as *mut T)
    }
}



pub struct PeriRegLock<T>
{
    lock_cell: UnsafeCell<usize>,
    address  : usize,
    nothing  : PhantomData<T>,
}


unsafe impl<T> Send for PeriRegLock<T> where T: Sized + Send {}
unsafe impl<T> Sync for PeriRegLock<T> where T: Sized + Send {}

impl<T> PeriRegLock<T> {

    pub const fn new(address: usize) -> Self {
        Self {
            lock_cell: UnsafeCell::new(1),
            address:   address,
            nothing:   PhantomData,
        }
    }
}

impl<T: Register + Sized> Mutex for PeriRegLock<T>
{
    type Data = T;

    fn lock<R>(&self, f: impl FnOnce(&mut Self::Data) -> R) -> R {

        // wait for lock
        unsafe {
            while asm_try_lock(self.lock_cell.get()) == 0 { }
        }

        let data = unsafe { Self::Data::bare(self.address) };
        let result = f(data);
        // after use
        unsafe {
            core::ptr::write_volatile(self.lock_cell.get(), 1);
        }
        return result;
    }
}
