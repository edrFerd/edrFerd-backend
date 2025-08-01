use flexi_logger::{Logger, colored_detailed_format};
use log::info;

/// 初始化日志系统。
///
/// 配置 flexi_logger 以使用带颜色的详细格式，
/// 并设置默认日志级别为 trace。
///
/// 注释中包含了可选的文件日志和日志轮转配置，
/// 目前暂未启用这些功能。
pub fn init_logger() {
    Logger::try_with_str("trace") // 设置默认日志级别为 info
        .unwrap()
        .format(colored_detailed_format) // 使用带颜色的详细格式
        .start()
        .expect("日志初始化失败");
    info!("日志已初始化")
    // 还不需要的rotate
    // .log_to_file(FileSpec::default().directory("logs")) // 将日志写入 "logs" 目录下的文件
    // .duplicate_to_stderr(Duplicate::All) // 同时将所有日志输出到控制台
    // .rotate( // 配置日志轮转
    //     Criterion::Size(10 * 1024 * 1024), // 当文件达到 10MB 时轮转
    //     Naming::Timestamps, // 使用时间戳命名旧的日志文件
    //     flexi_logger::Cleanup::KeepLogFiles(7), // 保留最近的7个日志文件
    // )
}
