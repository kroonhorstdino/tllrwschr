use crate::game::input::InputContent;
use crate::periphery::messages::{FromCommMsgContent, FromUnitMsgContent};

#[derive(Debug)]
pub enum FromCommGeneral {
    CommReady,
    CommDisconnected,
    DeviceConnected,
    DeviceDisconnected,
    Input(InputContent),
}
impl FromCommMsgContent for FromCommGeneral {}

pub enum FromUnitGeneral {
    Shutdown(ShutdownReason),
}
impl FromUnitMsgContent for FromUnitGeneral {}

pub enum ShutdownReason {
    //Probably only used when error crash happens
    Unknown,
    ///Central unit has been shutdown manually
    Manual,
    ///Shutdown due to low battery/no power supply
    EnergyLow,
}