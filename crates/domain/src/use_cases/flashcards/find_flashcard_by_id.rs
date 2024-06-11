use anyhow::Result;
use utils::AppError;
use uuid::Uuid;

use crate::{entities::Flashcard, repositories::FlashcardsRepository};

pub struct FindFlashcardByIdUseCase<T> {
    repository: Box<dyn FlashcardsRepository<T>>,
}

impl<T> FindFlashcardByIdUseCase<T> {
    pub fn new(repository: Box<dyn FlashcardsRepository<T>>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid) -> Result<Flashcard, AppError> {
        let flashcard = self.repository.find_by_id(id).await?;

        Ok(flashcard)
    }
}
