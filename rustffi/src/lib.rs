#![no_std]
extern crate alloc;

#[allow(non_camel_case_types, non_upper_case_globals, unused, non_snake_case)]
pub mod ffi;

mod allocator;
pub mod cstr;
pub mod delay;
pub mod fmt;
pub mod sync;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("PANIC:\n{}", _info);
    loop {}
}
