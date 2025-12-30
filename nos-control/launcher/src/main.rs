use tokio::sync::mpsc;
use tokio::runtime::Builder; // 引入构建器

fn main() -> anyhow::Result<()> {
    // 1. 手动构建 Tokio Runtime
    let runtime = Builder::new_multi_thread()
        .worker_threads(4)               // 指定启动 4 个物理工作线程 (也可不写，默认自动检测)
        .thread_name("nos-worker")       // 【关键】设置线程名字前缀
        .enable_all()                    // 启用 IO 和 时间驱动
        .build()
        .unwrap();

    // 2. 在 Runtime 中运行我们的逻辑
    runtime.block_on(async {
        println!("🚀 RustyNOS 控制面启动 (PID: {})", std::process::id());

        // --- 原有的业务逻辑 ---
        let (tx, rx) = mpsc::channel(100);

        // 启动 RIB
        tokio::spawn(async move {
            comp_ribmgr::run(rx).await;
        });

        // 启动 BGP
        tokio::spawn(async move {
            comp_bgp::run(tx).await;
        });

        // 挂起主线程
        tokio::signal::ctrl_c().await
    })?;

    println!("🛑 进程退出");
    Ok(())
}