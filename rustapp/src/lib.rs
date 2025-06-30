#![no_std]
extern crate alloc;
use alloc::{ffi::CString, vec};
use core::future::pending;
use core::sync::atomic::{AtomicU32, Ordering};
use embedded_hal::delay::DelayNs;
use embedded_hal::spi::{SpiBus, SpiDevice};

#[allow(non_camel_case_types, non_upper_case_globals, unused, non_snake_case)]
use rustffi::println;
use rustffi::{cstr, delay};

static CNT: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn rust_main() -> u32 {
    let a = vec![1, 2, 3];
    let cnt = CNT.fetch_add(2, Ordering::SeqCst);
    println!("Hello, Rust {} {:?}", cnt, a);

    let c_string = CString::new("HelloW").unwrap();
    let bytes = c_string.as_bytes();
    println!("bytes = {:?}", bytes);

    // let a = cstr::RtName::from("Hello");
    // println!("a = {:?}", a);

    // let bytes = unsafe {
    // core::slice::from_raw_parts_mut(bytes.as_ptr() as *const u8 as *mut u8, bytes.len() + 1)
    // };
    // bytes[bytes.len() - 1] = b'\0';
    // let b = core::ffi::CStr::from_bytes_until_nul(bytes).unwrap();
    let b = unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(bytes) };
    println!(
        "b len = {},  {}->{:?}",
        b.count_bytes(),
        b.to_str().unwrap(),
        b
    );

    let c_string = CString::new("HelloW").unwrap();
    let c_string = c_string.into_bytes_with_nul();
    let b = core::ffi::CStr::from_bytes_with_nul(&c_string).unwrap();
    println!(
        "b len = {},  {}->{:?}",
        b.count_bytes(),
        b.to_str().unwrap(),
        b
    );
    cnt
}

#[no_mangle]
pub extern "C" fn rust_str() -> *const u8 {
    // a's lifetime is not long enough...
    // let a = unsafe { alloc::ffi::CString::from_vec_unchecked("rust_CString".into()) };
    // println!("get a = {:?}", a);
    // let a = a.as_ptr();
    // a

    // this is ok, becasue we use leak... must free it manually
    // let c_str = CString::new("rust_CString").expect("Invalid CString");
    // let boxed = Box::new(c_str);
    // Box::leak(boxed).as_ptr()

    static C_STR: &str = "rust_CString";
    let a = unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(C_STR.as_bytes()) };
    a.as_ptr() as *const u8
}

#[no_mangle]
pub extern "C" fn create_mq() {
    let mq = rustffi::sync::mq::MessageQueue::<u32>::new("testmq", 10).unwrap();
    println!("mq = {:?}", mq);
    mq.send(1);
    mq.blocking_send(2, 100);

    let a = mq.recv().unwrap();
    println!("recv = {}", a);
    core::mem::forget(mq);
}

#[no_mangle]
pub extern "C" fn rust_test_spi() {
    let mut spidev = match rustffi::spi::RtSpiDevice::new("spi10", "spi1", "PB.6") {
        Ok(spi) => spi,
        Err(e) => {
            println!("Failed to create SPI device: {:?}", e);
            return;
        }
    };
    println!("spi get success");

    let rt_spi_config = rustffi::ffi::rt_spi_configuration {
        mode: 0 | 1 << 2,
        data_width: 8,
        reserved: 0,
        max_hz: 8000000,
    };

    spidev.configure(&rt_spi_config).unwrap_or_else(|e| {
        println!("spi configure failed: {:?}", e);
    });

    let mut delay = delay::Delay;
    let mut cnt = 0u8;

    while cnt < 10 {
        cnt += 1;

        let mut buf = [0u8; 2];
        buf[0] = 0x80 | 0x02;
        buf[1] = 0x80 | 0x02;

        spidev.transfer_in_place(&mut buf).unwrap();
        println!("SPI transfer successful, buf = {:?}", buf);
        delay.delay_ms(1000);
    }
}
