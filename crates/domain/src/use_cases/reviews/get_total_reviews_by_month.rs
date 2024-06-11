use anyhow::Result;
use utils::AppError;

use crate::{entities::MonthlyReviewStats, repositories::ReviewsRepository};

pub struct GetTotalReviewsByMonthUseCase<T> {
    repository: Box<dyn ReviewsRepository<T>>,
}

impl<T> GetTotalReviewsByMonthUseCase<T> {
    pub fn new(repository: Box<dyn ReviewsRepository<T>>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> Result<Vec<MonthlyReviewStats>, AppError> {
        let total_reviews_by_month = self.repository.get_total_reviews_by_month().await?;

        Ok(total_reviews_by_month)
    }
}
