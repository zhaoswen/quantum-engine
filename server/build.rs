#[cfg(windows)]
extern crate winres;
#[cfg(windows)]
fn main() {
    // 仅windows下有效
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        // 设置图标
        res.set_icon("resource/SimxIcon.ico")
            // 设置内部名称
            .set("InternalName", "Quantum")
            // 设置描述
            .set("FileDescription", "Quantum Engine Server")
            // 设置产品名称
            .set("ProductName", "Quantum Server")
            // 设置版权信息
            .set("LegalCopyright", "Copyright © 2025 ZhaoShenWen")
            // 设置公司名称
            .set("CompanyName", "NJ Lab")
            // 设置产品版本
            .set("ProductVersion", "1.0.0")
            // 设置文件版本
            .set("FileVersion", "1.0.0");
        // 编译资源
        res.compile().expect("Failed to compile resources");
    }
}
#[cfg(unix)]
fn main() {}