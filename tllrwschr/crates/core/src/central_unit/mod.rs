use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{bail, Result};
use tokio::sync::mpsc::{channel, Receiver};
use tokio::sync::Mutex;

use types::UnitCommunicationHandler;

use crate::periphery::communicator::CommUnitConnectionSocket;
use crate::periphery::messages::comm::{FromCommGeneral, FromUnitGeneral};
use crate::periphery::{
    communicator::CommunicatorId,
    device::{Device, DeviceId},
    messages::master::{FromCommMasterOrder, FromUnitMasterEvent},
    messages::slave::{FromCommSlaveEvent, FromUnitSlaveOrder},
    messages::{FromCommMsgContent, FromUnitMsgContent},
    FromCommMsg, FromUnitMsg, FromUnitReceiverMap, SenderFromComm, SenderFromUnit, ToCommSenderVec,
};

pub mod types;

pub struct CentralUnit {
    //Channels
    master_channels: UnitCommunicationHandler<FromCommMasterOrder, FromUnitMasterEvent>,
    slave_channels: UnitCommunicationHandler<FromCommSlaveEvent, FromUnitSlaveOrder>,
    general_channels: UnitCommunicationHandler<FromCommGeneral, FromUnitGeneral>,

    prime_comm_sender: SenderFromComm<FromCommMasterOrder>,

    registered_devices: HashMap<DeviceId, Device>,
}

impl CentralUnit {
    pub fn with_prime_comm(
        prime_comm_master_sender: &SenderFromComm<FromCommMasterOrder>,
        prime_unit_master_sender: &SenderFromUnit<FromUnitMasterEvent>,
        prime_unit_master_recv: Mutex<Receiver<FromCommMsg<FromCommMasterOrder>>>,
        prime_unit_general_sender: &SenderFromUnit<FromUnitGeneral>,
        prime_unit_general_recv: Mutex<Receiver<FromCommMsg<FromCommGeneral>>>,
        //prime_comm_general_recv: Mutex<Receiver<FromUnitMsg<FromUnitGeneral>>>,
    ) -> Self {
        let comm_id = CommunicatorId::get_prime_comm_id();

        Self {
            prime_comm_sender: prime_comm_master_sender.clone(),
            master_channels: UnitCommunicationHandler {
                unit_sender: vec![(comm_id, prime_unit_master_sender.clone())],
                unit_receiver: Some(prime_unit_master_recv),
                //comms_receiver: comms_receiver_master,
            },

            slave_channels: UnitCommunicationHandler {
                unit_sender: Vec::with_capacity(8_usize),
                //comms_receiver: FromUnitReceiverMap::with_capacity(8_usize),
                unit_receiver: None,
            },
            registered_devices: Default::default(),
            general_channels: UnitCommunicationHandler {
                unit_sender: vec![(comm_id, prime_unit_general_sender.clone())],
                //comms_receiver: comms_receiver_general,
                unit_receiver: Some(prime_unit_general_recv),
            },
        }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        //TODO: Some init logic?

        let mut general_recv = self
            .general_channels
            .unit_receiver
            .as_ref()
            .unwrap()
            .lock()
            .await; //TODO: Return lock?
        let msg = tokio::time::timeout(Duration::from_secs(60), general_recv.recv())
            .await
            .unwrap()
            .unwrap();
        println!(
            "Received first message from general channel by device {}",
            msg.device_id
        );

        //TODO: let mut slave_recv = self
        //    .slave_channels
        //.unit_receiver
        //    .as_ref()
        //    .unwrap()
        //.lock()
        //    .await; //TODO: Return lock?

        //Loop for entirety of program
        loop {
            while let Ok(msg) = &general_recv.try_recv() {
                self.handle_general_msg(msg).unwrap();
            }

            //TODO: while let Ok(msg) = &slave_recv?.try_recv() {}

            //If appropriate command or event, forward to game scheduler

            //Frequency of 60 Hz to avoid battery loss
            tokio::time::sleep(Duration::from_millis(166)).await;
        }
    }

    fn handle_master_msg(&self, msg: &FromCommMsg<FromCommMasterOrder>) -> Result<()> {
        println!("Received master message from device {}", msg.device_id);
        Ok(())
    }

    fn handle_slave_msg(&self, msg: &FromCommMsg<FromCommSlaveEvent>) -> Result<()> {
        println!("Received slave message from device {}", msg.device_id);
        Ok(())
    }

    fn handle_general_msg(&self, msg: &FromCommMsg<FromCommGeneral>) -> Result<()> {
        println!("Received general message from device {}", msg.device_id);
        Ok(())
    }

    ///Create channels, sender and receivers and generate a communicator socket with specified messages
    fn generate_add_comms_socket<ToUnit: FromCommMsgContent, ToComm: FromUnitMsgContent>(
        &mut self,
        channels: &mut UnitCommunicationHandler<ToUnit, ToComm>,
        prime_comm_sender: &SenderFromComm<ToUnit>,
        comm_id: &CommunicatorId,
    ) -> Result<CommUnitConnectionSocket<ToUnit, ToComm>> {
        //Simple instructions:
        //Unit -> Comm NEW
        //Comm -> Unit COPY!

        //Find prime communicator and copy its sender to create a new instance for this communicator
        let _comm_send = &self.prime_comm_sender;

        let can_insert_send = !channels.unit_sender.iter().any(|(c, _)| c == comm_id);

        if !(can_insert_send) {
            bail!("Comm {} already exists!", comm_id)
        }
        //Create new channel for Unit to Communicator messages
        let (unit_send, comm_recv) = channel::<FromUnitMsg<ToComm>>(32_usize);
        let comm_recv = Mutex::new(comm_recv);

        channels.unit_sender.push((*comm_id, unit_send));

        Ok(CommUnitConnectionSocket {
            to_unit: prime_comm_sender.clone(),
            from_unit: comm_recv,
        })
    }
}