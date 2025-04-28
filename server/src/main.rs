use engine_common::engine::kernel::init;
use engine_common::logger::interface::info;

mod restful;
mod entity;

#[tokio::main]
async fn main() {
    // 引擎运行前的准备和初始化动作
    init().await.expect("Engine init failed.");
    info("Engine starts restful service listening");
    restful::basic::restful_serve().await;
}
