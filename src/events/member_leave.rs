use crate::events::Member;
use crate::events::Source;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MemberLeaveEvent {
    pub mode: String,
    pub timestamp: i64,
    pub source: Source,
    pub left: Left,
}

#[derive(Deserialize, Debug)]
pub struct Left {
    pub members: Vec<Member>,
}
