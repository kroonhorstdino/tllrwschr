use crate::central_unit::StringLocaliserId;
use crate::game::scheduler::PlayerId;
use std::ops::RangeInclusive;

pub struct GameOutput {
    pub target: OutputTarget,
    pub content: GameOutputContent,
}

///Defines target of output
pub enum OutputTarget {
    All,
    ///If multiple players are meant to be targeted, simply create more output structs
    ///If not specifically filtered, player events are also forwarded to master
    Player(PlayerId),
    PlayerAll,
    MasterOnly,
}

#[derive(Clone)]
pub enum GameOutputContent {
    Display {
        display_id: u8,
        is_permanent: bool,
        style_options: bool,
        content: DisplayContent,
    },
    PlayerAdded {
        player_id: PlayerId,
    },
    PlayerRemoved {
        player_id: PlayerId,
    },
    PhaseEvent(StringLocaliserId),
    GeneralEvent(StringLocaliserId),
    CustomStr {
        msg: StringLocaliserId,
    },
}

#[derive(Clone)]
pub enum DisplayContent {
    Confirm {
        text: StringLocaliserId,
    },
    ///Binary choice between two, two different choices
    Binary {
        opt_a: StringLocaliserId,
        opt_b: StringLocaliserId,
    },
    ///Multiple choices, each with own text
    Multiple {
        opts: Vec<StringLocaliserId>,
    },
    ///Selection of items, all confirmed by one action/button
    Selection {
        opts: Vec<StringLocaliserId>,
        confirm_txt: StringLocaliserId,
    },
    ///Same as Selection, but with multiple items selectable at a time
    SelectionMultiple {
        ///Amount of acceptable selected items
        range: RangeInclusive<u8>,
        opts: Vec<StringLocaliserId>,
        confirm_txt: StringLocaliserId,
    },
}
