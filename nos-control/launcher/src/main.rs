use tracing::info;
use tokio::sync::mpsc;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 初始化日志
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    info!("RustyNOS Control Plane Launching...");

    // 2. 创建 BGP -> RIB 的高速内存通道 (Internal Channel)
    // 缓冲区大小 100，防止 RIB 处理不过来时 BGP 无限发
    let (rib_tx, rib_rx) = mpsc::channel(100);

    // 3. 启动 RIB 组件 (消费者)
    // 把它放到后台运行 (Green Thread)
    let rib_handle = tokio::spawn(async move {
        comp_ribmgr::run(rib_rx).await;
    });

    // 4. 启动 BGP 组件 (生产者)
    // 把它放到后台运行
    let bgp_handle = tokio::spawn(async move {
        comp_bgp::run(rib_tx).await;
    });

    // 5. 等待任务结束 (实际上它们是无限循环，除非 panic)
    // 这里我们用 select 等待任意一个退出
    tokio::select! {
        _ = rib_handle => info!("RIB actor exited"),
        _ = bgp_handle => info!("BGP actor exited"),
        _ = tokio::signal::ctrl_c() => info!("Ctrl-C received"),
    }

    info!("Shutting down...");
    Ok(())
}