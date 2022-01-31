use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;

use communicator::CommunicatorId;
use device::DeviceId;

use crate::periphery::messages::{FromCommMsgContent, FromUnitMsgContent};

pub mod communicator;
pub mod device;
pub mod messages;

#[derive(Debug)]
pub struct FromCommMsg<T: FromCommMsgContent> {
    pub device_id: DeviceId,
    pub msg: T,
}

#[derive(Debug)]
pub struct FromUnitMsg<T: FromUnitMsgContent> {
    pub msg: T,
}

pub type SenderFromUnit<M> = Sender<FromUnitMsg<M>>;
pub type SenderFromComm<M> = Sender<FromCommMsg<M>>;

pub type ToCommSenderVec<M> = Vec<(CommunicatorId, SenderFromUnit<M>)>;
pub type FromUnitReceiverMap<M> = HashMap<CommunicatorId, Mutex<Receiver<FromUnitMsg<M>>>>;