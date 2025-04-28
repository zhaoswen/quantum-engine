use crate::controller::interface::check_require;
use crate::dispatch::common::redress_stream_dispatch;
use crate::dispatch::general::dispatch_general;
use crate::dispatch::logic::dispatch_loop::dispatch_loop;
// use crate::dispatch::logic::dispatch_loop;
use crate::dispatch::service::dispatch_service;
use crate::exception::flow::flow_dispatch_err_handler;
use crate::flow::resolver::interface::flow_resolver;
use crate::logger::interface::{debug, fail};
use crate::runtime::flow::{get_flow_runtime, set_flow_runtime};
use crate::runtime::history::{history_persistent, log_history};
use crate::tools::common::{get_current_time, get_timestamp, get_uuid};
use engine_share::entity::common::HistoryLog;
use engine_share::entity::exception::common::NodeStatus;
use engine_share::entity::exception::dispatch::DispatchErr;
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::flow::blueprint::Blueprint;
use engine_share::entity::flow::flow::{Flow, FlowData, FlowRuntimeModel, SystemFlowData};
use engine_share::entity::flow::node::{Node, NodeTag};
use std::path::Path;

// 通过蓝图文件进行调度
pub async fn dispatch_bp_file(path: &Path) -> Result<(), DispatchErr> {
    // 流对象
    let flow: Flow;

    // 在缓存中搜索路径是否存在，如果存在就不再走文件系统
    // TODO：这个判断并不稳定，有可能flow会发生改变，因此需要缓存文件修改日期进行比较，这个是后续的功能
    match get_flow_runtime(path.to_str().unwrap()) {
        Some(f) => {
            // 直接使用缓存中的数据
            flow = f;
        }
        None => {
            // 尝试解析蓝图
            if path.exists() && path.is_file() {
                // 加载蓝图并解析为Flow对象
                flow = flow_resolver(path);
                // 将当前流加入到缓存
                set_flow_runtime(path.to_str().unwrap(), flow.clone());
            } else {
                fail("Cannot find or open flow file.");
                return Err(DispatchErr::FlowNotFound(
                    path.to_str().unwrap().to_string(),
                ));
            }
        }
    }
    dispatch_flow(flow).await
}

// 通过Json进行调度(Json化的Flow对象）
pub async fn dispatch_json_str(json: String) -> Result<(), DispatchErr> {
    // 流对象
    let flow: Flow = serde_json::from_str(&json).expect("Cannot parse json to flow object.");
    dispatch_flow(flow).await
}

// 流调度执行器
// 此方法会根据蓝图的path，生成Flow运行时并调度执行
pub async fn dispatch_flow(mut flow: Flow) -> Result<(), DispatchErr> {
    // 检查蓝图的环境要求
    match check_require(flow.clone().requirements) {
        Ok(_) => {}
        Err(e) => {
            fail(e.as_str());
            return Err(DispatchErr::RequireError(e));
        }
    }

    let uuid = get_uuid();
    // 记录执行开始时间
    let start_time = get_timestamp();
    // 创建流运行时
    flow.runtime = Some(FlowRuntimeModel {
        current_node: None,
        data: FlowData {
            // 系统数据表
            basics: SystemFlowData {
                flow_id: uuid.clone(),
                flow_name: "".to_string(),
                route: Default::default(),
                logs: vec![],
                start_time: start_time.to_string(),
                end_time: "".to_string(),
            },
            // 用户变量表
            params: Default::default(),
            // 节点Json数据
            json: Default::default(),
            // 节点二进制数据
            binary: Default::default(),
        },
    });

    debug(format!("Flow {} :[{}] will be exec.", flow.name, uuid).as_str());

    let runtime = flow.clone().runtime.unwrap();
    // 取入口节点群并尝试执行
    let endpoints = flow.clone().blueprint.endpoints;
    // 执行所有开始节点
    for endpoint in endpoints {
        // 根据入口节点id获取节点对象
        let node = flow
            .blueprint
            .routes
            .get(&endpoint)
            .expect("Cannot find endpoint in router.");
        let mut node = node.clone();
        node.id = Some(endpoint.as_str().to_string());
        // 判断开始节点是否挂载了服务，如果挂载了服务，就交给服务调度
        match node.attr.get("service") {
            Some(service) => {
                let service = node
                    .attr
                    .get("service")
                    .unwrap()
                    .as_str()
                    .expect("Cannot get service name.");
                // 必须不等于none，才会继续进行服务调度，否则回退到普通调度
                if service != "none" {
                    // TODO 创建新线程
                    dispatch_service(
                        service,
                        flow.blueprint.clone(),
                        node.clone(),
                        &mut runtime.data.clone(),
                    )
                        .await
                        .expect("dispatch start service error");
                    continue;
                }
            }
            None => {
                // 找不到service属性，可能是较老的蓝图，直接略过
            }
        }
        // 执行节点
        match dispatch_nodes(flow.blueprint.clone(), node, &mut runtime.data.clone()).await {
            Ok(_) => {}
            Err(e) => return flow_dispatch_err_handler(e),
        }
    }
    let sec = (get_timestamp().parse::<f64>().unwrap()
        - runtime.data.basics.start_time.parse::<f64>().unwrap())
        / 1_000_000_000.0;
    debug(
        format!(
            "Flow {} :[{}] completes in {} second",
            flow.name,
            uuid.clone(),
            sec
        )
            .as_str(),
    );
    // 将历史日志进行持久化
    history_persistent(uuid);
    Ok(())
}

// 调度节点（蓝图）
// 蓝图、首节点（从这个节点开始执行）、数据
pub async fn dispatch_nodes(
    blueprint: Blueprint,
    current_node: Node,
    data: &mut FlowData,
) -> Result<(), DispatchErr> {
    let node_uuid = get_uuid();
    let node_handler = current_node.clone().handler;
    let blueprint_id = current_node.clone().id.unwrap();
    let flow_id = data.clone().basics.flow_id.clone();
    let log_data = match current_node.clone().common {
        None => false,
        Some(comm) => comm.log_data.unwrap_or_else(|| false),
    };
    let data_logger: Option<FlowData>;
    if log_data {
        data_logger = Some(data.clone())
    } else {
        data_logger = None;
    }
    // 对通用配置进行操作
    node_common_handle(
        flow_id.clone(),
        node_uuid.clone(),
        node_handler.clone(),
        blueprint_id.clone(),
        NodeStatus::Start,
        "Node will be exec.".to_string(),
        data_logger.clone(),
    );
    // 当前节点的标签列表
    let tags = match current_node.tags {
        None => Vec::new(),
        Some(ref tags) => tags.clone(),
    };
    // 当前节点是否为开始节点（开始节点不处理）
    let is_start = tags.contains(&NodeTag::Start);
    // 当前节点是否为loop
    let is_loop = tags.contains(&NodeTag::Loop);
    // 是否为Jump节点
    let is_jump = tags.contains(&NodeTag::Jump);

    // Loop节点、Jump节点、Opt节点（操作节点）
    // Loop、Jump、Opt不参与exec_node调度，直接在此处实现
    if !current_node.handler.is_empty() {
        // 作为普通节点进行调度
        let ret = dispatch_general(blueprint, current_node, data).await;
        node_common_handle(
            flow_id.clone(),
            node_uuid.clone(),
            node_handler.clone(),
            blueprint_id.clone(),
            NodeStatus::End,
            "Node exec successfully.".to_string(),
            data_logger,
        );
        ret
    } else if is_start {
        Ok(())
    } else if is_loop {
        match Box::pin(dispatch_loop(blueprint.clone(), current_node.clone(), data)).await {
            Ok(_) => {
                node_common_handle(
                    flow_id.clone(),
                    node_uuid.clone(),
                    node_handler.clone(),
                    blueprint_id.clone(),
                    NodeStatus::End,
                    "Node exec successfully.".to_string(),
                    data_logger,
                );
                Ok(())
            }
            Err(_) => {
                fail(
                    "The implicated compensation mechanism is triggered"
                        .to_string()
                        .as_str(),
                );
                node_common_handle(
                    flow_id,
                    node_uuid.clone(),
                    node_handler.clone(),
                    blueprint_id.clone(),
                    NodeStatus::Fail,
                    "The implicated compensation mechanism is triggered.".to_string(),
                    data_logger,
                );
                // 执行当前节点的Redress_stream，如果节点报错，会依次执行之前所有节点的Redress_stream
                return redress_stream_dispatch(
                    NodeError::Redress(
                        "The implicated compensation mechanism is triggered".to_string(),
                    ),
                    &current_node,
                    &blueprint,
                    data,
                )
                    .await;
            }
        }
    } else if is_jump {
        // 作为Jump节点进行处理
        Ok(())
    } else {
        Ok(())
    }
}

fn node_common_handle(
    flow_id: String,
    node_id: String,
    handler: String,
    bp_id: String,
    status: NodeStatus,
    message: String,
    data: Option<FlowData>,
) {
    let history = &mut HistoryLog {
        handler,
        bp_id,
        node_id,
        status,
        snapshot: data.clone(),
        message,
        log_dt: get_current_time("%Y-%m-%d %H:%M:%S%.9f"),
    };

    log_history(flow_id, history.to_owned());
}
