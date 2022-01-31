use crate::game::output::GameOutputContent;
use crate::periphery::messages::{FromCommMsgContent, FromUnitMsgContent};

pub enum FromUnitMasterEvent {
    GameMessage(GameOutputContent),
}

impl FromUnitMsgContent for FromUnitMasterEvent {}

pub enum FromCommMasterOrder {
    SetUnitStatus(UnitStatus),
    SetPerDeviceSettings(MasterDeviceSettings),
    SetUnitSettings(UnitSettings),
}

impl<'a> FromCommMsgContent for FromCommMasterOrder {}

pub enum UnitStatus {
    Off,
    Standby,
    Active,
}

pub struct MasterDeviceSettings {
    pub send_raw_player_event: bool,
}

pub struct UnitSettings {
    sound_volume: u8,
}