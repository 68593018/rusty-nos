use tokio::sync::mpsc; // 虽然这里没直接用 channel，但保留引用也没事

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🚀 RustyNOS 控制面启动 (完全解耦架构)...");

    // 1. 实例化 RIB 服务 (Concrete Implementation)
    // 得到 service (给 BGP 用) 和 rx (给 RIB 自己用)
    let (rib_service, rib_rx) = comp_ribmgr::RibServiceConcrete::new(100);

    // 2. 启动 RIB 消费者线程
    tokio::spawn(async move {
        comp_ribmgr::run(rib_rx).await;
    });

    // 3. 启动 BGP 生产者线程
    // 【关键步骤】：向上转型 (Upcasting)
    // 把具体的 rib_service 包装成抽象的 Box<dyn RibService>
    let rib_abstraction = Box::new(rib_service);

    tokio::spawn(async move {
        comp_bgp::run(rib_abstraction).await;
    });

    // 4. 阻塞主线程，防止退出
    tokio::signal::ctrl_c().await?;
    println!("🛑 进程退出");
    Ok(())
}