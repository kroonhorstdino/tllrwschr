use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
pub struct CommunicatorId {
    pub id: u8,
}

impl Display for CommunicatorId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.id, f)
    }
}

impl CommunicatorId {
    //Prime communicator, not using zero to avoid defaults!
    pub fn get_prime_comm_id() -> Self {
        Self { id: 1 }
    }
}
