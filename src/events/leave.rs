use crate::events::Source;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LeaveEvent {
    pub mode: String,
    pub timestamp: i64,
    pub source: Source,
}
