// =====================================
// 1. 模块声明 (对应您截图里的文件夹)
// =====================================
pub mod packet;
pub mod fsm;
pub mod family;
mod peer; // 假设 peer.rs 存在于 src 根目录

// =====================================
// 2. BGP 业务逻辑
// =====================================
use tokio::sync::mpsc::Sender;
use std::time::Duration;
use ipnet::IpNet;
use nos_common::internal::rib::{RibEvent, RouteProtocol};

// 私有属性结构 (只在 BGP 内部使用)
#[derive(Debug)]
#[allow(dead_code)] // ✅ 新增这一行，忽略未使用字段的警告
struct BgpAttributes {
    origin: u8,
    as_path: Vec<u32>,
    local_pref: u32,
}

pub async fn run(tx: Sender<RibEvent>) {
    println!("🌍 BGP 组件启动 (All-in-One Mode)");

    // 模拟等待 TCP 建立
    tokio::time::sleep(Duration::from_secs(1)).await;

    // 1. 模拟收到报文，解析出内部属性
    let private_attr = BgpAttributes {
        origin: 0,
        as_path: vec![100, 200, 300],
        local_pref: 100,
    };
    
    let prefix: IpNet = "1.1.1.0/24".parse().unwrap();

    println!("⚡ BGP 选路完成: {}", prefix);
    println!("   (内部属性 AS_Path: {:?})", private_attr.as_path);

    // 2. 转换为通用格式发给 RIB
    let event = RibEvent::Update {
        protocol: RouteProtocol::BGP,
        prefix,
        nexthop: "192.168.1.1".parse().unwrap(),
        metric: 0,
        admin_distance: 20,
    };

    println!("📤 发送路由给 RIB...");
    if let Err(e) = tx.send(event).await {
        println!("发送失败: {}", e);
    }
}