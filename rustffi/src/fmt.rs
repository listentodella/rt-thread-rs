use crate::ffi;
use core::fmt;

// value from rtconfig.h
const RT_CONSOLEBUF_SIZE: usize = 64;

pub struct RtConsole;

impl fmt::Write for RtConsole {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut buf = [0; RT_CONSOLEBUF_SIZE];
        for chunk in s.as_bytes().chunks(RT_CONSOLEBUF_SIZE - 1) {
            buf[..chunk.len()].copy_from_slice(chunk);
            buf[chunk.len()] = 0;
            unsafe {
                ffi::rt_kputs(buf.as_ptr() as *const _);
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let _ = write!($crate::fmt::RtConsole, $($arg)*);
        }
    };
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let _ = writeln!($crate::fmt::RtConsole, $($arg)*);
        }
    };
}

#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut s = alloc::string::String::new();
            let _ = write!(&mut s, $($arg)*);
            s
        }
    };
    () => {
        String::new()
    };
}
