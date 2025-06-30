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
