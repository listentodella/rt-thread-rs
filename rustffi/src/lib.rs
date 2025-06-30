#![no_std]
extern crate alloc;

#[allow(non_camel_case_types, non_upper_case_globals, unused, non_snake_case)]
pub mod ffi;

mod allocator;
pub mod cstr;
pub mod delay;
pub mod fmt;
pub mod gpio;
pub mod spi;
pub mod sync;

pub use allocator::*;
pub use cstr::*;
pub use delay::*;
pub use ffi::*;
pub use fmt::*;
pub use gpio::*;
pub use spi::*;
pub use sync::*;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("PANIC:\n{}", _info);
    loop {}
}
