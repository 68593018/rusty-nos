use tokio::sync::mpsc;
use async_trait::async_trait;
use ipnet::IpNet;
use std::net::IpAddr;
use nos_common::internal::rib::{RibEvent, RouteProtocol};
use nos_common::services::RibService;

// 1. 定义具体的服务实现结构体
// 它持有发送端，可以被克隆
#[derive(Clone)]
pub struct RibServiceConcrete {
    tx: mpsc::Sender<RibEvent>,
}

// 2. 实现 nos-common 定义的接口
#[async_trait]
impl RibService for RibServiceConcrete {
    async fn add_route(&self, prefix: IpNet, nexthop: IpAddr) {
        let event = RibEvent::Update {
            protocol: RouteProtocol::BGP,
            prefix,
            nexthop,
            metric: 0,
            admin_distance: 20,
        };
        // 忽略错误处理，或者打印日志
        if let Err(e) = self.tx.send(event).await {
            println!("❌ RIB Service 发送失败: {}", e);
        }
    }
}

// 3. 构造函数
impl RibServiceConcrete {
    pub fn new(capacity: usize) -> (Self, mpsc::Receiver<RibEvent>) {
        let (tx, rx) = mpsc::channel(capacity);
        (Self { tx }, rx)
    }
}

// 4. RIB 主循环 (消费者)
pub async fn run(mut rx: mpsc::Receiver<RibEvent>) {
    println!("📚 RIBMgr 组件启动 (等待接口调用)...");

    // 这个循环不会退出，除非所有 Sender 都被销毁
    while let Some(event) = rx.recv().await {
        match event {
            RibEvent::Update { prefix, nexthop, .. } => {
                println!("---------------------------------------");
                println!("📥 RIB 收到路由更新");
                println!("   Prefix: {}", prefix);
                println!("   NextHop: {}", nexthop);
                println!("---------------------------------------");
            }
            _ => {}
        }
    }
    println!("⚠️ RIBMgr 退出 (通道关闭)");
}