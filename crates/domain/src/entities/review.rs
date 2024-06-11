use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Serialize derive is necessary for tauri command send serialized UUID to the front-end.
// TODO: remove derive Serialize from here and serialize it in the controller (?).
#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Review {
    pub id: Uuid,
    pub deck_id: Uuid,
    pub flashcard_id: Uuid,
    pub quality: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthlyReviewStats {
    pub month: String,
    pub count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityWiseReview {
    pub quality: String,
    pub count: i32,
}

impl Review {
    pub fn new(deck_id: Uuid, flashcard_id: Uuid, quality: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            deck_id,
            flashcard_id,
            quality,
        }
    }
}
