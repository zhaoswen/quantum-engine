use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;
use std::collections::HashMap;

pub async fn stop() {
    // 结束进程
    std::process::exit(0);
}

pub async fn run_exec() {
    let default_node = Node {
        id: None,
        name: "default exec".to_string(),
        tags: None,
        handler: "simx.core.exec".to_string(),
        attr: HashMap::new(),
        common: None,
        downstream: vec![],
        redress_stream: None,
    };

    let default_data = &mut FlowData::default();

    match engine_common::handler::interface::root_handler(default_node, default_data).await {
        Ok(_) => {}
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
