use anyhow::Result;
use async_trait::async_trait;

pub type PlayerBoxID = u8;

#[async_trait]
pub trait Communicator {
    async fn activate() -> Result<()>;
    async fn get_all_buttons() -> Result<()>;
    async fn deactivate() -> Result<()>;
}

#[async_trait]
pub trait PlayerBox {
    async fn connect(&self) -> Result<()>;
    async fn disconnect(&self) -> Result<()>;
    fn is_connected(&self) -> bool;
    async fn get_id(&self) -> PlayerBoxID;
}
