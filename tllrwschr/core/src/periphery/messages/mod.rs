use crate::game::input::{InputContent, InputData, InputId};
use crate::game::output::{GameOutput, GameOutputContent};
use std::fmt::Display;

pub mod master;
pub mod slave;

pub trait FromUnitMsgContent {}
pub trait FromCommMsgContent {}
