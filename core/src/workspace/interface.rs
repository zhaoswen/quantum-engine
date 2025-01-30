use crate::logger::interface::info;
use crate::runtime::config::get_simx_config;
use std::fs;
use std::path::PathBuf;

// 加载workspace到内存
// 这个方法一般与设计器有关
pub async fn init_workspace() {
    // 系统引擎配置
    let engine_conf = get_simx_config().engine;
    let workspace_path = engine_conf.workspace_path;
    // 判断workspace路径是否存在，如果不存在则退出（可能用户并没有工作空间）
    if fs::metadata(workspace_path.clone()).is_err() {
        info("Workspace path not found, load skip.");
        return;
    }
    // 读取工作空间下所有的项目
    let projects = fs::read_dir(workspace_path).unwrap();
    // 遍历文件
    for project in projects {
        // 获取文件路径
        let project_path = project.unwrap().path();
        // 仅对文件夹进行加载，且不加载任何以.开头的文件夹，这些文件夹是工作空间的配置文件
        if !project_path.starts_with(".") && project_path.is_dir() {
            // 在新的线程中初始化工作空间
            tokio::spawn(async move { load_project(project_path).await });
        }
    }
}

pub async fn load_project(path: PathBuf) {
    info(format!("Found an project: {:?}, ready to load", path.file_name()).as_str());
    // 加载项目配置（优先级高于工作空间默认的simx配置）
    let pro_config = path.join("config");
    if pro_config.exists() {
        info("Load project config.")
    }
    // 加载扩展插件到内存
    let pro_extensions = path.join("reference");
    if pro_extensions.exists() {
        info("Load project extensions.")
    }
    // 加载蓝图到内存
    let pro_blueprint = path.join("blueprint");
    if pro_blueprint.exists() {
        info("Load project blueprint.")
    }
    // 加载触发器
    let pro_trigger = path.join("trigger");
    if pro_trigger.exists() {
        info("Load project trigger.")
    }
    // 加载项目资源
    let pro_resource = path.join("resource");
    if pro_resource.exists() {
        info("Load project resource.")
    }
}
