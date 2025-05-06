use engine_common::engine::kernel::{clean, init, run};
use std::env;

#[tokio::main]
async fn main() {    // 读取cron配置文件
    // 引擎运行前的准备和初始化动作
    init().await.expect("Engine init failed.");
    // 分析用户输入参数，如果没有输入参数，就代表默认的启动方式
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    // 如果没有输入参数
    if args.len() > 1 {
        // 解析输入参数
        match args[1].as_str() {
            "run" => run().await,
            _ => println!("No params"),
        }
        return;
    }
    // 程序运行结束后的清理动作
    // 注意，用户手动结束进程不会触发此方法
    clean();
}