use alloc::vec;
use alloc::vec::Vec;
use embedded_hal::spi::{SpiBus, SpiDevice};
use rustffi::spi::{RtSpiBus, RtSpiDevice, RtSpiError};

/// 虚拟片选类型（用于演示）
pub struct VirtualCS;

/// SPI 设备使用示例
pub struct SpiExample {
    spi: RtSpiDevice<VirtualCS>,
}

impl SpiExample {
    /// 创建新的 SPI 示例实例
    pub fn new() -> Result<Self, RtSpiError> {
        // 使用 RT-Thread 的 SPI 设备
        let spi = RtSpiDevice::<VirtualCS>::new("spi3")?;

        Ok(Self { spi })
    }

    /// 读取 SPI 设备 ID
    pub fn read_device_id(&mut self) -> Result<[u8; 3], RtSpiError> {
        let mut id = [0u8; 3];

        // 发送读取 ID 命令
        let cmd = [0x9F]; // 常见的读取 ID 命令

        self.spi.transaction(&mut [
            embedded_hal::spi::Operation::Write(&cmd),
            embedded_hal::spi::Operation::Read(&mut id),
        ])?;

        Ok(id)
    }

    /// 写入数据到 SPI 设备
    pub fn write_data(&mut self, data: &[u8]) -> Result<(), RtSpiError> {
        self.spi
            .transaction(&mut [embedded_hal::spi::Operation::Write(data)])
    }

    /// 读取数据从 SPI 设备
    pub fn read_data(&mut self, len: usize) -> Result<Vec<u8>, RtSpiError> {
        let mut data = vec![0u8; len];

        self.spi
            .transaction(&mut [embedded_hal::spi::Operation::Read(&mut data)])?;

        Ok(data)
    }

    /// 全双工传输
    pub fn transfer_data(&mut self, data: &[u8]) -> Result<Vec<u8>, RtSpiError> {
        let mut read_data = vec![0u8; data.len()];

        self.spi
            .transaction(&mut [embedded_hal::spi::Operation::Transfer(&mut read_data, data)])?;

        Ok(read_data)
    }
}

/// 使用 embedded_hal 的 SPI 驱动示例
pub fn spi_demo() -> Result<(), RtSpiError> {
    use rustffi::println;

    println!("Starting SPI demo...");

    // 创建 SPI 设备
    let mut spi_example = SpiExample::new()?;
    println!("SPI device created successfully");

    // 读取设备 ID
    match spi_example.read_device_id() {
        Ok(id) => println!("Device ID: {:02X?}", id),
        Err(e) => println!("Failed to read device ID: {:?}", e),
    }

    // 写入测试数据
    let test_data = [0x01, 0x02, 0x03, 0x04];
    match spi_example.write_data(&test_data) {
        Ok(()) => println!("Data written successfully"),
        Err(e) => println!("Failed to write data: {:?}", e),
    }

    // 读取数据
    match spi_example.read_data(4) {
        Ok(data) => println!("Read data: {:02X?}", data),
        Err(e) => println!("Failed to read data: {:?}", e),
    }

    // 全双工传输
    let tx_data = [0xAA, 0xBB, 0xCC, 0xDD];
    match spi_example.transfer_data(&tx_data) {
        Ok(rx_data) => println!("Transfer result: {:02X?}", rx_data),
        Err(e) => println!("Failed to transfer data: {:?}", e),
    }

    println!("SPI demo completed");
    Ok(())
}

/// 直接使用 SPI 总线的示例
pub fn spi_bus_demo() -> Result<(), RtSpiError> {
    use rustffi::println;

    println!("Starting SPI bus demo...");

    // 创建 SPI 总线
    let mut spi_bus = RtSpiBus::<VirtualCS>::new("spi4")?;
    println!("SPI bus created successfully");

    // 写入数据
    let write_data = [0x12, 0x34, 0x56, 0x78];
    match spi_bus.write(&write_data) {
        Ok(()) => println!("Data written to bus"),
        Err(e) => println!("Failed to write to bus: {:?}", e),
    }

    // 读取数据
    let mut read_data = [0u8; 4];
    match spi_bus.read(&mut read_data) {
        Ok(()) => println!("Data read from bus: {:02X?}", read_data),
        Err(e) => println!("Failed to read from bus: {:?}", e),
    }

    // 全双工传输
    let tx_data = [0xAA, 0xBB, 0xCC, 0xDD];
    let mut rx_data = [0u8; 4];
    match spi_bus.transfer(&mut rx_data, &tx_data) {
        Ok(()) => println!("Transfer completed: {:02X?}", rx_data),
        Err(e) => println!("Failed to transfer: {:?}", e),
    }

    println!("SPI bus demo completed");
    Ok(())
}
