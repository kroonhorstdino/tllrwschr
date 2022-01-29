use std::collections::HashMap;
use std::fmt::{Display, Formatter, Octal};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;

use communicator_id::CommunicatorId;
use device::DeviceId;

use crate::periphery::messages::{FromCommMsgContent, FromUnitMsgContent};

pub mod communicator_id;
pub mod device;
pub mod messages;

#[derive(Copy, Clone)]
pub enum ConnectionKind {
    Bluetooth,
    SerialPort,
    Ssh,
    Http,
}

pub struct FromCommMsg<T: FromCommMsgContent> {
    pub device_id: DeviceId,
    pub msg: T,
}

pub struct FromUnitMsg<T: FromUnitMsgContent> {
    pub msg: T,
}

//Contains channels which are necessary for communication with central unit
pub struct CommDeviceSocket<FromComm: FromCommMsgContent, FromUnit: FromUnitMsgContent> {
    pub to_unit: SenderFromComm<FromComm>,
    pub from_unit: Arc<Mutex<Receiver<FromUnitMsg<FromUnit>>>>,
}

pub type SenderFromUnit<M: FromUnitMsgContent> = Sender<FromUnitMsg<M>>;
pub type SenderFromComm<M: FromCommMsgContent> = Sender<FromCommMsg<M>>;

pub type ToCommSenderVec<M: FromUnitMsgContent> = Vec<(CommunicatorId, SenderFromUnit<M>)>;
pub type FromUnitReceiverMap<M: FromUnitMsgContent> =
    HashMap<CommunicatorId, Arc<Mutex<Receiver<FromUnitMsg<M>>>>>;
