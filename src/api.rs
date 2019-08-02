use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub id: Uuid,
    pub name: String,
}
