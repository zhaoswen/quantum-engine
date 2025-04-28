use engine_common::engine::kernel::{clean, init};

// 运行流程
#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn run() -> Result<(), String> {
    let future = async {
        init().await.unwrap();
        engine_common::engine::kernel::run().await;
        clean();
    };
    tokio::runtime::Runtime::new().unwrap().block_on(future);
    Ok(())
}

// 运行监听（服务）
#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn serve() -> Result<(), String> {
    let future = async {
        init().await.unwrap();
        // engine_common::engine::kernel::serve().await.unwrap();
        clean();
    };
    tokio::runtime::Runtime::new().unwrap().block_on(future);
    Ok(())
}