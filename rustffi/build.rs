use std::env;
use std::path::PathBuf;

fn main() {
    // 获取 rt-thread 根目录的路径，如果未设置，则使用默认值 ""
    let rtthread_path = env::var("RTT_ROOT")
        .unwrap_or_else(|_| "/Users/leo/work/rt-thread-rs/rt-thread".to_string());
    // 获取硬件目标 BSP 的路径，如果未设置，则使用默认值为 rt-thread_path + "/bsp/stm32/stm32h750vbt6"
    let hw_target_path = env::var("HW_TARGET")
        // .unwrap_or_else(|_| format!("{}/bsp/stm32/stm32f401-st-nucleo", rtthread_path));
        .unwrap_or_else(|_| format!("{}/../myboard/stm32h750vbt6", rtthread_path));
    let cpu_path =
        env::var("CPU_PATH").unwrap_or_else(|_| format!("{}/libcpu/arm/cortex-m7", rtthread_path));

    // 默认尝试使用 Homebrew 安装的 arm-none-eabi-gcc 路径（请根据实际情况调整）
    let cross_path = env::var("CROSS_INCLUDE").unwrap_or_else(|_| {
        "/Applications/ArmGNUToolchain/14.2.rel1/arm-none-eabi/arm-none-eabi/include".to_string()
    });

    // 如果 header 文件发生变化，重新生成绑定
    println!(
        "cargo:rerun-if-changed={}/include/rtthread.h",
        rtthread_path
    );
    println!(
        "cargo:rerun-if-changed={}/components/drivers/include/drivers/dev_spi.h",
        rtthread_path
    );

    // 构造 bindgen 构建器
    let bindings = bindgen::Builder::default()
        // 指定主头文件
        .header(format!("{}/include/rtthread.h", rtthread_path))
        // 添加 SPI 头文件
        .header(format!(
            "{}/components/drivers/include/drivers/dev_spi.h",
            rtthread_path
        ))
        // 使用 core 而非 std
        .use_core()
        // 合并 extern "C" 块
        .merge_extern_blocks(true)
        // 不在 enum 前面添加前缀
        .prepend_enum_name(false)
        // 关闭布局测试
        .layout_tests(false)
        // 添加额外的 clang 参数（include 路径）
        .clang_arg(format!("-I{}", hw_target_path))
        .clang_arg(format!("-I{}/include", rtthread_path))
        .clang_arg(format!("-I{}/components/legacy", rtthread_path))
        .clang_arg(format!("-I{}/components/drivers/include", rtthread_path))
        .clang_arg(format!(
            "-I{}/components/drivers/include/drivers",
            rtthread_path
        ))
        .clang_arg(format!("-I{}/components/finsh", rtthread_path))
        .clang_arg(format!("-I{}", cpu_path))
        .clang_arg(format!("-I{}", cross_path))
        // 生成绑定
        .generate()
        .expect("Unable to generate bindings");

    // 将生成的绑定写入到 $OUT_DIR/bindings.rs 中
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("ffi.rs"))
        .expect("Couldn't write ffi!");
}
