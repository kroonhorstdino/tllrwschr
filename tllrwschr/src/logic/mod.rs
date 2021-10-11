use self::{command::Command, game::GamePlan};

pub mod command;
pub mod game;

pub struct Scheduler {}

impl Scheduler {
    pub fn start(&self) {}
    pub fn stop(&self) {}

    pub fn run_plan(&self, plan: impl GamePlan) {}
    pub fn stop_current_plan(&self) {}

    pub fn get_current_status(&self) -> bool {
        true
    }
}
