pub mod http;

use std::ops::Deref;

use anyhow::Result;
use async_trait::async_trait;

use crate::entity::{Guest, GuestInfo, Player, PlayerInfo};
use entropy_base::grid::{navi, NodeData};

/// 可能被远程或其他客户端改变
#[async_trait]
pub trait PhantomRead {
    async fn refresh(&mut self) -> Result<()>;
}

#[async_trait]
pub trait Access: Sized {
    fn from_url(server: String) -> Result<Self>;
    async fn server_ping(&self) -> Result<()>;
    async fn player_info(&self, id: i32) -> Result<PlayerInfo>;
    async fn player_register(&self, name: String, password: String) -> Result<Player>;
    async fn player_verify(&self, name: String, password: String) -> Result<Player>;
    async fn play(&self, name: String, password: String) -> Result<impl Play>;
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
pub trait Navigate {
    fn from_access(account: Player) -> Self;
    async fn node(&self, x: i16, y: i16) -> NodeData;
}
