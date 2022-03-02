#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![feature(alloc_error_handler)]

macro_rules! binding {
    ($v:expr) => {
        include!(concat!(env!("OUT_DIR"), $v));
    };
}

pub mod hal {
    pub mod video {
        binding!("/bindings_video.rs");
    }

    pub mod debug {
        binding!("/bindings_debug.rs");
    }
}

pub mod clib {
    binding!("/bindings_pdclib.rs");
}

pub mod winapi {
    binding!("/bindings_windows.rs");
}

pub struct XboxKernelAlloc {}

unsafe impl core::alloc::GlobalAlloc for XboxKernelAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        clib::malloc(layout.size() as libc::c_uint) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        clib::free(ptr as *mut libc::c_void)
    }

    unsafe fn alloc_zeroed(&self, layout: core::alloc::Layout) -> *mut u8 {
        let csize = layout.size() as libc::c_uint;
        let mem = clib::malloc(csize);
        if mem == core::ptr::null_mut() {
            return core::ptr::null_mut();
        }

        clib::memset(mem, 0, csize);
        mem as *mut u8
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: core::alloc::Layout, new_size: usize) -> *mut u8 {
        clib::realloc(ptr as *mut libc::c_void, new_size as libc::c_uint) as *mut u8
    }
}

#[global_allocator]
static ALLOCATOR: XboxKernelAlloc = XboxKernelAlloc {};

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
