use anyhow::Result;
use utils::AppError;

use crate::{entities::QualityWiseReview, repositories::ReviewsRepository};

pub struct GetTotalReviewsByQualityUseCase<T> {
    repository: Box<dyn ReviewsRepository<T>>,
}

impl<T> GetTotalReviewsByQualityUseCase<T> {
    pub fn new(repository: Box<dyn ReviewsRepository<T>>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> Result<Vec<QualityWiseReview>, AppError> {
        let total_reviews_by_quality = self.repository.get_total_reviews_by_quality().await?;

        Ok(total_reviews_by_quality)
    }
}
