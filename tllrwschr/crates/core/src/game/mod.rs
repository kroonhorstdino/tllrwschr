use anyhow::Result;
use async_trait::async_trait;

use crate::game::input::GameInputTypeSettings;
use crate::game::output::GameOutput;
use crate::game::scheduler::PlayerId;

pub mod input;
pub mod output;
pub mod scheduler;

///Encodes rules and actions for a game
///Acts like a state machine
///TODO: Create parser for game script langauge (like ludo)
#[async_trait]
pub trait GamePlan<O = GameOutput> {
    //Init, play, end phase.

    fn get_required_inputs(&self) -> Vec<GameInputTypeSettings>;

    /// Called for inputs from player
    fn post_player_input<'a>(&self) -> Result<&'a Vec<O>>;
    /// Called for inputs from master
    fn post_master_input<'a>(&self) -> Result<&'a Vec<O>>;

    fn post_phase_cmd(&self) -> Result<Vec<O>>;
}

pub struct Karrierepoker {
    //TODO: Required layout/buttons
    //layout: [SlaveLayout; 2],
    //TODO: state
    ///Order of players at start of each round
    all_players: Vec<PlayerId>,

    //Active players in turn (until all have skipped or one has won)
    active_turn_players: Vec<PlayerId>,
    //Players active in round, all others have removed all cards from hand
    active_round_players: Vec<PlayerId>,
}

//Basic request operations:
//MASTER
//SelectOptions with multiple options (identify with index) Vec of strings -> Response is index 0-n
//SelectBinary { opt_a: String, opt_b: String } -> Response is index 0-1
//Confirm { text: Option<String> }

//SLAVE
//No request operations to slave!

//Events:
//PressButtonDigital { input: SlaveInput }

//Master can always cancel game

//Init phase:

//Select players
//Listen to player button presses, add to player list
//Until -> Max reached, master confirms
// Dialogue -> Confirm order OR retry init -> Wait for answer

//Game phase:
//Start next round
// Wait for either
// Continue OR Skip by current player OR wait for 10 seconds
// If Continue, advance idx. If skip do that and remove from active players

//Master can at all times:
// Force Skip cur player, Force Continue cur player, end current round

//When

//End phase
//Display scores
//Winner is current first player in all_players
//}
