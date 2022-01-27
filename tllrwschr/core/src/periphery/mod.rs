pub mod messages;
pub mod slave;

use crate::periphery::messages::{ToDeviceCommMsg, ToUnitCommMsg};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Octal};
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};

pub type DeviceId = u8;

#[derive(Copy, Clone)]
pub enum ConnectionKind {
    Bluetooth,
    SerialPort,
    Ssh,
    Http,
}

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
}

pub struct FromCommsMsg<T: ToUnitCommMsg> {
    pub device_id: DeviceId,
    pub msg: T,
}

pub struct FromUnitMsg<T: ToDeviceCommMsg> {
    pub msg: T,
}

//Contains channels which are necessary for communication with central unit
pub struct CommunicatorDeviceSocket<ToUnit: ToUnitCommMsg, ToDevice: ToDeviceCommMsg> {
    pub to_unit: SenderFromComm<ToUnit>,
    pub from_unit: Arc<Receiver<FromUnitMsg<ToDevice>>>,
}

pub type SenderFromUnit<M: ToDeviceCommMsg> = Sender<FromUnitMsg<M>>;
pub type SenderFromComm<M: ToUnitCommMsg> = Sender<FromCommsMsg<M>>;

pub type ToCommsSenderVec<M: ToDeviceCommMsg> = Vec<(CommunicatorId, SenderFromUnit<M>)>;
pub type FromUnitReceiverMap<M: ToDeviceCommMsg> =
    HashMap<CommunicatorId, Arc<Receiver<FromUnitMsg<M>>>>;

#[derive(Eq, PartialEq, Clone, Copy)]
pub struct CommunicatorId {
    pub id: u8,
}

impl Hash for CommunicatorId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl Display for CommunicatorId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.id, f)
    }
}

impl CommunicatorId {
    //Prime communicator, not using zero to avoid defaults!
    pub fn get_prime_comm_id() -> Self {
        Self { id: 1 }
    }
}
