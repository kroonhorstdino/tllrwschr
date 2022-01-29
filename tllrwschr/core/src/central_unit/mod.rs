use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, bail, Result};
use tokio::sync::mpsc::{channel, Receiver};
use tokio::sync::Mutex;

use types::UnitCommunicationHandler;

use crate::{
    game::output::{GameOutput, OutputTarget},
    periphery::{
        communicator_id::CommunicatorId,
        device::{Device, DeviceId},
        messages::master::{FromCommMasterOrder, FromUnitMasterEvent},
        messages::slave::{FromCommSlaveEvent, FromUnitSlaveOrder},
        messages::{FromCommMsgContent, FromUnitMsgContent},
        CommDeviceSocket, FromCommMsg, FromUnitMsg, FromUnitReceiverMap, SenderFromComm,
        SenderFromUnit, ToCommSenderVec,
    },
};

pub mod types;

pub struct CentralUnit {
    //Channels
    master_channels: UnitCommunicationHandler<FromCommMasterOrder, FromUnitMasterEvent>,
    slave_channels: UnitCommunicationHandler<FromCommSlaveEvent, FromUnitSlaveOrder>,

    prime_comm_sender: SenderFromComm<FromCommMasterOrder>,

    registered_devices: HashMap<DeviceId, Device>,
}

impl CentralUnit {
    pub fn with_prime_comm<'a>(
        prime_comm_sender: &'a SenderFromComm<FromCommMasterOrder>,
        prime_comm_recv: &'a Arc<Mutex<Receiver<FromUnitMsg<FromUnitMasterEvent>>>>,
        prime_unit_sender: &'a SenderFromUnit<FromUnitMasterEvent>,
        prime_unit_recv: &Arc<Mutex<Receiver<FromCommMsg<FromCommMasterOrder>>>>,
    ) -> Self {
        let comm_id = CommunicatorId::get_prime_comm_id();

        let mut comms_receiver = FromUnitReceiverMap::with_capacity(4 as usize);
        comms_receiver.insert(comm_id.clone(), prime_comm_recv.clone());

        Self {
            prime_comm_sender: prime_comm_sender.clone(),
            master_channels: UnitCommunicationHandler {
                unit_sender: vec![(comm_id.clone(), prime_unit_sender.clone())],
                unit_receiver: Some(Arc::clone(prime_unit_recv)),
                comms_receiver,
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
        //TODO: Some init logic?

        let mut master_recv = self
            .master_channels
            .unit_receiver
            .as_ref()
            .unwrap()
            .lock()
            .await; //TODO: Return lock?
        let msg = tokio::time::timeout(Duration::from_secs(60), master_recv.recv())
            .await
            .unwrap()
            .unwrap();
        println!("Received first message from device {}", msg.device_id);

        //TODO: let mut slave_recv = self
        //    .slave_channels
        //.unit_receiver
        //    .as_ref()
        //    .unwrap()
        //.lock()
        //    .await; //TODO: Return lock?

        //Loop for entirety of program
        loop {
            while let Ok(msg) = &master_recv.try_recv() {
                self.handle_master_msg(msg).unwrap();
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

    ///Create channels, sender and receivers and generate a communicator socket with specified messages
    fn generate_add_comms_socket<ToUnit: FromCommMsgContent, ToComm: FromUnitMsgContent>(
        &mut self,
        channels: &mut UnitCommunicationHandler<ToUnit, ToComm>,
        prime_comm_sender: &SenderFromComm<ToUnit>,
        comm_id: &CommunicatorId,
    ) -> Result<CommDeviceSocket<ToUnit, ToComm>> {
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
        let comm_recv = Arc::new(Mutex::new(comm_recv));

        channels
            .unit_sender
            .push((comm_id.clone(), unit_send.clone()));
        channels
            .comms_receiver
            .insert(comm_id.clone(), comm_recv.clone());

        Ok(CommDeviceSocket {
            to_unit: prime_comm_sender.clone(),
            from_unit: comm_recv.clone(),
        })
    }
}
