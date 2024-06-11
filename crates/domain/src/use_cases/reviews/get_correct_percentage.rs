use anyhow::Result;
use utils::AppError;
use uuid::Uuid;

use crate::repositories::ReviewsRepository;

pub struct GetCorrectPercentageUseCase<T> {
    repository: Box<dyn ReviewsRepository<T>>,
}

impl<T> GetCorrectPercentageUseCase<T> {
    pub fn new(repository: Box<dyn ReviewsRepository<T>>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, deck_id: Uuid) -> Result<f32, AppError> {
        let correct_percentage = self.repository.get_correct_percentage(deck_id).await?;

        Ok(correct_percentage)
    }
}
