use core::{
    central_unit::CentralUnit,
    periphery::{
        messages::FromMasterDeviceOrder, messages::ToDeviceCommMsg, messages::ToMasterDeviceMsg,
        messages::ToUnitCommMsg, CommunicatorId, FromCommsMsg, FromUnitMsg,
    },
};
use std::io::Error;
use std::sync::Arc;
use tokio::sync::mpsc::channel;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, tllrwshr!");

    let (u_s, c_r) = channel::<FromUnitMsg<ToMasterDeviceMsg>>(32 as usize);
    let (c_s, u_r) = channel::<FromCommsMsg<FromMasterDeviceOrder>>(32 as usize);
    //Create central_unit

    let central_unit =
        &mut CentralUnit::with_prime_comm(&c_s, &Arc::new(c_r), &u_s, &Arc::new(u_r));
    //Create some communicator
    //Get communicator socket from c_u

    central_unit.run().await;

    //Send message back and forth, process it through logging
    //???
    //Profit!

    //Each slave is conceptually the same, differences in hardware and other
    // behaviour are abstracted away in the interpreter/communicator layer

    //Interpreter converts unit/game signals into peripheral signals. This may just be a conversion into another protocol
    //for "high-end" devices (bluetooth, http) or an entire translation into new signals, e.g. for arduino slave boxes
    Ok(())
}
