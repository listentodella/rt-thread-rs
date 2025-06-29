use crate::{ffi::*, println, RtName};
use core::marker::PhantomData;
use embedded_hal::spi::{ErrorType, Operation, SpiBus, SpiDevice};

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

/// RT-Thread SPI 总线适配器
pub struct RtSpiBus<CS> {
    device: *mut rt_spi_device,
    _cs: PhantomData<CS>,
}

impl<CS> RtSpiBus<CS> {
    /// 创建新的 SPI 总线实例
    pub fn new(device_name: &str) -> Result<Self, RtSpiError> {
        let device = unsafe {
            let name = RtName::from(device_name);
            println!("try to find device = {:?}", name);
            rt_device_find(name.into()) as *mut rt_spi_device
        };

        if device.is_null() {
            return Err(RtSpiError::DeviceNotFound);
        }

        Ok(Self {
            device,
            _cs: PhantomData,
        })
    }

    /// 释放设备
    pub fn close(&mut self) {}
}

impl<CS> ErrorType for RtSpiBus<CS> {
    type Error = RtSpiError;
}

impl<CS> SpiBus<u8> for RtSpiBus<CS> {
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.transfer(words, &[])?;
        Ok(())
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.transfer(&mut [], words)?;
        Ok(())
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        let _ = unsafe {
            rt_spi_transfer(
                self.device,
                write.as_ptr() as *const core::ffi::c_void,
                read.as_mut_ptr() as *mut core::ffi::c_void,
                write.len() as rt_size_t,
            )
        };

        Ok(())
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        let write_data = words.to_vec();
        self.transfer(words, &write_data)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// RT-Thread SPI 设备适配器（带片选）
pub struct RtSpiDevice<CS> {
    bus: RtSpiBus<CS>,
    _cs: PhantomData<CS>,
}

impl<CS> RtSpiDevice<CS> {
    /// 创建新的 SPI 设备实例
    pub fn new(device_name: &str) -> Result<Self, RtSpiError> {
        let bus = RtSpiBus::new(device_name)?;
        Ok(Self {
            bus,
            _cs: PhantomData,
        })
    }
}

impl<CS> ErrorType for RtSpiDevice<CS> {
    type Error = RtSpiError;
}

impl<CS> SpiDevice<u8> for RtSpiDevice<CS> {
    fn transaction(&mut self, operations: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
        for op in operations {
            match op {
                Operation::Read(buf) => self.bus.read(buf)?,
                Operation::Write(buf) => self.bus.write(buf)?,
                Operation::Transfer(read, write) => self.bus.transfer(read, write)?,
                Operation::TransferInPlace(buf) => self.bus.transfer_in_place(buf)?,
                Operation::DelayNs(_) => {
                    // 对于 SPI 事务中的延迟，我们可以忽略或者使用 RT-Thread 的延迟函数
                    // 这里暂时忽略
                }
            }
        }
        Ok(())
    }
}

impl<CS> Drop for RtSpiDevice<CS> {
    fn drop(&mut self) {
        self.bus.close();
    }
}

impl<CS> Drop for RtSpiBus<CS> {
    fn drop(&mut self) {
        self.close();
    }
}

// 外部 C 函数声明
extern "C" {
    fn rt_device_find(name: *const core::ffi::c_char) -> *mut rt_device;
}
