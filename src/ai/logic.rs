use anyhow::Result;
use async_trait::async_trait;

use crate::client::{Play, Visit};

#[async_trait]
pub trait PlayerLogic: Sized + Send + Sync {
    async fn init(&mut self, player: &mut impl Play) -> Result<()>;
    async fn tick(&mut self, player: &mut impl Play) -> Result<()>;
}

#[async_trait]
pub trait GuestLogic: Sized + Send + Sync {
    async fn init(&mut self, guest: &mut impl Visit) -> Result<()>;
    async fn tick(&mut self, guest: &mut impl Visit) -> Result<()>;
}
