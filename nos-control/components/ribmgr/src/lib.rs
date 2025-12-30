use tokio::sync::mpsc::Receiver;
use nos_common::internal::rib::RibEvent;

// RIB 主循环
pub async fn run(mut rx: Receiver<RibEvent>) {
    println!("📚 RIB (路由表) 组件启动，等待数据...");

    while let Some(event) = rx.recv().await {
        match event {
            RibEvent::Update { protocol, prefix, nexthop, metric, .. } => {
                println!("---------------------------------------");
                println!("📥 RIB 收到路由更新!");
                println!("   来源协议: {:?}", protocol);
                println!("   前缀    : {}", prefix);
                println!("   下一跳  : {}", nexthop);
                println!("   Metric  : {}", metric);
                println!("---------------------------------------");
            }
            RibEvent::Withdraw { prefix, .. } => {
                println!("🗑️ RIB 删除路由: {}", prefix);
            }
        }
    }
}