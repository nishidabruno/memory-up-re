use anyhow::Result;
use utils::AppError;

use crate::entities::Flashcard;
use crate::repositories::FlashcardsRepository;

pub struct UpdateFlashcardUseCase<T> {
    repository: Box<dyn FlashcardsRepository<T>>,
}

impl<T> UpdateFlashcardUseCase<T> {
    pub fn new(repository: Box<dyn FlashcardsRepository<T>>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        id: uuid::Uuid,
        front: String,
        back: String,
    ) -> Result<(), AppError> {
        // TODO: should we really use as mutable?
        let binding = self.repository.find_by_id(id).await?;
        let mut flashcard = Flashcard {
            id: binding.id,
            front,
            back,
            deck_id: binding.deck_id,
            ease_factor: binding.ease_factor,
            interval: binding.interval,
            repetitions: binding.repetitions,
        };

        self.repository.update_flashcard(&mut flashcard).await?;

        Ok(())
    }
}
