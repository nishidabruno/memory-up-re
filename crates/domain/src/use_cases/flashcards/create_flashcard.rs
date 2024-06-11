use anyhow::Result;
use utils::AppError;
use uuid::Uuid;

use crate::entities::Flashcard;
use crate::repositories::FlashcardsRepository;

pub struct FlashcardInputDTO {
    pub deck_id: Uuid,
    pub front: String,
    pub back: String,
}

pub struct CreateFlashcardUseCase<T> {
    repository: Box<dyn FlashcardsRepository<T>>,
}

impl<T> CreateFlashcardUseCase<T> {
    pub fn new(repository: Box<dyn FlashcardsRepository<T>>) -> Self {
        CreateFlashcardUseCase { repository }
    }

    pub async fn execute(&self, input: FlashcardInputDTO) -> Result<(), AppError> {
        let flashcard = Flashcard::new(input.front, input.back, input.deck_id);
        self.repository.save(flashcard).await?;

        Ok(())
    }
}
