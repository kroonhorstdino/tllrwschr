use crate::central_unit::StringLocaliserId;
use crate::game::scheduler::PlayerId;
use crate::periphery::DeviceId;
use std::io::Bytes;
use std::ops::RangeInclusive;

pub type InputId = u8;

pub struct PlayerInput {
    player_id: PlayerId,
    content: InputContent,
}

pub struct InputContent {
    input_id: InputId,
    data: InputData,
}

/// Types of player input which are accepted by game plans
pub enum InputData {
    /// Can be physical button or virtual one, a button with text
    ButtonDigital,
    /// Same as digital button, but with analog inputs
    ButtonAnalog {
        val: i64,
    },
    Text(StringLocaliserId),
    Integer(i64),
    Float(f64),
    /// Unspecified protocol must be known by all participating devices
    CustomStr(StringLocaliserId),
}

pub enum MasterInput {
    PhaseCmd(MasterPhaseCmd),
    ///Add some player, must get device id returned!
    PlayerAdd,
    PlayerKick(PlayerId),
}

pub enum MasterPhaseCmd {
    EndGame,
    SkipToNext,
    SkipToPhase(u8),
    CustomCommand,
}

pub enum GameInputTypeSettings {
    ButtonDigital { is_large: bool },
    ButtonAnalog { range: RangeInclusive<i64> },
    Text { can_empty: bool, max_length: u16 },
    Integer { range: RangeInclusive<i64> },
    Float { range: RangeInclusive<f64> },
    RawOrder { custom_settings: String },
}
