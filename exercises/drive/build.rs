use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 获取当前的UNIX时间戳
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    // 将其转换为字符串并设置为环境变量
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
}