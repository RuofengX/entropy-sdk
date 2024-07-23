pub mod http;

use std::ops::Deref;

use anyhow::Result;
use async_trait::async_trait;

use entropy_base::{
    entity::{Guest, GuestInfo, Player, PlayerInfo},
    grid::{navi, Node, NodeID},
};

/// 可能被远程或其他客户端改变，所以叫Phantom
#[async_trait]
pub trait PhantomRead {
    async fn refresh(&mut self) -> Result<()>;
}

/// 基本的连接对象
#[async_trait]
pub trait Access: Sized {
    /// 从url字符串构建
    fn from_url(server: String) -> Result<Self>;

    /// 测试连通性
    async fn ping(&self) -> Result<()>;

    /// 获取玩家信息，返回公开的信息
    async fn player_info(&self, id: i32) -> Result<PlayerInfo>;

    /// 注册玩家，返回玩家完整模型
    async fn player_register<T: AsRef<str> + Sync + Send>(
        &self,
        name: T,
        password: T,
    ) -> Result<Player>;

    /// 测试用户名密码
    async fn player_verify<T: AsRef<str> + Sync + Send>(
        &self,
        id: i32,
        password: T,
    ) -> Result<Player>;

    /// 获取一个可被操控的对象
    async fn play<T: AsRef<str> + Sync + Send>(&self, id: i32, password: T) -> Result<impl Play>;

    /// 获取一个导航对象，用作地图
    async fn guide(&self) -> Result<impl Guide>;
}

#[async_trait]
pub trait Play: Deref<Target = Player> {
    async fn list_guest(&self) -> Result<Vec<Guest>>;
    async fn spawn_guest(&self) -> Result<Guest>;
    async fn visit<'g>(&'g self, guest_id: i32) -> Result<impl Visit<'g>>;
}

#[async_trait]
pub trait Visit<'g>: Deref<Target = Guest> + PhantomRead {
    async fn detect(&self) -> Result<Vec<GuestInfo>>;
    async fn walk(&mut self, to: navi::Direction) -> Result<()>;
    async fn harvest(&mut self, at: usize) -> Result<()>;
    async fn arrange(&mut self, transfer_energy: i64) -> Result<impl Visit>;
    async fn heat(&mut self, at: usize, energy: i64) -> Result<()>;
}

#[async_trait]
pub trait Guide {
    async fn get_node(&self, (x, y): (i16, i16)) -> Result<Node>;
    async fn list_nodes(
        &self,
        ids: impl Iterator<Item = (i16, i16)> + Sync + Send,
    ) -> Result<Vec<Node>>;
}

#[async_trait]
pub trait CachedGuide {
    async fn get_node_cached(&self, id: NodeID) -> Result<Node>;
    async fn list_nodes_cached(&self, ids: impl Iterator<Item = NodeID>) -> Result<Vec<Node>>;
    fn truncate_cache(&self);
    fn outdate_cache(&self, id: NodeID);
}
