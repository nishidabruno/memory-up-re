use anyhow::Result;
use utils::AppError;
use uuid::Uuid;

use crate::{entities::Review, repositories::ReviewsRepository};

pub struct CreateReviewUseCase<T> {
    repository: Box<dyn ReviewsRepository<T>>,
}

impl<T> CreateReviewUseCase<T> {
    pub fn new(repository: Box<dyn ReviewsRepository<T>>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        deck_id: Uuid,
        flashcard_id: Uuid,
        quality: i32,
    ) -> Result<(), AppError> {
        let review = Review {
            id: Uuid::new_v4(),
            deck_id,
            flashcard_id,
            quality,
        };

        self.repository.save(review).await?;

        Ok(())
    }
}
