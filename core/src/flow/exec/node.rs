use crate::handler::interface::root_handler;
use crate::logger::interface::debug;
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;

// Node 调度
// Node 需要对应的Handler执行
// 需要传入标准的handler路径字符串和参数列表，并返回统一传回对象
pub async fn exec_node(node: Node, data: &mut FlowData) -> Result<(), NodeError> {
    let name = node.name.clone();
    debug(format!("[ Node {} exec start ]", name).as_str());

    let ret = root_handler(node, data).await;

    debug(format!("[ Node {} exec end ]", name).as_str());
    ret
}