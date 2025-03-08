#![no_std]
extern crate alloc;
use alloc::vec;
use core::sync::atomic::{AtomicU32, Ordering};

#[allow(non_camel_case_types, non_upper_case_globals, unused, non_snake_case)]
use rustffi::println;

static CNT: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn rust_main() -> u32 {
    let a = vec![1, 2, 3];
    let cnt = CNT.fetch_add(2, Ordering::SeqCst);
    println!("Hello, Rust {} {:?}", cnt, a);
    cnt
}
