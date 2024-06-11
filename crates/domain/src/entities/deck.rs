use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Serialize derive is necessary for tauri command send serialized UUID to the front-end.
#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Deck {
    pub id: Uuid,
    pub title: String,
}

impl Deck {
    pub fn new(title: String) -> Self {
        Deck {
            id: Uuid::new_v4(),
            title,
        }
    }
}
