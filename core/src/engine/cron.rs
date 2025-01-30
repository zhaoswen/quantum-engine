// CRON 服务核心
use crate::runtime::config::get_simx_config;
use crate::script::interface::exec_script;
use job_scheduler::{Job, JobScheduler};
use std::time::Duration;

// 初始化 CRON 服务
pub fn cron_service_init() {
    // 读取系统中的cron配置
    let cron_conf = get_simx_config().cron;
    // 如果未开启CRON，直接退出执行
    if !cron_conf.enable {
        return;
    }
    // 拆取cron字符串和要执行的命令，格式: 1/10 * * * * * 执行器 "参数路径"
    let mut sched = JobScheduler::new();
    for cron in cron_conf.cron_list {
        let mut cron_vec = cron.split(" ").collect::<Vec<&str>>();
        let exec_path = cron_vec
            .pop()
            .expect("cannot get cron exec path.")
            .to_string(); // Convert to String
        let exec_type = cron_vec
            .pop()
            .expect("cannot get cron exec type.")
            .to_string(); // Convert to String
        let cron_str = cron_vec.join(" ");
        sched.add(Job::new(cron_str.parse().unwrap(), move || {
            println!("exec_path --> {}， {}", exec_path, exec_type);
            match exec_type.as_str() {
                "script" => {
                    exec_script(exec_path.as_ref());
                }
                "blueprint" => {
                    println!("exec_path --> {}", exec_path);
                }
                _ => {}
            }
        }));
    }

    // TODO: 创建新线程
    loop {
        // 检查
        sched.tick();
        // 休眠500毫秒
        std::thread::sleep(Duration::from_millis(500));
    }
}
