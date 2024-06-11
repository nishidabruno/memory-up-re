use anyhow::Result;
use utils::AppError;
use uuid::Uuid;

use crate::repositories::FlashcardsRepository;

pub struct FindFlashcardsLeftForReviewUseCase<T> {
    repository: Box<dyn FlashcardsRepository<T>>,
}

impl<T> FindFlashcardsLeftForReviewUseCase<T> {
    pub fn new(repository: Box<dyn FlashcardsRepository<T>>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid) -> Result<i32, AppError> {
        let flashcard_count = self.repository.find_flashcards_left_for_review(id).await?;

        Ok(flashcard_count)
    }
}
