use serde_derive::Deserialize;

// 环境配置
#[derive(Debug, Deserialize, Clone)]
pub struct EnvConfig {
    /// 是否启用 Python 脚本
    #[serde(default = "default_enable_python_script")]
    pub enable_python_script: bool,

    /// 是否启用 Shell 脚本
    #[serde(default = "default_enable_shell_script")]
    pub enable_shell_script: bool,

    /// 是否启用 Batch 脚本
    #[serde(default = "default_enable_batch_script")]
    pub enable_batch_script: bool,

    /// 是否启用 PowerShell 脚本
    #[serde(default = "default_enable_powershell_script")]
    pub enable_powershell_script: bool,

    /// 是否在 macOS 上启用 Shell 脚本
    #[serde(default = "default_enable_shell_script_in_mac")]
    pub enable_shell_script_in_mac: bool,

    /// 是否在 Linux 上启用 Shell 脚本
    #[serde(default = "default_enable_shell_script_in_linux")]
    pub enable_shell_script_in_linux: bool,

    /// Python 解释器路径
    #[serde(default = "default_python_path")]
    pub python_path: String,
}

impl Default for EnvConfig {
    fn default() -> Self {
        Self {
            enable_python_script: default_enable_python_script(),
            enable_shell_script: default_enable_shell_script(),
            enable_batch_script: default_enable_batch_script(),
            enable_powershell_script: default_enable_powershell_script(),
            enable_shell_script_in_mac: default_enable_shell_script_in_mac(),
            enable_shell_script_in_linux: default_enable_shell_script_in_linux(),
            python_path: default_python_path(),
        }
    }
}

// 默认值函数
fn default_enable_python_script() -> bool {
    true
}


fn default_enable_shell_script() -> bool {
    true
}

fn default_enable_batch_script() -> bool {
    true
}

fn default_enable_powershell_script() -> bool {
    true
}

fn default_enable_shell_script_in_mac() -> bool {
    true
}

fn default_enable_shell_script_in_linux() -> bool {
    true
}

fn default_python_path() -> String {
    "python3".to_string()
}
