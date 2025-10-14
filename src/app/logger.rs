use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;
pub fn init()
{
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(
            tracing_subscriber::fmt::layer()
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_target(true)
        )
        .init();
    //unwrap_or_else -> 使用闭包 只有失败了才执行
}