use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{bail, Result};
use tokio::sync::mpsc::{channel, Receiver};

use crate::game::output::{GameOutput, OutputTarget};
use crate::periphery::messages::{
    FromMasterDeviceOrder, ToDeviceCommMsg, ToMasterDeviceMsg, ToUnitCommMsg,
};
use crate::periphery::slave::messages::{FromSlaveDeviceEvent, ToSlaveDeviceMsg};
use crate::periphery::{
    CommunicatorDeviceSocket, CommunicatorId, Device, DeviceId, FromCommsMsg, FromUnitMsg,
    FromUnitReceiverMap, SenderFromComm, SenderFromUnit, ToCommsSenderVec,
};

pub(crate) struct UnitCommunicationHandler<ToUnit: ToUnitCommMsg, ToDevice: ToDeviceCommMsg> {
    ///Stores channels targeting a communicator each
    pub unit_sender: ToCommsSenderVec<ToDevice>,
    ///Receiver end of unit senders
    pub comms_receiver: FromUnitReceiverMap<ToDevice>,

    ///One sender for each communicator sends messages here, if any exist
    pub unit_receiver: Option<RefCell<Receiver<FromCommsMsg<ToUnit>>>>,
}

pub(crate) struct UnitGameOutputSenderHandle {
    master_sender: ToCommsSenderVec<ToMasterDeviceMsg>,
    slave_sender: ToCommsSenderVec<ToSlaveDeviceMsg>,
}

impl UnitGameOutputSenderHandle {
    pub fn send_output(&self, output: &Vec<GameOutput>) -> Result<()> {
        self.send_output_internal(output)
    }

    fn send_output_internal(&self, output: &Vec<GameOutput>) -> Result<()> {
        for o in output {
            match o.target {
                OutputTarget::All => {}
                OutputTarget::Player(_) => {}
                OutputTarget::PlayerAll => self.slave_sender.iter().for_each(|(_, c)| {
                    c.send(FromUnitMsg {
                        msg: ToSlaveDeviceMsg::GameMessage(o.content.to_owned()),
                    });
                }),
                OutputTarget::MasterOnly => self.master_sender.iter().for_each(|(_, c)| {
                    c.send(FromUnitMsg {
                        msg: ToMasterDeviceMsg::GameMessage(o.content.to_owned()),
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

pub struct CentralUnit {
    //Channels
    master_channels: UnitCommunicationHandler<FromMasterDeviceOrder, ToMasterDeviceMsg>,
    slave_channels: UnitCommunicationHandler<FromSlaveDeviceEvent, ToSlaveDeviceMsg>,

    prime_comm_sender: SenderFromComm<FromMasterDeviceOrder>,

    registered_devices: HashMap<DeviceId, Device>,
}

impl CentralUnit {
    pub fn with_prime_comm<'a>(
        prime_comm_sender: &'a SenderFromComm<FromMasterDeviceOrder>,
        prime_comm_recv: &'a Arc<Receiver<FromUnitMsg<ToMasterDeviceMsg>>>,
        prime_unit_sender: &'a SenderFromUnit<ToMasterDeviceMsg>,
        prime_unit_recv: &'a RefCell<Receiver<FromCommsMsg<FromMasterDeviceOrder>>>,
    ) -> Self {
        let comm_id = CommunicatorId::get_prime_comm_id();

        Self {
            prime_comm_sender: prime_comm_sender.clone(),
            master_channels: UnitCommunicationHandler {
                unit_sender: vec![(comm_id.clone(), prime_unit_sender.clone())],
                comms_receiver: FromUnitReceiverMap::from([(
                    comm_id.clone(),
                    prime_comm_recv.clone(),
                )]),
                unit_receiver: Some(prime_unit_recv.clone()),
            },

            slave_channels: UnitCommunicationHandler {
                unit_sender: Vec::with_capacity(8 as usize),
                comms_receiver: FromUnitReceiverMap::with_capacity(8 as usize),
                unit_receiver: None,
            },
            registered_devices: Default::default(),
        }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        //Register already connected slaves on serial ports

        //Some init logic?
        loop {
            //Wait for command, event or game event

            let unit_recv = &mut self.master_channels.unit_receiver..unwrap();
            let msg = unit_recv.get_mut();

            //If appropriate command or event, forward to game scheduler
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }

    ///Create channels, sender and receivers and generate a communicator socket with specified messages
    fn generate_add_comms_socket<ToUnit: ToUnitCommMsg, ToComm: ToDeviceCommMsg>(
        &mut self,
        channels: &mut UnitCommunicationHandler<ToUnit, ToComm>,
        prime_comm_sender: &SenderFromComm<ToUnit>,
        comm_id: &CommunicatorId,
    ) -> Result<CommunicatorDeviceSocket<ToUnit, ToComm>> {
        //Simple instructions:
        //Unit -> Comm NEW
        //Comm -> Unit COPY!

        //Find prime communicator and copy its sender to create a new instance for this communicator
        let comm_send = &self.prime_comm_sender;

        let (can_insert_send, can_insert_recv) = (
            !channels.unit_sender.iter().any(|(c, _)| &c == &comm_id),
            !channels.comms_receiver.iter().any(|(c, _)| &c == &comm_id),
        );

        if !(can_insert_send && can_insert_recv) {
            bail!("Comm {} already exists!", comm_id)
        }
        //Create new channel for Unit to Communicator messages
        let (unit_send, comm_recv) = channel::<FromUnitMsg<ToComm>>(32 as usize);
        let comm_recv = Arc::new(comm_recv);

        channels
            .unit_sender
            .push((comm_id.clone(), unit_send.clone()));
        channels
            .comms_receiver
            .insert(comm_id.clone(), comm_recv.clone());

        Ok(CommunicatorDeviceSocket {
            to_unit: prime_comm_sender.clone(),
            from_unit: comm_recv,
        })
    }
}
