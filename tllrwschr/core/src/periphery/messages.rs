use crate::game::input::{InputContent, InputData, InputId};
use crate::game::output::{GameOutput, GameOutputContent};

pub trait ToDeviceCommMsg {}
pub trait ToUnitCommMsg {}

pub enum ToMasterDeviceMsg {
    UnitShutdownEvent {
        unit_is_restarting: bool,
        reason: ShutdownReason,
    },
    GameMessage(GameOutputContent),
}
impl ToDeviceCommMsg for ToMasterDeviceMsg {}

pub enum ShutdownReason {
    //Probably only used when error crash happens
    Unknown,
    ///Central unit has been shutdown manually
    Manual,
    ///Shutdown due to low battery/no power supply
    EnergyLow,
}

pub enum FromMasterDeviceOrder {
    Start,
    Input(InputContent),
    SetPerDeviceSettings(MasterDeviceSettings),
    SetUnitSettings(UnitSettings),
}
impl<'a> ToUnitCommMsg for FromMasterDeviceOrder {}

pub struct MasterDeviceSettings {
    pub send_raw_player_event: bool,
}

pub struct UnitSettings {
    sound_volume: u8,
}
