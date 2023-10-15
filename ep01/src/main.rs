#![no_main]
#![no_std]

extern crate r_efi;

use r_efi::efi;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

const HELLO_STR: &str = "Hello, world. Press any key to return to UEFI firmware.";

#[export_name = "efi_main"]
pub extern "C" fn main(_h: efi::Handle, st: *mut efi::SystemTable) -> efi::Status {
    let mut s = [0u16; HELLO_STR.len() + 1];
    let mut i = 0usize;
    for c in HELLO_STR.encode_utf16() {
        s[i] = c;
        i += 1;
        if i >= s.len() {
            break;
        }
    }

    // Print "Hello World!".
    let r =
        unsafe { ((*(*st).con_out).output_string)((*st).con_out, s.as_ptr() as *mut efi::Char16) };
    if r.is_error() {
        return r;
    }

    // Wait for key input, by waiting on the `wait_for_key` event hook.
    let r = unsafe {
        let mut x: usize = 0;
        ((*(*st).boot_services).wait_for_event)(1, &mut (*(*st).con_in).wait_for_key, &mut x)
    };
    if r.is_error() {
        return r;
    }

    efi::Status::SUCCESS
}