pub mod messages;

pub type LightId = u8;

pub enum SlaveState {
    Off,    //Slave is offline
    Idle,   //Will not send any signals until reactivated by master
    Active, //Will send signals to master until set to off or idle
}

// /// Available slave buttons
// #[derive(Eq, PartialEq)]
// pub enum SlaveInput {
//     Primary { press_dur: u64 },
//     Secondary { press_dur: u64 },
//     //TextField(String),
//     //NumberField(u64)
//     //etc
// }
//
// #[derive(Eq, PartialEq)]
// pub enum SlaveLayout {
//     Primary,
//     Secondary,
//     //TextField
//     //NumberField
// }
