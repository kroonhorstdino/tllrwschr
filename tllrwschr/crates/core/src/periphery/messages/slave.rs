use crate::game::output::GameOutputContent;

use crate::periphery::{DeviceId, FromCommMsgContent, FromUnitMsgContent};

///Messages sent from unit to slave communicator
pub enum FromUnitSlaveOrder {
    //Set state of slave
    SetState(SlaveState),
    GameMessage(GameOutputContent),
}
impl FromUnitMsgContent for FromUnitSlaveOrder {}

///Messages sent from slave to unit communicator
pub enum FromCommSlaveEvent {
    DisconnectDevice(DeviceId),
}
impl FromCommMsgContent for FromCommSlaveEvent {}

pub enum SlaveDisconnected {
    Manual,
    BatteryLow,
    SerialInterrupted,
}

pub enum SlaveState {
    Off,    //Slave is offline
    Idle,   //Will not send any signals until reactivated by master
    Active, //Will send signals to master until set to off or idle
}