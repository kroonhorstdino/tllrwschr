use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::sync::Arc;

use anyhow::Result;
use tokio::sync::{mpsc::Receiver, Mutex};

use crate::periphery::device::Device;
use crate::periphery::messages::comm::{FromCommGeneral, FromUnitGeneral};
use crate::periphery::messages::master::{FromCommMasterOrder, FromUnitMasterEvent};
use crate::periphery::messages::slave::{FromCommSlaveEvent, FromUnitSlaveOrder};
use crate::periphery::messages::{FromCommMsgContent, FromUnitMsgContent};
use crate::periphery::{FromUnitMsg, SenderFromComm};

#[derive(Copy, Clone)]
pub enum ConnectionKind {
    Bluetooth,
    SerialPort,
    Ssh,
    Http,
}

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
pub struct CommunicatorId {
    pub id: u8,
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

/// Communicator is responsible for handling communication between devices of one connection type central unit
/// Handles protocols, method of communication, etc. internally
pub trait Communicator {
    fn with_communicator(id: CommunicatorId, comm: CommUnitConnection) -> Self;

    fn start(&self) -> Result<()>;
    fn close(&mut self) -> Result<()>;

    fn get_id(&self) -> CommunicatorId;
    fn get_registered_devices<'a>() -> Option<&'a Vec<Device>>;
}

pub struct CommData {
    pub id: CommunicatorId,
    pub conn: CommUnitConnection,
}

//Contains channels which are necessary for communication with central unit
pub struct CommUnitConnectionSocket<FromComm: FromCommMsgContent, FromUnit: FromUnitMsgContent> {
    pub to_unit: SenderFromComm<FromComm>,
    pub from_unit: Mutex<Receiver<FromUnitMsg<FromUnit>>>,
}

/// Contains comm device sockets for master and slave messages
pub struct CommUnitConnection {
    pub master: CommUnitConnectionSocket<FromCommMasterOrder, FromUnitMasterEvent>,
    pub slave: Option<CommUnitConnectionSocket<FromCommSlaveEvent, FromUnitSlaveOrder>>,
    pub general: CommUnitConnectionSocket<FromCommGeneral, FromUnitGeneral>,
}