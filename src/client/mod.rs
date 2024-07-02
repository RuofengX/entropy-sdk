pub mod http;

use std::ops::Deref;

use anyhow::Result;
use async_trait::async_trait;

use entropy_base::{
    entity::{Guest, GuestInfo, Player, PlayerInfo},
    grid::{navi, Node, NodeData, NodeID},
};
use futures::stream::FuturesUnordered;

/// 可能被远程或其他客户端改变
#[async_trait]
pub trait PhantomRead {
    async fn refresh(&mut self) -> Result<()>;
}

#[async_trait]
pub trait Access: Sized {
    fn from_url(server: String) -> Result<Self>;
    async fn ping(&self) -> Result<()>;
    async fn player_info(&self, id: i32) -> Result<PlayerInfo>;
    async fn player_register<T: AsRef<str> + Sync + Send>(
        &self,
        name: T,
        password: T,
    ) -> Result<Player>;
    async fn player_verify<T: AsRef<str> + Sync + Send>(
        &self,
        id: i32,
        password: T,
    ) -> Result<Player>;
    async fn play<T: AsRef<str> + Sync + Send>(&self, id: i32, password: T) -> Result<impl Play>;
    async fn guide(&self) -> Result<impl Guide>;
}

#[async_trait]
pub trait Play: Deref<Target = Player> + Sized {
    async fn list_guest(&self) -> Result<Vec<Guest>>;
    async fn spawn_guest(&self) -> Result<Guest>;
    async fn visit<'g>(&'g self, guest_id: i32) -> Result<impl Visit<'g>>;
}

#[async_trait]
pub trait Visit<'g>: Deref<Target = Guest> + PhantomRead + Sized {
    async fn detect(&self) -> Result<Vec<GuestInfo>>;
    async fn walk(&mut self, to: navi::Direction) -> Result<()>;
    async fn harvest(&mut self, at: usize) -> Result<()>;
    async fn arrange(&mut self, transfer_energy: i64) -> Result<impl Visit>;
    async fn heat(&mut self, at: usize, energy: i64) -> Result<()>;
}

#[async_trait]
pub trait Guide {
    async fn get_node(&self, id: NodeID) -> Result<Node>;
    async fn list_nodes(
        &self,
        ids: impl Iterator<Item = NodeID> + Sync + Send,
    ) -> Result<Vec<Node>>;
}

#[async_trait]
pub trait CachedGuide {
    async fn get_node_cached(&self, id: NodeID) -> Result<Node>;
    async fn list_nodes_cached(
        &self,
        ids: impl Iterator<Item = NodeID>,
    ) -> Result<Vec<Node>>;
    fn truncate_cache(&self);
    fn outdate_cache(&self, id:NodeID);
}
