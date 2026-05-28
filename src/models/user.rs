use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct _User {
    pub id: u64,
    pub name: String,
    pub email: String,
}
