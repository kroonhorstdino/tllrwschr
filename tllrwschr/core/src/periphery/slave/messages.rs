use crate::game::input::InputContent;
use crate::game::output::GameOutputContent;
use crate::game::scheduler::PlayerId;
use crate::periphery::slave::SlaveState;
use crate::periphery::{DeviceId, ToDeviceCommMsg, ToUnitCommMsg};

///Messages sent from unit to slave communicator
pub enum ToSlaveDeviceMsg {
    //Set state of slave
    SetState(SlaveState),
    GameMessage(GameOutputContent),
}
impl ToDeviceCommMsg for ToSlaveDeviceMsg {}

///Messages sent from slave to unit communicator
pub enum FromSlaveDeviceEvent {
    Disconnected {
        device_id: DeviceId,
        content: SlaveDisconnected,
    },
    Input(InputContent),
}
impl ToUnitCommMsg for FromSlaveDeviceEvent {}

pub enum SlaveDisconnected {
    Manual,
    BatteryLow,
    SerialInterrupted,
}
