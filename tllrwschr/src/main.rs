use std::io::Error;

use tokio::sync::mpsc::channel;
use tokio::sync::Mutex;

use communicator_tui::TuiCommunicator;
use core::periphery::messages::comm::{FromCommGeneral, FromUnitGeneral};
use core::periphery::messages::master::{FromCommMasterOrder, FromUnitMasterEvent, UnitStatus};
use core::{
    central_unit::CentralUnit,
    periphery::{
        communicator::{
            CommUnitConnection, CommUnitConnectionSocket, Communicator, CommunicatorId,
        },
        FromCommMsg, FromUnitMsg,
    },
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, tllrwshr!");

    let (unit_master_sender, comm_master_recv) =
        channel::<FromUnitMsg<FromUnitMasterEvent>>(32_usize);
    let (comm_master_sender, unit_master_recv) =
        channel::<FromCommMsg<FromCommMasterOrder>>(32_usize);
    let (comm_general_sender, unit_general_recv) =
        channel::<FromCommMsg<FromCommGeneral>>(32_usize);
    let (unit_general_sender, comm_general_recv) =
        channel::<FromUnitMsg<FromUnitGeneral>>(32_usize);
    //Create central_unit

    let mut central_unit = CentralUnit::with_prime_comm(
        &comm_master_sender,
        &unit_master_sender,
        Mutex::new(unit_master_recv),
        &unit_general_sender,
        Mutex::new(unit_general_recv),
    );
    //Create some communicator.
    //Get communicator socket from c_u

    comm_master_sender
        .try_send(FromCommMsg {
            msg: FromCommMasterOrder::SetUnitStatus(UnitStatus::Active),
            device_id: 8,
        })
        .unwrap_or_else(|_| panic!("Test"));

    let tui = TuiCommunicator::with_communicator(
        CommunicatorId { id: 0 },
        CommUnitConnection {
            master: CommUnitConnectionSocket {
                to_unit: comm_master_sender,
                from_unit: Mutex::new(comm_master_recv),
            },
            slave: None,
            general: CommUnitConnectionSocket {
                to_unit: comm_general_sender,
                from_unit: Mutex::new(comm_general_recv),
            },
        },
    );
    tui.start();

    tokio::spawn(async move {
        central_unit.run().await.unwrap();
        println!("Central unit done!");
    })
    .await
    .unwrap();

    //Send message back and forth, process it through logging
    //???
    //Profit!

    //Each slave is conceptually the same, differences in hardware and other
    // behaviour are abstracted away in the interpreter/communicator layer

    //Interpreter converts unit/game signals into peripheral signals. This may just be a conversion into another protocol
    //for "high-end" devices (bluetooth, http) or an entire translation into new signals, e.g. for arduino slave boxes
    Ok(())
}