#![no_std]
extern crate alloc;
use alloc::{ffi::CString, vec};
use assign_resources::assign_resources;
use core::sync::atomic::{AtomicU32, Ordering};
use critical_section::CriticalSection;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::peripherals;
use embassy_stm32::spi;
use embassy_stm32::time::mhz;
use embassy_stm32::Config;
use embedded_hal::delay::DelayNs as _;

#[allow(non_camel_case_types, non_upper_case_globals, unused, non_snake_case)]
use rustffi::println;
use rustffi::{cstr, delay};

static CNT: AtomicU32 = AtomicU32::new(0);
static mut LED: Option<Output> = None;

assign_resources! {
    // usb: UsbHwResources {
    //     dp: PA12,
    //     dm: PA11,
    //     usb: USB_OTG_FS,
    // }
    // lcd: LcdHwResources {
    //     cs: PE11,
    //     sck: PE12,
    //     mosi: PE14,
    //     //txdma: DMA1_CH3,
    //     dc: PE15,
    //     bl: PD15,
    //     spi: SPI4,
    // }
    imu: ImuHwResources {
        spi: SPI3,
        sck: PB3,
        mosi: PB5,
        miso: PB4,
        txdma: DMA1_CH3,
        rxdma: DMA1_CH4,
        cs: PB7,
        int1: PB8,
        exti:EXTI8
    }
    led: LedHwResources {
        pin: PC13,
    }
}

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
    if LED.is_none() {
        let p = embassy_stm32::Peripherals::steal();
        let r = split_resources!(p);
        //请注意，当输出被丢弃时，引脚将返回其浮动状态
        //如果引脚应无限期地保留其状态，则保留输出的所有权，或将其传递给 core::mem::forget
        //当然，前提是如果是在局部创建的话...
        LED = Some(Output::new(r.led.pin, Level::High, Speed::High));
    }

    if let Some(ref mut led) = LED {
        led.toggle();
    }
}

#[no_mangle]
pub extern "C" fn rust_imu() {
    let mut delay = delay::Delay;
    let p = unsafe { embassy_stm32::Peripherals::steal() };
    let r = split_resources!(p);
    let mut spi_config = spi::Config::default();
    spi_config.frequency = mhz(16);
    unsafe {
        let cs = CriticalSection::new();
        embassy_stm32::rcc::enable_and_reset_with_cs::<peripherals::SPI3>(cs);
    }
    // let mut spi = spi::Spi::new(
    //     r.imu.spi,
    //     r.imu.sck,
    //     r.imu.mosi,
    //     r.imu.miso,
    //     r.imu.txdma,
    //     r.imu.rxdma,
    //     spi_config,
    // );
    let mut spi = spi::Spi::new_blocking(r.imu.spi, r.imu.sck, r.imu.mosi, r.imu.miso, spi_config);
    let mut cs = Output::new(r.imu.cs, Level::High, Speed::High);

    let buf = [0x11u8, 0x01];
    cs.set_low();
    spi.blocking_write(&buf).unwrap();
    cs.set_high();
    delay.delay_ms(200);

    cs.set_low();
    let buf = [0x75u8 | 0x80];
    let mut chipid = [0x00u8];
    spi.blocking_write(&buf).unwrap();
    spi.blocking_read(&mut chipid).unwrap();
    cs.set_high();
    println!("chipid = {:?}", chipid);
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
