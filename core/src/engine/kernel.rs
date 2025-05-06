use crate::engine::cron::cron_service_init;
use crate::engine::initialize::engine_init;
use crate::engine::thread::init_thread_monitor;
use crate::extension::core::call_init;
use crate::flow::interface::exec_flow;
use crate::logger::interface::{fail, info};
use crate::runtime::config::get_simx_config;
use crate::runtime::extension::get_all_extension_info;
use std::path::Path;
use std::{env, fs};
use tklog::{Format, LOG};

/// # 运行流
///
/// 此方法不会开启额外的线程，只是通过流引擎执行目标的流
pub async fn run() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    let file_path: &str;
    // 判断文件路径是否为空
    if args.len() <= 2 {
        file_path = args[1].as_str();
    } else {
        file_path = args[2].as_str();
    }
    // 获取文件路径
    let path = Path::new(file_path);

    // 判断文件是否存在
    if !path.exists() {
        fail("The file is not exist.");
        return;
    }

    if !(path.extension().unwrap().to_str().unwrap() != ".bp") {
        fail("The flow file must be selected.");
        return;
    }

    // 调用流引擎执行该文件
    let ret = exec_flow(path).await;
    if ret.is_err() {
        fail(format!("Flow run done with err: {:?}", ret.err().unwrap()).as_str());
    }
}

/// # 基础初始化方法
///
/// ⚠️ 注意 run 方式调用不经过此方法
pub async fn init() -> Result<(), String> {
    // 检查日志文件夹
    let engine_conf = get_simx_config().engine;
    // 检查运行目录下是否有日志目录
    let log_path = Path::new(engine_conf.log_path.as_str());
    if !log_path.is_dir() {
        // 重建日志目录
        fs::create_dir(engine_conf.log_path.as_str())
            .expect("Engine cannot fix workspace, Please check your environment.");
    }

    //  初始化核心线程池
    match init_thread_monitor() {
        Ok(_) => info("Thread monitor init complete."),
        Err(_) => {
            fail("Thread monitor init failed");
            return Err("Thread monitor init failed.".to_string());
        }
    }

    // 初始化日志格式
    LOG.set_console(true)
        // .set_level(LEVEL::Info)
        .set_format(Format::LevelFlag | Format::Time | Format::ShortFileName)
        .set_cutmode_by_size(
            log_path.join("engine.log").to_str().unwrap(),
            1 << 20,
            10,
            true,
        )
        .set_formatter("{level} {time} :{message}\n");

    // 执行系统初始化事件
    engine_init().await?;

    // 新创建一个线程执行cron
    tokio::spawn(async move {
        // 监听定时任务
        cron_service_init();
    });

    // 获取插件列表
    let extensions = get_all_extension_info();
    // 遍历插件列表，调用init方法
    for extension in extensions {
        if extension.init.is_empty() {
            // 如果找不到初始化方法，则跳过插件的初始化（并不强制所有插件必须有初始化方法）
            continue;
        }
        // 调用插件的init方法
        // 注意，新线程中执行init，init的执行结果的顺序不能保证
        tokio::spawn(async move {
            call_init(extension).unwrap();
        });
    }

    Ok(())
}

// 这个是为了后续的内存池清理工作的准备
pub fn clean() {
    info("Simx engine run complete.");
}
