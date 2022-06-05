use core::cell::UnsafeCell;
use core::mem;
use libc::{c_void, intptr_t, memcpy, memset, sbrk, size_t};

static mut HEAP_BASE: Option<&UnsafeCell<Allocation>> = None;
const MAGIC: u32 = 0xdeadbeef;
const METADATA_SIZE: usize = mem::size_of::<UnsafeCell<Allocation>>();

#[repr(C)]
#[derive(Debug)]
struct Allocation {
    size: usize,
    is_freed: bool,
    next_block: Option<&'static UnsafeCell<Allocation>>,
    magic: u32, //todo: remove in release mode.
}

unsafe fn request_space(size: usize) -> Option<&'static UnsafeCell<Allocation>> {
    let p: *mut c_void = sbrk(0);
    let metadata_and_size = size + mem::size_of::<UnsafeCell<Allocation>>();
    let request = sbrk((metadata_and_size) as intptr_t);
    assert_eq!(p, request); // no thread safe.
    if request == !0 as *mut c_void {
        return None;
    }
    let ret: &mut UnsafeCell<Allocation> =
        &mut *(request as *const u8 as *mut UnsafeCell<Allocation>);
    let mut ret_mut = &mut *(ret.get());
    ret_mut.is_freed = false;
    ret_mut.size = size;
    ret_mut.next_block = None;
    ret_mut.magic = MAGIC;
    Some(ret)
}
enum FreeResult {
    Fit(&'static UnsafeCell<Allocation>),
    LastNonEmpty(&'static UnsafeCell<Allocation>),
}
/// Find an already allocated but freed block big enough to contain the new requested allocation
/// Traverses the linked list chain starting from base, and coalesces free blocks along the way.
/// TODO: if the last block is freed, we can return it and extend the memory by requesting the missing space.
unsafe fn find_free(base: &'static UnsafeCell<Allocation>, size: usize) -> FreeResult {
    let mut ret = Some(base);
    let mut i = 0;
    loop {
        let r_cell = ret.unwrap();
        let r = &mut *r_cell.get();
        //if this and the next block are free, coalesce them. - backward coaleshing.
        // We do this now to avoid traversing the linked list during the call to free.
        if r.is_freed && r.next_block.filter(|b| (*(*b).get()).is_freed).is_some() {
            let next = r.next_block.unwrap().get();
            r.next_block = (*next).next_block;
            r.size += (*next).size + METADATA_SIZE;
        }
        // if this block is available and enough big:
        if r.is_freed && r.size >= size {
            return FreeResult::Fit(r_cell);
        }
        i += 1;
        if r.next_block.is_none() {
            return FreeResult::LastNonEmpty(r_cell);
        }
        ret = r.next_block;
    }
}

unsafe fn split_block(allocation: &UnsafeCell<Allocation>, requested_space: usize) {
    let mut alloc = &mut *(allocation.get());
    let remaining_size = alloc.size as isize - requested_space as isize - METADATA_SIZE as isize;
    assert!(remaining_size > 0);
    alloc.size = requested_space;
    let alloc_ptr = alloc as *const Allocation as *const u8;
    let new_alloc_ptr = alloc_ptr.add(METADATA_SIZE + alloc.size);
    let new_metadata: &mut UnsafeCell<Allocation> =
        &mut *(new_alloc_ptr as *mut UnsafeCell<Allocation>);
    memset(
        new_metadata as *mut UnsafeCell<Allocation> as *mut c_void,
        0,
        METADATA_SIZE,
    );
    let mut new_alloc_inner = &mut *(new_metadata.get());
    new_alloc_inner.size = remaining_size as usize;
    new_alloc_inner.next_block = alloc.next_block;
    new_alloc_inner.is_freed = true;
    new_alloc_inner.magic = MAGIC;
    alloc.next_block = Some(new_metadata);
}

///
/// # Safety
/// Always check the return.
#[no_mangle]
pub unsafe extern "C" fn malloc(requested_size: usize) -> *const u8 {
    if requested_size == 0 {
        return core::ptr::null::<u8>();
    }
    // align up:
    let size = requested_size % 8 + requested_size;

    let allocation = match HEAP_BASE {
        Some(base) => {
            let find_free_block = find_free(base, size);
            match find_free_block {
                FreeResult::Fit(alloc) => {
                    (*alloc.get()).is_freed = false;
                    alloc
                }
                FreeResult::LastNonEmpty(last) => {
                    let new_alloc = request_space(size).expect("request_space failed!");
                    (*last.get()).next_block = Some(new_alloc);
                    new_alloc
                }
            }
        }
        None => {
            let ret = request_space(size).expect("request_space_failed!");
            HEAP_BASE = Some(ret);
            ret
        }
    };
    // Let's split the block in two blocks if possible:
    // Assume allocation block is 10, Metadata size is 1 and requested size is 5. IF we split the block,
    // We need enough space to accomodate a new metadata header + remaining space.
    if (*allocation.get()).size > size + (METADATA_SIZE) {
        split_block(allocation, size);
    }

    let alloc = allocation as *const UnsafeCell<Allocation> as *const u8;
    let alloc_skip_metadatas = alloc.add(METADATA_SIZE);
    //eprintln!("Final position: {:?}", alloc_skip_metadatas);
    alloc_skip_metadatas
}
unsafe fn get_metadata_from_alloc_ptr(ptr: *const u8) -> Option<&'static UnsafeCell<Allocation>> {
    let ptr_metadata = ptr.sub(METADATA_SIZE);
    let ret = &*(ptr_metadata as *const UnsafeCell<Allocation>);
    if (*ret.get()).magic != MAGIC {
        None
    } else {
        Some(ret)
    }
}

///
/// # Safety
/// Always check the return.
#[no_mangle]
pub unsafe extern "C" fn free(ptr: *const u8) {
    if ptr.is_null() {
        return;
    }
    let metadata = get_metadata_from_alloc_ptr(ptr).unwrap();
    let metadata_inner = &mut *metadata.get();
    if metadata_inner.magic != MAGIC {
        //TODO: review this panic.
        panic!("");
    }
    metadata_inner.is_freed = true;
    if let Some(next_block) = metadata_inner.next_block {
        if (*next_block.get()).is_freed {
            metadata_inner.next_block = (*next_block.get()).next_block;
            metadata_inner.size += (*next_block.get()).size + METADATA_SIZE
        }
    }
}

///
/// void *realloc(void *ptr, size_t size)
/// # Safety
/// Always check the return., ptr should be valid.
#[no_mangle]
pub unsafe extern "C" fn realloc(ptr: *const u8, size: usize) -> *const u8 {
    if ptr.is_null() {
        return malloc(size);
    }

    let metadata = get_metadata_from_alloc_ptr(ptr).unwrap();
    let metadata_inner = &mut *metadata.get();
    if size > metadata_inner.size {
        metadata_inner.is_freed = true;
        let ret = malloc(size);
        memcpy(
            ret as *mut c_void,
            ptr as *const c_void,
            metadata_inner.size as size_t,
        );

        ret
    } else {
        ptr
    }
}

/// void *calloc(size_t nelem, size_t elsize) {
/// # Safety
/// Always check the return.
#[no_mangle]
pub unsafe extern "C" fn calloc(nelem: usize, elsize: usize) -> *const u8 {
    //eprintln!("calloc");
    let size = nelem * elsize;
    //eprintln!("calloc: requested: {}", size);
    let ptr = malloc(size);
    memset(ptr as *mut c_void, 0_i32, size as size_t);
    ptr
}
