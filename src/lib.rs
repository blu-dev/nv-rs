use std::usize;

use libc::c_void;
pub type AllocFunc = extern "C" fn(size: usize, align: usize, user: *mut c_void) -> *mut c_void;
pub type DeallocFunc = extern "C" fn(addr: *mut c_void, user: *mut c_void);
pub type ReallocFunc = extern "C" fn(addr: *mut c_void, size: usize, user: *mut c_void) -> *mut c_void;

extern "C" {
    #[link_name = "\u{1}_ZN2nv20SetGraphicsAllocatorEPFPvmmS0_EPFvS0_S0_EPFS0_S0_mS0_ES0_"]
    fn SetGraphicsAllocator(
        alloc: AllocFunc,
        dealloc: DeallocFunc,
        realloc: ReallocFunc,
        user: *mut c_void
    );

    #[link_name = "\u{1}_ZN2nv18InitializeGraphicsEPvm"]
    fn InitializeGraphics(
        memory: *mut c_void,
        size: usize
    );
}

pub fn set_allocators(alloc: AllocFunc, dealloc: DeallocFunc, realloc: ReallocFunc) {
    unsafe {
        SetGraphicsAllocator(alloc, dealloc, realloc, 0 as _);
    }
}

pub fn init(memory: *mut c_void, size: usize) {
    unsafe {
        InitializeGraphics(memory, size);
    }
}