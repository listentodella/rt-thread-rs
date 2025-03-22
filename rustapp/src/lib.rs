#![no_std]
extern crate alloc;
use alloc::{ffi::CString, vec};
use core::sync::atomic::{AtomicU32, Ordering};
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::time::mhz;
use embassy_stm32::Config;
use embedded_hal::delay::DelayNs;

#[allow(non_camel_case_types, non_upper_case_globals, unused, non_snake_case)]
use rustffi::println;
use rustffi::{cstr, delay};

static CNT: AtomicU32 = AtomicU32::new(0);
static mut LED: Option<Output> = None;

#[no_mangle]
#[allow(dead_code)]
pub extern "C" fn rust_system_clock_init() {
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: mhz(25),
            mode: HseMode::Oscillator,
        });
        config.rcc.hsi = None;
        config.rcc.csi = false;

        config.rcc.hsi48 = Some(Hsi48Config {
            sync_from_usb: true,
        }); // needed for USB
        config.rcc.pll1 = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV5,
            mul: PllMul::MUL160,
            divp: Some(PllDiv::DIV2),
            divq: Some(PllDiv::DIV4),
            //divr: None,
            divr: Some(PllDiv::DIV2),
        });
        config.rcc.sys = Sysclk::PLL1_P; // 400 Mhz
        config.rcc.ahb_pre = AHBPrescaler::DIV2; // 200 Mhz
        config.rcc.apb1_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb2_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb3_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb4_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.voltage_scale = VoltageScale::Scale1;
        // config.rcc.mux.usbsel = mux::Usbsel::HSI48;
    }
    let _p = embassy_stm32::init(config);
}

#[no_mangle]
pub unsafe extern "C" fn rust_led() {
    let p = embassy_stm32::Peripherals::steal();
    if LED.is_none() {
        //请注意，当输出被丢弃时，引脚将返回其浮动状态
        //如果引脚应无限期地保留其状态，则保留输出的所有权，或将其传递给 core::mem::forget
        //当然，前提是如果是在局部创建的话...
        LED = Some(Output::new(p.PC13, Level::High, Speed::High));
    }

    if let Some(ref mut led) = LED {
        led.toggle();
    }
}

#[no_mangle]
pub extern "C" fn rust_main() -> u32 {
    let a = vec![1, 2, 3];
    let cnt = CNT.fetch_add(2, Ordering::SeqCst);
    println!("Hello, Rust {} {:?}", cnt, a);

    let c_string = CString::new("HelloW").unwrap();
    let bytes = c_string.as_bytes();
    println!("bytes = {:?}", bytes);

    let a = cstr::RtName::from("Hello");
    println!("a = {:?}", a);

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
pub extern "C" fn rust_str() -> *const i8 {
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
    a.as_ptr()
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
