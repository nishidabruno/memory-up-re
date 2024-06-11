use anyhow::Result;
use utils::AppError;
use uuid::Uuid;

use crate::{entities::Flashcard, repositories::FlashcardsRepository};

pub struct ReviewFlashcardUseCase<T> {
    repository: Box<dyn FlashcardsRepository<T>>,
}

impl<T> ReviewFlashcardUseCase<T> {
    pub fn new(repository: Box<dyn FlashcardsRepository<T>>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid, quality: i32) -> Result<(), AppError> {
        // TODO: should we really use as mutable?
        let mut binding = self.repository.find_by_id(id).await?;
        let flashcard = &mut binding;

        Flashcard::review(flashcard, quality);
        self.repository.update_flashcard(flashcard).await?;

        Ok(())
    }
}
