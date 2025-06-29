/*
 * Copyright (c) 2006-2023, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       Notes
 * 2019-10-25     zylx         first version
 */

#include "rtthread.h"
#include <rtdevice.h>
#include <drv_gpio.h>
#include <rust.h>
#include <drivers/dev_spi.h>

// 函数声明
rt_err_t rt_hw_spi_device_attach(const char *bus_name, const char *device_name, rt_base_t cs_pin);

/* defined the LED1 pin: PC13 */
// #define LED1_PIN GET_PIN(C, 13)

int main(void)
{
    int count = 1;
    /* set LED0 pin mode to output */
    // rt_pin_mode(LED1_PIN, PIN_MODE_OUTPUT);
    // rust_system_clock_init();

    create_mq();

    // 检查 SPI 总线是否存在
    rt_kprintf("Checking SPI bus initialization...\n");
    struct rt_spi_bus *spi_bus = (struct rt_spi_bus *)rt_device_find("spi3");
    if (spi_bus) {
        rt_kprintf("SPI3 bus found and initialized\n");
    } else {
        rt_kprintf("SPI3 bus not found - this may cause the mutex error\n");
    }

    // 先创建 SPI 设备
    struct rt_spi_device *spi_device = rt_malloc(sizeof(struct rt_spi_device));
    rt_err_t result                  = rt_spi_bus_attach_device_cspin(spi_device, "spi30", "spi3", rt_pin_get("PB.7"), RT_NULL);
    if (result != RT_EOK) {
        rt_kprintf("Failed to attach SPI3 device, error: %d\n", result);
    } else {
        rt_kprintf("SPI3 device attached successfully\n");
    }

    // 查找 SPI 设备（不是总线）
    struct rt_spi_device *spi3 = (struct rt_spi_device *)rt_device_find("spi30");
    if (spi3) {
        rt_kprintf("spi30 device found\n");

        // 配置 SPI 设备
        struct rt_spi_configuration cfg;
        cfg.data_width = 8;
        cfg.mode       = RT_SPI_MODE_0 | RT_SPI_MSB; // SPI 模式 0，MSB 优先
        cfg.max_hz     = 1000000;                    // 1MHz

        result = rt_spi_configure(spi3, &cfg);
        if (result != RT_EOK) {
            rt_kprintf("Failed to configure SPI3, error: %d\n", result);
        } else {
            rt_kprintf("SPI3 configured successfully\n");
        }

        // 测试 SPI 传输
        rt_uint8_t send_buf[2] = {0x80 | 0x75};
        rt_uint8_t recv_buf[2] = {0, 0};
        rt_spi_send(spi3, send_buf, 1);
        rt_spi_recv(spi3, recv_buf, 1);
        rt_kprintf("recv_buf:  %02x %02x\n", recv_buf[0], recv_buf[1]);
        // result                 = rt_spi_send_then_recv(spi3, send_buf, 1, recv_buf, 2);
        // if (result == RT_EOK) {
        //     rt_kprintf("SPI transfer successful, recv_buf: %02x %02x\n", recv_buf[0], recv_buf[1]);
        // } else {
        //     rt_kprintf("SPI transfer failed, error: %d\n", result);
        // }
    } else {
        rt_kprintf("spi30 device not found\n");
    }

    rust_test_spi();
    while (count++) {

        // rust_main();
        // rt_kprintf("rust_str:  %s\n", rust_str());
        // rt_pin_write(LED1_PIN, PIN_HIGH);
        rt_thread_mdelay(500);
        // rt_pin_write(LED1_PIN, PIN_LOW);
    }
    return RT_EOK;
}
