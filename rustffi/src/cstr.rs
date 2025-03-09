#![allow(dead_code)]
use crate::ffi::RT_NAME_MAX;

use core::ffi::c_char;
use core::str::from_utf8_unchecked;

#[allow(non_camel_case_types)]
pub type c_str = *const c_char;
#[allow(non_camel_case_types)]
pub type c_mut_str = *mut c_char;

type NameArray = [u8; RT_NAME_MAX as usize];

#[derive(Clone, Default)]
pub struct RtName {
    buf: NameArray,
}

impl RtName {
    pub fn new(buf: NameArray) -> Self {
        RtName { buf }
    }

    #[inline]
    pub fn as_array_str(&self) -> &[u8] {
        &self.buf
    }

    #[inline]
    pub fn as_mut_array_str(&mut self) -> &mut [u8] {
        &mut self.buf
    }
}

impl From<&str> for RtName {
    fn from(value: &str) -> Self {
        let mut buf = [0u8; RT_NAME_MAX as usize];
        let bytes = value.as_bytes();
        let len = bytes.len().clamp(0, RT_NAME_MAX as usize);
        buf[..len].copy_from_slice(&bytes[..len]);
        RtName { buf }
    }
}

impl Into<c_str> for RtName {
    #[inline]
    fn into(self) -> c_str {
        /*
        as 与 cast 写法在此处均正确，但 .cast() 是更现代、更推荐的方式，因为：
        明确表达“指针类型转换”的意图。
        在泛型或复杂类型场景中更易维护。
        与 Rust 标准库的 API 设计风格一致。
         */
        self.buf.as_ptr().cast()
    }
}

impl Into<c_mut_str> for RtName {
    #[inline]
    fn into(mut self) -> c_mut_str {
        /*
        as 与 cast 写法在此处均正确，但 .cast() 是更现代、更推荐的方式，因为：
        明确表达“指针类型转换”的意图。
        在泛型或复杂类型场景中更易维护。
        与 Rust 标准库的 API 设计风格一致。
         */
        self.buf.as_mut_ptr().cast()
    }
}

pub struct RtNameRef<'a> {
    buf: &'a NameArray,
}

impl<'a> RtNameRef<'a> {
    pub fn new(buf: &'a NameArray) -> Self {
        RtNameRef { buf }
    }
}

impl<'a> From<&'a [u8; RT_NAME_MAX as usize]> for RtNameRef<'a> {
    #[inline]
    fn from(buf: &'a [u8; RT_NAME_MAX as usize]) -> Self {
        RtNameRef { buf }
    }
}
impl<'a> From<&'a [i8; RT_NAME_MAX as usize]> for RtNameRef<'a> {
    #[inline]
    fn from(buf: &'a [i8; RT_NAME_MAX as usize]) -> Self {
        RtNameRef {
            buf: unsafe { core::mem::transmute(buf) },
        }
    }
}

impl<'a> Into<&'a str> for RtNameRef<'a> {
    #[inline]
    fn into(self) -> &'a str {
        unsafe { from_utf8_unchecked(self.buf) }
    }
}
impl<'a> AsRef<str> for RtNameRef<'a> {
    #[inline]
    fn as_ref(&self) -> &str {
        unsafe { from_utf8_unchecked(self.buf) }
    }
}
