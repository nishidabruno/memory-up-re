use async_trait::async_trait;
use utils::AppError;
use uuid::Uuid;

use crate::entities::Flashcard;

#[async_trait]
pub trait FlashcardsRepository<T>: Send + Sync {
    fn new(pool: T) -> Self
    where
        T: Send + Sync,
        Self: Sized;
    async fn save(&self, flashcard: Flashcard) -> Result<(), AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Flashcard, AppError>;
    async fn find_flashcard_by_deck_id(&self, deck_id: Uuid) -> Result<Vec<Flashcard>, AppError>;
    async fn update_flashcard(&self, flashcard: &mut Flashcard) -> Result<(), AppError>;
    async fn find_flashcard_for_review(&self, deck_id: Uuid)
        -> Result<Option<Flashcard>, AppError>;
    async fn get_unique_flashcard_by_quality(
        &self,
        deck_id: Uuid,
        quality: i8,
    ) -> Result<Flashcard, AppError>;
    async fn get_total_flashcards_by_deck(&self, deck_id: Uuid) -> Result<i32, AppError>;
    async fn find_flashcards_left_for_review(&self, deck_id: Uuid) -> Result<i32, AppError>;
}
