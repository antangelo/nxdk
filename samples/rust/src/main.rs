#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let msg = cstr_core::CString::new("Hello from Rust!\n").expect("Unable to alloc string");

    unsafe {
        nxdk_rs::hal::video::XVideoSetMode(640, 480, 32, 0);
        nxdk_rs::hal::debug::debugPrint(msg.as_ptr() as *const libc::c_char);
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
