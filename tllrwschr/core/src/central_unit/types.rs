use crate::game::output::{GameOutput, OutputTarget};
use crate::periphery::messages::master::FromUnitMasterEvent;
use crate::periphery::messages::slave::FromUnitSlaveOrder;
use crate::periphery::messages::{FromCommMsgContent, FromUnitMsgContent};
use crate::periphery::{FromCommMsg, FromUnitMsg, FromUnitReceiverMap, ToCommSenderVec};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex;

pub(crate) struct UnitCommunicationHandler<ToUnit: FromCommMsgContent, ToDevice: FromUnitMsgContent>
{
    ///Stores channels targeting a communicator each
    pub unit_sender: ToCommSenderVec<ToDevice>,
    ///Receiver end of unit senders
    pub comms_receiver: FromUnitReceiverMap<ToDevice>,

    ///One sender for each communicator sends messages here, if any exist
    pub unit_receiver: Option<Arc<Mutex<Receiver<FromCommMsg<ToUnit>>>>>,
}

pub(crate) struct UnitGameOutputSenderHandle {
    master_sender: ToCommSenderVec<FromUnitMasterEvent>,
    slave_sender: ToCommSenderVec<FromUnitSlaveOrder>,
}

impl UnitGameOutputSenderHandle {
    pub fn send_output(&self, output: &Vec<GameOutput>) -> anyhow::Result<()> {
        self.send_output_internal(output)
    }

    fn send_output_internal(&self, output: &Vec<GameOutput>) -> anyhow::Result<()> {
        for o in output {
            match o.target {
                OutputTarget::All => {}
                OutputTarget::Player(_) => {}
                OutputTarget::PlayerAll => self.slave_sender.iter().for_each(|(_, c)| {
                    c.send(FromUnitMsg {
                        msg: FromUnitSlaveOrder::GameMessage(o.content.to_owned()),
                    });
                }),
                OutputTarget::MasterOnly => self.master_sender.iter().for_each(|(_, c)| {
                    c.send(FromUnitMsg {
                        msg: FromUnitMasterEvent::GameMessage(o.content.to_owned()),
                    });
                }),
            }
        }

        Ok(())
    }
}

pub type StringLocaliserId = u16;

pub struct StringLocaliser<'a> {
    pub map: HashMap<StringLocaliserId, &'a str>,
}
