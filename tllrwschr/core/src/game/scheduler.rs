use std::borrow::Borrow;

use anyhow::{anyhow, Result};

use crate::central_unit::types::UnitGameOutputSenderHandle;
use crate::game::GamePlan;
use crate::game::output::{GameOutput, GameOutputContent};
use crate::periphery::device::DeviceId;
use crate::periphery::messages::master::{FromCommMasterOrder, FromUnitMasterEvent};
use crate::periphery::messages::slave::{FromCommSlaveEvent, FromUnitSlaveOrder};

pub type PlayerId = u8;

///Takes care of processing game plan
pub struct GameScheduler<Plan: GamePlan> {
    //TODO: Teams? Just a grouping of players
    active_players: Vec<(DeviceId, PlayerId)>,
    inactive_players: Vec<(DeviceId, PlayerId)>,

    sender_handle: UnitGameOutputSenderHandle,
    game_plan: Option<Plan>,
}

///React to inputs with game plan state machine
impl<Plan: GamePlan> GameScheduler<Plan> {
    pub fn event_from_slave(&self, signal: &FromCommSlaveEvent) {
        // Trigger state machine game plan
    }

    pub fn order_from_master(&self, signal: &FromCommMasterOrder) -> Result<()> {
        if let Ok(outputs) = self
            .game_plan
            .as_ref()
            .ok_or_else(|| anyhow!("No game plan active!"))?
            .post_master_input()
        {
            ///Responses which are to be processed further instead of just sent ahead
            //self.sender_handle
            // .send_output(outputs.iter().map(|o| ToMasterDeviceMsg::GameMessage()));
            let mut process_responses = outputs.iter().filter_map(|o| match o.content {
                GameOutputContent::PlayerAdded { .. } => Some(o),
                GameOutputContent::PlayerRemoved { .. } => Some(o),
                GameOutputContent::PhaseEvent(_) => Some(o),
                _ => None,
            });
        }

        Ok(())
    }
}
