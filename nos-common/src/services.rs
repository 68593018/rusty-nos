use async_trait::async_trait;
use ipnet::IpNet;
use std::net::IpAddr;

// 定义 RIB 服务的标准接口
// Send + Sync 是为了让它能在多线程 Runtime 中安全传递
#[async_trait]
pub trait RibService: Send + Sync {
    async fn add_route(&self, prefix: IpNet, nexthop: IpAddr);
}
