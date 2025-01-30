use engine_share::entity::exception::dispatch::DispatchErr;
use engine_share::entity::flow::blueprint::Blueprint;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;

// 服务调度，此方法仅适用于开始节点形成的服务调度
pub async fn dispatch_service(
    service: &str,
    blueprint: Blueprint,
    current_node: Node,
    data: &mut FlowData,
) -> Result<(), DispatchErr> {
    // 调度服务（可以调度流程、脚本的服务，比如cron、http服务）
    // 功能服务（持续提供某种功能的服务，比如mysql、redis服务）
    // 此处仅支持调度类型的服务，比如cron、http服务

    match service {
        // cron是内置的服务，可以在此处直接调用
        "cron" => {
            // // 获取cron字符串
            // let cron = current_node
            //     .attr
            //     .get("cron")
            //     .expect("must be has cron str")
            //     .as_str()
            //     .expect("cannot covert cron str");
            // // 拆取cron字符串和要执行的命令，格式: 1/10 * * * * * 执行器 "参数路径"
            // let mut sched = JobScheduler::new();
            // sched.add(Job::new(cron.parse().unwrap(), move || {
            //     println!("exec_path --> ", );
            //     // 执行节点
            //     // dispatch_nodes(blueprint, current_node, data);
            // }));
            // // 这会让流被阻塞（多线程情况下）
            // loop {
            //     // 检查
            //     sched.tick();
            //     // 休眠500毫秒
            //     tokio::time::sleep(Duration::from_millis(500)).await;
            // }
        }
        _ => {
            // TODO: 联系服务调度器进行调用
        }
    }

    Ok(())
}
