use crate::{ffi::*, RtName};
use alloc::boxed::Box;
use embedded_hal::spi::{ErrorType, Operation, SpiDevice};

/// RT-Thread SPI 设备错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtSpiError {
    /// 设备未找到
    DeviceNotFound,
    /// 打开设备失败
    OpenFailed,
    /// 传输错误
    TransferError,
    /// 控制命令错误
    ControlError,
}

impl embedded_hal::spi::Error for RtSpiError {
    fn kind(&self) -> embedded_hal::spi::ErrorKind {
        match self {
            RtSpiError::DeviceNotFound => embedded_hal::spi::ErrorKind::Other,
            RtSpiError::OpenFailed => embedded_hal::spi::ErrorKind::Other,
            RtSpiError::TransferError => embedded_hal::spi::ErrorKind::Overrun,
            RtSpiError::ControlError => embedded_hal::spi::ErrorKind::Other,
        }
    }
}

/// RT-Thread SPI 设备封装
pub struct RtSpiDevice {
    dev: *mut rt_spi_device,
}

impl RtSpiDevice {
    /// 通过设备名查找SPI设备
    pub fn find(device_name: &str) -> Result<Self, RtSpiError> {
        let name = RtName::from(device_name);
        let dev = unsafe { rt_device_find(name.into()) };
        if dev.is_null() {
            return Err(RtSpiError::DeviceNotFound);
        }
        let spi_dev = dev as *mut rt_spi_device;
        Ok(Self { dev: spi_dev })
    }

    /// 通过总线名和片选引脚注册新设备
    pub fn new(device_name: &str, bus_name: &str, cs_pin: &str) -> Result<Self, RtSpiError> {
        let cs_pin = unsafe { rt_pin_get(RtName::from(cs_pin).into()) };

        // let spi_device = unsafe {
        //     rt_malloc(core::mem::size_of::<rt_spi_device>() as u32) as *mut rt_spi_device
        // };
        // if spi_device.is_null() {
        //     return Err(RtSpiError::OpenFailed);
        // }
        // use Box instead of malloc
        let spi_device = Box::new(unsafe { core::mem::zeroed::<rt_spi_device>() });
        let raw_dev_ptr = Box::into_raw(spi_device);
        let result = unsafe {
            rt_spi_bus_attach_device_cspin(
                raw_dev_ptr,
                RtName::from(device_name).into(),
                RtName::from(bus_name).into(),
                cs_pin,
                RT_NULL as *mut core::ffi::c_void,
            )
        };
        if result != 0 {
            return Err(RtSpiError::OpenFailed);
        }
        Ok(Self { dev: raw_dev_ptr })
    }

    pub fn configure(&mut self, config: &rt_spi_configuration) -> Result<(), RtSpiError> {
        let result = unsafe { rt_spi_configure(self.dev, config as *const _ as *mut _) };
        if result != 0 {
            return Err(RtSpiError::OpenFailed);
        }
        Ok(())
    }

    pub fn read(&mut self, words: &mut [u8]) -> Result<(), RtSpiError> {
        self.transfer(words, &[])?;
        Ok(())
    }
    pub fn write(&mut self, words: &[u8]) -> Result<(), RtSpiError> {
        self.transfer(&mut [], words)?;
        Ok(())
    }
    pub fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), RtSpiError> {
        let _ = unsafe {
            rt_spi_transfer(
                self.dev,
                write.as_ptr() as *const core::ffi::c_void,
                read.as_mut_ptr() as *mut core::ffi::c_void,
                write.len() as rt_size_t,
            )
        };
        Ok(())
    }
    pub fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), RtSpiError> {
        // 直接使用同一个缓冲区进行发送和接收，就像 C 版本的 rt_spi_transfer
        let _ = unsafe {
            rt_spi_transfer(
                self.dev,
                words.as_ptr() as *const core::ffi::c_void,
                words.as_mut_ptr() as *mut core::ffi::c_void,
                words.len() as rt_size_t,
            )
        };
        Ok(())
    }
}

impl ErrorType for RtSpiDevice {
    type Error = RtSpiError;
}

impl SpiDevice<u8> for RtSpiDevice {
    fn transaction(&mut self, operations: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
        for op in operations {
            match op {
                Operation::Read(buf) => self.read(buf)?,
                Operation::Write(buf) => self.write(buf)?,
                Operation::Transfer(read, write) => self.transfer(read, write)?,
                Operation::TransferInPlace(buf) => self.transfer_in_place(buf)?,
                Operation::DelayNs(_) => {}
            }
        }
        Ok(())
    }
}

// 外部 C 函数声明
extern "C" {
    fn rt_device_find(name: *const core::ffi::c_char) -> *mut rt_device;
}
