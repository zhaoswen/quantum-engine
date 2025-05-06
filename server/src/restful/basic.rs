use crate::restful::handler;
use crate::restful::handler::test::test;
use axum::routing::{get, post};
use axum::Router;

pub async fn restful_serve() {
    // 构建监听路由
    let app = Router::new()
        // 流程操作
        // 引擎操作
        // 直接调用根目录视为测试
        .route("/engine/test", get(test))
        // 停止服务
        .route("/engine/stop", post(handler::engine::stop))
        // 执行指定蓝图（本地路径或直接蓝图文件）
        // 执行特定处理器
        // 根据蓝图路径运行
        .route("/flow/run/{path}", get(handler::flow::run_bp_path));
    // .route("/flow/run/bypath", post(handler::flow::run_bp_path));

    // 默认绑定到 0.0.0.0:9828
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9828").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
