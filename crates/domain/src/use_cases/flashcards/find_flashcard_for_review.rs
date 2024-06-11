use anyhow::Result;
use utils::AppError;
use uuid::Uuid;

use crate::{entities::Flashcard, repositories::FlashcardsRepository};

pub struct FindNextFlashcardForReviewUseCase<T> {
    repository: Box<dyn FlashcardsRepository<T>>,
}

impl<T> FindNextFlashcardForReviewUseCase<T> {
    pub fn new(repository: Box<dyn FlashcardsRepository<T>>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, deck_id: Uuid) -> Result<Option<Flashcard>, AppError> {
        let flashcard = self.repository.find_flashcard_for_review(deck_id).await?;

        Ok(flashcard)
    }
}
