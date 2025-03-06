use crate::ffi;

pub struct Delay;

impl embedded_hal::delay::DelayNs for Delay {
    fn delay_ns(&mut self, _ns: u32) {
        todo!("impl nano second delay")
    }

    fn delay_ms(&mut self, ms: u32) {
        unsafe {
            ffi::rt_thread_mdelay(ms as _);
        }
    }

    fn delay_us(&mut self, us: u32) {
        unsafe {
            ffi::rt_hw_us_delay(us);
        }
    }
}
