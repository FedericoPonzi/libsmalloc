use nix::libc::{c_int, c_void, intptr_t, memset, sbrk, size_t};

///
/// # Safety
/// Always check the return.
#[no_mangle]
pub unsafe extern "C" fn malloc(size: usize) -> *const usize {
    if size == 0 {
        return std::ptr::null::<usize>();
    }
    //eprintln!("malloc: requested: {}", size);
    let p: *mut c_void = sbrk(0);
    let req: *mut c_void = sbrk(size as intptr_t);
    if req == !0 as *mut c_void {
        std::ptr::null::<usize>()
    } else {
        assert_eq!(p, req);
        p as *const usize
    }
}

///
/// # Safety
/// Always check the return.
#[no_mangle]
pub unsafe extern "C" fn free(ptr: *const usize) {}

///
/// void *realloc(void *ptr, size_t size)
/// # Safety
/// Always check the return., ptr should be valid.
#[no_mangle]
pub unsafe extern "C" fn realloc(ptr: *const usize, size: usize) -> *const usize {
    malloc(size)
}

/// void *calloc(size_t nelem, size_t elsize) {
/// # Safety
/// Always check the return.
#[no_mangle]
pub unsafe extern "C" fn calloc(nelem: usize, elsize: usize) -> *const usize {
    let size = nelem * elsize;
    //eprintln!("calloc: requested: {}", size);
    let ptr = malloc(size);
    memset(ptr as *mut c_void, nelem as c_int, elsize as size_t);
    ptr
}
