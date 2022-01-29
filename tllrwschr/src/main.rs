use std::io::Error;
use std::sync::Arc;

use tokio::sync::mpsc::channel;
use tokio::sync::Mutex;

use core::periphery::communicator_id::CommunicatorId;
use core::periphery::messages::master::{FromCommMasterOrder, FromUnitMasterEvent};
use core::{
    central_unit::CentralUnit,
    periphery::{
        messages::FromCommMsgContent, messages::FromUnitMsgContent, FromCommMsg, FromUnitMsg,
    },
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, tllrwshr!");

    let (u_s, c_r) = channel::<FromUnitMsg<FromUnitMasterEvent>>(32 as usize);
    let (c_s, u_r) = channel::<FromCommMsg<FromCommMasterOrder>>(32 as usize);
    //Create central_unit

    let mut central_unit = CentralUnit::with_prime_comm(
        &c_s,
        &Arc::new(Mutex::new(c_r)),
        &u_s,
        &Arc::new(Mutex::new(u_r)),
    );
    //Create some communicator
    //Get communicator socket from c_u

    //TODO: Create tui communicator that sends Start signal
    c_s.try_send(FromCommMsg {
        device_id: 0,
        msg: FromCommMasterOrder::Start,
    })
    .unwrap_or_else(|_| panic!("No msg sent"));

    c_s.try_send(FromCommMsg {
        device_id: 0,
        msg: FromCommMasterOrder::Start,
    })
    .unwrap_or_else(|_| panic!("No msg sent"));

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
