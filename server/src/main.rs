use engine_common::engine::kernel::init;

mod restful;

#[tokio::main]
async fn main() {
    // 引擎运行前的准备和初始化动作
    init();
    restful::basic::restful_serve().await;
}
