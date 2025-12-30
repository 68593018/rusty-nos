use std::net::IpAddr;
use ipnet::IpNet;

// 移除 BgpAttributes 定义，因为它不属于公共契约
// 移除 Arc 引用

// 定义通用的路由来源类型
#[derive(Debug, Clone, Copy)]
pub enum RouteProtocol {
    Static,
    BGP,
    OSPF,
    Direct,
}

// 定义通用的路由更新事件
// 这是所有协议组件（BGP, OSPF）向 RIB 汇报的标准格式
#[derive(Debug, Clone)]
pub enum RibEvent {
    Update {
        protocol: RouteProtocol,
        prefix: IpNet,
        nexthop: IpAddr,
        metric: u32,
        // admin_distance 用于 RIB 比较不同协议的优先级 (例如 BGP=20, OSPF=10)
        admin_distance: u8, 
    },
    Withdraw {
        protocol: RouteProtocol,
        prefix: IpNet,
    },
}