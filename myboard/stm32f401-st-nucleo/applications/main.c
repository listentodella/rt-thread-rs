/*
 * Copyright (c) 2006-2021, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       Notes
 * 2018-11-06     SummerGift   first version
 * 2023-12-03     Meco Man     support nano version
 */

#include <board.h>
#include <rtthread.h>
#include <drv_gpio.h>
#include <drivers/dev_spi.h>
#ifndef RT_USING_NANO
#include <rtdevice.h>
#endif /* RT_USING_NANO */
#include <rust.h>

/* defined the LD2 (user LED) pin: PA5 */
#define LED2_PIN GET_PIN(A, 5)

int main(void)
{
    /* set LED0 pin mode to output */
    // rt_pin_mode(LED2_PIN, PIN_MODE_OUTPUT);

    create_mq();

    rust_test_spi();

    while (1) {
        // rust_main();
        // rt_pin_write(LED2_PIN, PIN_HIGH);
        rt_thread_mdelay(500);
        // rt_pin_write(LED2_PIN, PIN_LOW);
        rt_thread_mdelay(500);
    }
}
