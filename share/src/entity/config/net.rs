use serde_derive::Deserialize;

// 网络配置相关
#[derive(Debug, Deserialize, Clone)]
pub struct NetConfig {
    /// 是否启动 HTTP 监听
    #[serde(default = "default_rest_enable_listener")]
    pub rest_enable_listener: bool,

    /// 监听地址
    #[serde(default = "default_rest_listener_address")]
    pub rest_listener_address: String,

    /// 监听端口
    #[serde(default = "default_rest_listener_port")]
    pub rest_listener_port: u16,

    /// 工作线程数
    #[serde(default = "default_rest_listener_worker")]
    pub rest_listener_worker: usize,

    /// HTTP temp 临时目录所在位置
    #[serde(default = "default_rest_listener_temp_path")]
    pub rest_listener_temp_path: String,
}

impl Default for NetConfig {
    fn default() -> Self {
        Self {
            rest_enable_listener: default_rest_enable_listener(),
            rest_listener_address: default_rest_listener_address(),
            rest_listener_port: default_rest_listener_port(),
            rest_listener_worker: default_rest_listener_worker(),
            rest_listener_temp_path: default_rest_listener_temp_path(),
        }
    }
}

// 默认值函数
fn default_rest_enable_listener() -> bool {
    true
}

fn default_rest_listener_address() -> String {
    "127.0.0.1".to_string()
}

fn default_rest_listener_port() -> u16 {
    9898
}

fn default_rest_listener_worker() -> usize {
    5
}

fn default_rest_listener_temp_path() -> String {
    "tmp/web".to_string()
}
