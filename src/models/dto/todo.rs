use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TodoDTO {
    pub id: String,
}
