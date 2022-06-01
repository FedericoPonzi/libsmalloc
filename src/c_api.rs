use nix::libc::{c_int, c_void, intptr_t, memcpy, memset, sbrk, size_t};
use std::cell::UnsafeCell;
use std::mem;

static mut HEAP_BASE: Option<&UnsafeCell<Allocation>> = None;
const MAGIC: u32 = 0xdeadbeef;

#[repr(C)]
#[derive(Debug)]
struct Allocation {
    size: usize,
    is_freed: bool,
    next_block: Option<&'static UnsafeCell<Allocation>>,
    magic: u32, //todo: remove in release mode.
}
impl Allocation {}

unsafe fn request_space(size: usize) -> Option<&'static UnsafeCell<Allocation>> {
    let p: *mut c_void = sbrk(0);
    let metadata_and_size = size + mem::size_of::<UnsafeCell<Allocation>>();
    let request = sbrk((metadata_and_size) as intptr_t);
    assert_eq!(p, request); // no thread safe.
    if request == !0 as *mut c_void {
        //eprintln!("sbrk failed.");
        return None;
    }
    /*/eprintln!(
        "Request: {:?}, size: {}, final size: {} ",
        request, size, metadata_and_size,
    );*/
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
unsafe fn find_free(base: &'static UnsafeCell<Allocation>, size: usize) -> FreeResult {
    let mut ret = Some(base);
    let mut i = 0;
    loop {
        let r_cell = ret.unwrap();
        let r = &*r_cell.get();
        if (*r).is_freed && (*r).size > size {
            //eprintln!("Found fit!");
            return FreeResult::Fit(r_cell);
        }
        i += 1;
        if r.next_block.is_none() {
            //eprintln!("Found last non empty :) {}", i);
            return FreeResult::LastNonEmpty(r_cell);
        }
        ret = r.next_block;
    }
}
///
/// # Safety
/// Always check the return.
#[no_mangle]
pub unsafe extern "C" fn malloc(size: usize) -> *const u8 {
    //eprintln!("malloc");
    if size == 0 {
        return std::ptr::null::<u8>();
    }
    // align up:
    let size = size % 8 + size;

    let allocation = match HEAP_BASE {
        Some(base) => {
            let find_free_block = find_free(HEAP_BASE.unwrap(), size);
            match find_free_block {
                FreeResult::Fit(alloc) => {
                    (*alloc.get()).is_freed = false;
                    alloc
                }
                FreeResult::LastNonEmpty(mut last) => {
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
    let alloc = (allocation as *const UnsafeCell<Allocation> as *const u8);
    let alloc_skip_metadatas = alloc.add(mem::size_of::<Allocation>());
    //eprintln!("Final position: {:?}", alloc_skip_metadatas);
    alloc_skip_metadatas
}
unsafe fn get_metadata_from_alloc_ptr(ptr: *const u8) -> &'static UnsafeCell<Allocation> {
    let ptr_metadata = ptr.sub(mem::size_of::<Allocation>());
    &*(ptr_metadata as *const UnsafeCell<Allocation>)
}

///
/// # Safety
/// Always check the return.
#[no_mangle]
pub unsafe extern "C" fn free(ptr: *const u8) {
    if ptr.is_null() {
        return;
    }
    //eprintln!("free, ptr: {:?}", ptr);
    let metadata = get_metadata_from_alloc_ptr(ptr);
    let metadata_inner = &mut *metadata.get();
    if metadata_inner.magic != MAGIC {
        //eprintln!("Magic is broken! something went wrong.");
        panic!("");
    }
    metadata_inner.is_freed = true;
    //eprintln!("Freed {:?}, size: {}", ptr, metadata_inner.size);
}

///
/// void *realloc(void *ptr, size_t size)
/// # Safety
/// Always check the return., ptr should be valid.
#[no_mangle]
pub unsafe extern "C" fn realloc(ptr: *const u8, size: usize) -> *const u8 {
    //eprintln!("realloc: {:?}", ptr);
    if ptr.is_null() {
        return malloc(size);
    }

    let metadata = get_metadata_from_alloc_ptr(ptr);
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
