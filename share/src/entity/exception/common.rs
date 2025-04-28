use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum NodeStatus {
    // 节点错误
    Fail,
    // 节点警告
    Warn,
    // 节点信息
    Info,
    // 节点调试
    Debug,
    // 节点开始
    Start,
    // 节点结束
    End,
    // 无状态（默认）
    #[default]
    None,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum FlowStatus {
    // 无状态（默认）
    #[default]
    None,
    // 初始化中（流正在初始化）
    Init,
    // 开始（流程初始化完成，即将开始运行）
    Start,
    // 运行中（流正在运行中）
    Running,
    // 暂停（流的执行被暂停，状态保持在内存中，可恢复执行，但服务停止等操作会丢失操作）
    Pause,
    // 休眠（流正在休眠，状态保存在文件中，可恢复执行，即使系统重启也不会影响数据安全，但相比 Pause 耗时更久）
    Sleep,
    // 停止中（用户或调度器尝试停止流程，结果为Abort）
    Stoping,
    // 错误（执行过程中发生严重错误，流程执行结束）
    Fail,
    // 中止（被用户或调度器中止执行）
    Abort,
    // 等待（需要等待资源或请求等待）
    Waiting,
    // 异常（运行中有异常，但被认为是可接受错误，流程继续执行，但会始终保有此标签以进行警告）
    Exception,
    // 未知状态
    Unknown,
    // 结束
    End,
}