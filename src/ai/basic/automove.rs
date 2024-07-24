use anyhow::{bail, Result};
use async_trait::async_trait;
use entropy_base::grid::navi;

use crate::{ai::logic::GuestLogic, client::Visit};

/// 以当前点为中心，顺时针向外遍历Node
pub struct AutoMovement {
    planned_step_length: usize,
    current_step_length: usize,
    current_direction: navi::Direction,
}

impl AutoMovement {
    pub fn new() -> Self {
        Self {
            planned_step_length: 0,
            current_step_length: 0,
            current_direction: navi::UP,
        }
    }
}

#[async_trait]
impl GuestLogic for AutoMovement {
    async fn init(&mut self, _guest: &mut impl Visit) -> Result<()> {
        Ok(())
    }
    async fn tick(&mut self, guest: &mut impl Visit) -> Result<()> {
        if guest.energy < 1 {
            bail!("energy not enough")
        }
        if self.current_direction == navi::UP{
            self.planned_step_length += 1;
        }

        guest.walk(self.current_direction).await?;
        self.current_step_length += 1;

        if self.current_step_length > self.planned_step_length{
            todo!()

        }

        self.planned_step_length;
        Ok(())
    }
}
