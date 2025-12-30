use std::time::Duration;
use ipnet::IpNet;
use nos_common::services::RibService; // ✅ 只引用抽象接口

pub mod packet;
pub mod fsm;
pub mod family;
mod peer;

// 参数是 Box<dyn RibService>，表示“任何实现了该接口的对象”
pub async fn run(rib: Box<dyn RibService>) {
    println!("🌍 BGP 组件启动 (依赖注入版)");
    
    // 模拟建立连接耗时
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("🤝 BGP Neighbor Established");

    let mut counter = 0;

    // ✅ 死循环，确保 BGP 不退出
    loop {
        counter += 1;
        
        // 模拟产生不同的路由
        let ip_octet = counter % 255;
        let prefix_str = format!("10.0.{}.0/24", ip_octet);
        let prefix: IpNet = prefix_str.parse().unwrap();

        println!("\n--- [Tick {}] BGP 状态机触发 ---", counter);
        println!("⚡ BGP 计算出路由: {}", prefix);
        
        // 调用接口方法 (BGP 根本不知道对面是 Channel 还是 Actor)
        rib.add_route(prefix, "192.168.1.1".parse().unwrap()).await;

        // 每 5 秒循环一次
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}