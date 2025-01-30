use serde_derive::Deserialize;

// 环境配置
#[derive(Debug, Deserialize, Clone)]
pub struct CronConfig {
    /// 是否启用 CRON 监听
    #[serde(default = "default_enable")]
    pub enable: bool,

    /// 是否启用 Shell 脚本
    #[serde(default = "default_multi_thread")]
    pub multi_thread: bool,

    /// 是否启用 Batch 脚本
    #[serde(default = "default_cron_list")]
    pub cron_list: Vec<String>,
}

impl Default for CronConfig {
    fn default() -> Self {
        Self {
            enable: default_enable(),
            multi_thread: default_multi_thread(),
            cron_list: default_cron_list(),

        }
    }
}

// 默认值函数
fn default_enable() -> bool {
    false
}


fn default_multi_thread() -> bool {
    true
}

fn default_cron_list() -> Vec<String> {
    vec![]
}
