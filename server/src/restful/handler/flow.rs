use crate::entity::restful::SysResult;
use engine_common::dispatch::interface::dispatch_bp_file;
use std::path::Path;

// 运行指定文件路径下的蓝图
pub async fn run_bp_path(axum::extract::Path(path): axum::extract::Path<String>) -> SysResult<&'static str, String> {
    // 判断是否有文件路径
    if path.is_empty() {
        return SysResult::fail("file path is empty".to_string());
    }
    match dispatch_bp_file(Path::new("/Users/eyresimpson/Project/melt-project/melt-simx/quantum-engine/example/flow/init/local.bp")).await {
        Ok(_) => SysResult::ok("ok"),
        Err(_) => SysResult::fail("Run fail".to_string()),
    }
}
