use anyhow::Result;
use utils::AppError;
use uuid::Uuid;

use crate::{entities::Review, repositories::ReviewsRepository};

pub struct FindReviewByDeckIdUseCase<T> {
    repository: Box<dyn ReviewsRepository<T>>,
}

impl<T> FindReviewByDeckIdUseCase<T> {
    pub fn new(repository: Box<dyn ReviewsRepository<T>>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, deck_id: Uuid) -> Result<Vec<Review>, AppError> {
        let reviews_by_deck = self.repository.find_reviews_by_deck_id(deck_id).await?;

        Ok(reviews_by_deck)
    }
}
