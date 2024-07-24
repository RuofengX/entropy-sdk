use anyhow::{Ok, Result};
use async_trait::async_trait;
use entropy_base::grid::{Node, NodeData, NodeID};

use crate::{ai::logic::GuestLogic, client::Visit};

pub struct Harvester {
    node: Node,
    current_index: usize,
}
impl Harvester {
    pub fn new() -> Self {
        Self {
            node: Node {
                id: NodeID::ORIGIN,
                data: NodeData::from(b""),
            },
            current_index: 0,
        }
    }
}

#[async_trait]
impl GuestLogic for Harvester {
    async fn init(&mut self, guest: &mut impl Visit) -> Result<()> {
        self.node = guest.node().await?;
        Ok(())
    }
    async fn tick(&mut self, guest: &mut impl Visit) -> Result<()> {
        guest.harvest(self.current_index).await?;
        self.current_index += 1;
        if self.current_index >= self.node.data.len(){

        }
        Ok(())
    }
}
