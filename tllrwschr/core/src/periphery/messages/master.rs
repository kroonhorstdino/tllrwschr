use crate::game::input::InputContent;
use crate::game::output::GameOutputContent;
use crate::periphery::messages::{FromCommMsgContent, FromUnitMsgContent};
use std::fmt::{write, Display, Formatter};

pub enum FromUnitMasterEvent {
    UnitShutdownEvent {
        unit_is_restarting: bool,
        reason: ShutdownReason,
    },
    GameMessage(GameOutputContent),
}

impl FromUnitMsgContent for FromUnitMasterEvent {}

pub enum ShutdownReason {
    //Probably only used when error crash happens
    Unknown,
    ///Central unit has been shutdown manually
    Manual,
    ///Shutdown due to low battery/no power supply
    EnergyLow,
}

pub enum FromCommMasterOrder {
    Start,
    Input(InputContent),
    SetPerDeviceSettings(MasterDeviceSettings),
    SetUnitSettings(UnitSettings),
}

impl<'a> FromCommMsgContent for FromCommMasterOrder {}

pub struct MasterDeviceSettings {
    pub send_raw_player_event: bool,
}

pub struct UnitSettings {
    sound_volume: u8,
}
