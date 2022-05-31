use core::alloc::{GlobalAlloc, Layout};

pub use c_api::*;
mod align;
mod c_api;

pub struct SmallocAllocator;

unsafe impl Sync for SmallocAllocator {}

unsafe impl GlobalAlloc for SmallocAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        todo!()
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        todo!()
    }
}
