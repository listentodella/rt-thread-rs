use crate::{ffi::*, RtName};
use alloc::string::String;
use core::convert::Infallible;
use embedded_hal::digital::{ErrorType, InputPin, OutputPin, StatefulOutputPin};
use num_enum::IntoPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u8)]
#[allow(dead_code)]
enum RtPinMode {
    Output,
    Input,
    InputPullUp,
    InputPullDown,
    OutputOpenDrain,
}

struct RtPin {
    pin: rt_base_t,
}

#[allow(dead_code)]
impl RtPin {
    pub fn new(name: &str, mode: RtPinMode) -> Self {
        let pin = unsafe { rt_pin_get(RtName::from(name).into()) };
        let this = Self { pin };
        this.set_mode(mode);
        this
    }
    pub fn set_mode(&self, mode: RtPinMode) {
        unsafe { rt_pin_mode(self.pin, mode.into()) };
    }
}

impl TryFrom<&str> for RtPin {
    type Error = String;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        let pin = unsafe { rt_pin_get(RtName::from(name).into()) };
        if pin < 0 {
            return Err(alloc::format!("pin {} not found", name));
        }
        Ok(Self { pin })
    }
}

pub struct RtOutputPin {
    inner: RtPin,
}

pub struct RtInputPin {
    inner: RtPin,
}

impl RtOutputPin {
    pub fn new(name: &str) -> Self {
        let inner = RtPin::try_from(name).unwrap();
        inner.set_mode(RtPinMode::Output);
        Self { inner }
    }
}

impl ErrorType for RtOutputPin {
    type Error = Infallible;
}

impl OutputPin for RtOutputPin {
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe { rt_pin_write(self.inner.pin, 1) };
        Ok(())
    }

    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe { rt_pin_write(self.inner.pin, 0) };
        Ok(())
    }
}

impl ErrorType for RtInputPin {
    type Error = Infallible;
}

impl RtInputPin {
    pub fn new(name: &str) -> Self {
        let inner = RtPin::try_from(name).unwrap();
        inner.set_mode(RtPinMode::Input);
        Self { inner }
    }
}

impl InputPin for RtInputPin {
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(unsafe { rt_pin_read(self.inner.pin) == 1 })
    }

    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(unsafe { rt_pin_read(self.inner.pin) == 0 })
    }
}

impl StatefulOutputPin for RtOutputPin {
    #[inline]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(unsafe { rt_pin_read(self.inner.pin) == 1 })
    }

    #[inline]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(unsafe { rt_pin_read(self.inner.pin) == 0 })
    }
}
