use crate::periphery::PlayerBoxID;

//Game specific types and procedures
pub mod karrierepoker;

pub type PlayerID = u8;

///Contains instruction about how the control unit acts and how to respond to inputs from buttons and commands
pub trait GamePlan {}

pub struct Player {
    pub player_id: PlayerID,
    pub box_id: PlayerBoxID,
}
