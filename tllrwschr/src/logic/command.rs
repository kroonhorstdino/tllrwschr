#[derive(Clone, Copy)]
pub enum Command {
    WaitForSeconds { length: u64 },
    Beep { length: u64 },
    LightBlink(BlinkData),
    LightToggle { LightType: u64, on: bool },
}

#[derive(Clone, Copy)]
pub struct BlinkData {
    pub length: u64,
    pub off_interval: u64,
    pub on_interval: u64,
    pub is_loop: bool,
}

pub enum LightType {
    Main = 0,
}
