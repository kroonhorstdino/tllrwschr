use std::fmt::Display;

use crate::game::input::{InputContent, InputData, InputId};
use crate::game::output::{GameOutput, GameOutputContent};

pub mod master;
pub mod slave;

pub trait FromUnitMsgContent {}
pub trait FromCommMsgContent {}
