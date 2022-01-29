use crate::game::input::InputContent;
use crate::game::output::GameOutputContent;
use crate::game::scheduler::PlayerId;
use crate::periphery::device::DeviceId;
use crate::periphery::{FromCommMsgContent, FromUnitMsgContent};

///Messages sent from unit to slave communicator
pub enum FromUnitSlaveOrder {
    //Set state of slave
    SetState(SlaveState),
    GameMessage(GameOutputContent),
}
impl FromUnitMsgContent for FromUnitSlaveOrder {}

///Messages sent from slave to unit communicator
pub enum FromCommSlaveEvent {
    Disconnected {
        device_id: DeviceId,
        content: SlaveDisconnected,
    },
    Input(InputContent),
}
impl FromCommMsgContent for FromCommSlaveEvent {}

pub enum SlaveDisconnected {
    Manual,
    BatteryLow,
    SerialInterrupted,
}

pub type LightId = u8;

pub enum SlaveState {
    Off,    //Slave is offline
    Idle,   //Will not send any signals until reactivated by master
    Active, //Will send signals to master until set to off or idle
}
