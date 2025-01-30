use crate::engine::cron::cron_service_init;
use crate::engine::initialize::engine_init;
use crate::engine::thread::init_thread_monitor;
use crate::extension::core::call_init;
use crate::flow::interface::exec_flow;
use crate::logger::interface::{fail, info};
use crate::runtime::config::get_simx_config;
use crate::runtime::extension::get_all_extension_info;
use engine_share::entity::exception::engine::EngineErr;
use std::path::Path;
use std::{env, fs};

/// 引擎核心
/// 其实引擎启动主要是启动了系统监听，引擎本身并不会持续运行，否则会占用一些不必要的资源，当有请求抵达监听器时，
/// 才会调用引擎方法，发起流程或脚本
pub async fn serve() -> Result<(), EngineErr> {
    // 获取simx基础配置
    let simx_config = get_simx_config();

    //  初始化核心线程池
    match init_thread_monitor() {
        Ok(_) => info("Thread monitor init complete."),
        Err(err) => {
            fail("Thread monitor init failed");
            return Err(err);
        }
    }

    // 执行系统初始化事件
    match engine_init().await {
        Ok(_) => info("engine init complete."),
        Err(init_ret) => {
            fail(init_ret.as_str());
            // 退出执行，初始化属于重要操作，错误就退出引擎
            return Err(EngineErr::EngineInitErr(init_ret));
        }
    }

    let mut jobs = vec![];

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
        let job = tokio::spawn(async move {
            call_init(extension).unwrap();
        });
        jobs.push(job);
    }

    for job in jobs {
        // 只要有一个线程没有退出，就阻塞引擎不退出
        job.await.unwrap();
    }

    // 监听定时任务
    cron_service_init();
    // 检查配置中是否需要阻塞进程
    if simx_config.engine.run_strategy != "once" {
        info("Simx engine running, Press Ctrl + C Exit.");
        // 等待用户 ctrl + c 结束进程
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {}
        }
    }
    Ok(())
}

/// 运行流
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

// 初始化方法
pub fn init() {
    // 检查日志文件夹
    let engine_conf = get_simx_config().engine;
    // 检查运行目录下是否有日志目录
    let log_path = Path::new(engine_conf.log_path.as_str()).is_dir();
    if !log_path {
        // 重建日志目录
        fs::create_dir(engine_conf.log_path.as_str()).expect("Engine cannot fix workspace, Please check your environment.");
    }
}

// 这个是为了后续的内存池清理工作的准备
pub fn clean() {
    info("Simx engine run complete.");
}
