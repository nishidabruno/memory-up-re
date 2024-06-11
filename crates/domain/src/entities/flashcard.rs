use serde::{Deserialize, Serialize};
use uuid::Uuid;

// use super::QuizSession;

// Serialize derive is necessary for tauri command send serialized UUID to the front-end.
// TODO: remove derive Serialize from here and serialize it in the controller (?).
#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Flashcard {
    pub id: Uuid,
    pub front: String,
    pub back: String,
    pub interval: i32,
    pub ease_factor: f32,
    pub repetitions: i32,
    pub deck_id: Uuid,
}

impl Flashcard {
    pub fn new(front: String, back: String, deck_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            front,
            back,
            deck_id,
            ease_factor: 2.5,
            interval: 0,
            repetitions: 0,
        }
    }

    pub fn review(&mut self, quality: i32) {
        // ease factor is updated even if the quality is low.
        self.ease_factor = (self.ease_factor - 0.8 + 0.28 * quality as f32
            - 0.02 * quality as f32 * quality as f32)
            .max(1.3);

        if quality < 3 {
            self.repetitions = 0;
            self.interval = 0;

            // For the cards which quality is lower than 3 we need to put it into a session
            // in order to repeat it until the quality is at least 4.
            // let mut session = QuizSession::new(self.deck_id);
            // session.add_low_quality_flashcard(self.id);
            return;
        }

        self.repetitions += 1;

        self.interval = match self.repetitions {
            1 => 1,
            2 => 6,
            _ => (self.interval as f32 * self.ease_factor).round() as i32,
        };
    }
}
