use crate::periphery::communicator::CommunicatorId;
use crate::periphery::communicator::ConnectionKind;

pub type DeviceId = u8;

#[derive(Copy, Clone)]
pub struct DeviceInfo {
    pub name: &'static str,
    pub connection: ConnectionKind,
}

///Determines whether to receive or send commands to device
#[derive(Copy, Clone)]
pub enum DeviceKind {
    Slave,
    Master,
}

///Identifies one slave device
#[derive(Copy, Clone)]
pub struct Device {
    pub id: DeviceId,
    pub kind: DeviceKind,
    pub info: DeviceInfo,
    pub comm: CommunicatorId,
}