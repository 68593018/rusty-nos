use tokio::sync::mpsc::Sender;
use tokio::time::{self, Duration}; // 引入时间模块
use ipnet::IpNet;
use nos_common::internal::rib::{RibEvent, RouteProtocol};

// =====================================
// 模块声明
// =====================================
pub mod packet;
pub mod fsm;
pub mod family;
mod peer;

// 私有属性结构
#[derive(Debug)]
#[allow(dead_code)]
struct BgpAttributes {
    origin: u8,
    as_path: Vec<u32>,
    local_pref: u32,
}

pub async fn run(tx: Sender<RibEvent>) {
    println!("🌍 BGP 组件启动 (Loop Mode)");

    // 模拟 BGP 建立邻居耗时
    time::sleep(Duration::from_secs(2)).await;
    println!("🤝 BGP Session Established with 192.168.1.1");

    // 定义一个定时器，每 5 秒触发一次（模拟收到邻居的路由更新）
    let mut update_interval = time::interval(Duration::from_secs(5));
    
    // 定义一个计数器，用来修改路由属性，让每次打印不一样
    let mut counter = 0;

    // 【关键点】：死循环，保证任务不退出
    loop {
        tokio::select! {
            // 事件 A: 定时器响了 (模拟周期性收到路由)
            _ = update_interval.tick() => {
                counter += 1;
                println!("\n--- [Tick: {}] BGP 状态机事件触发 ---", counter);

                // 1. 构造内部属性 (模拟每次 AS_Path 都在变)
                let private_attr = BgpAttributes {
                    origin: 0,
                    as_path: vec![64512, 100, counter], // 每次加一个 AS 号
                    local_pref: 100,
                };
                
                let prefix: IpNet = "1.1.1.0/24".parse().unwrap();
                println!("⚡ BGP 计算路由: {} (AS_Path: {:?})", prefix, private_attr.as_path);

                // 2. 发送给 RIB
                let event = RibEvent::Update {
                    protocol: RouteProtocol::BGP,
                    prefix,
                    nexthop: "192.168.1.1".parse().unwrap(),
                    metric: 0,
                    admin_distance: 20,
                };

                if let Err(e) = tx.send(event).await {
                    println!("❌ 发送失败 (可能是 RIB 挂了): {}", e);
                    // 如果发送失败，通常意味着接收端关闭了，我们可以选择退出循环
                    // break; 
                } else {
                    println!("📤 已推送到 RIB");
                }
            }

            // 事件 B: 这里未来可以加 socket.recv() 处理 TCP 报文
            // msg = socket.read() => { ... }
        }
    }
}