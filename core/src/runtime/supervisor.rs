use engine_share::entity::common::SupervisorLog;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    // 流监督
    static ref FLOW_SUPERVISOR: Mutex<HashMap<String, Vec<SupervisorLog>>> = Mutex::new(HashMap::new());

}

pub fn flow_log(flow_id: String, value: SupervisorLog) {
    let mut data = FLOW_SUPERVISOR.lock().unwrap();
    let mut arr: Vec<SupervisorLog> = match data.get(flow_id.as_str()) {
        Some(arr) => arr.clone(),
        None => Vec::new(),
    };
    arr.push(value);
    data.insert(flow_id, arr);
}