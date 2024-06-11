use async_trait::async_trait;
use utils::AppError;
use uuid::Uuid;

use crate::entities::{MonthlyReviewStats, QualityWiseReview, Review};

#[async_trait]
pub trait ReviewsRepository<T>: Send + Sync {
    fn new(pool: T) -> Self
    where
        T: Send + Sync,
        Self: Sized;
    async fn save(&self, review: Review) -> Result<(), AppError>;
    async fn find_reviews_by_deck_id(&self, deck_id: Uuid) -> Result<Vec<Review>, AppError>;
    async fn get_total_reviews_by_month(&self) -> Result<Vec<MonthlyReviewStats>, AppError>;
    async fn get_total_reviews_by_quality(&self) -> Result<Vec<QualityWiseReview>, AppError>;
    async fn get_correct_percentage(&self, deck_id: Uuid) -> Result<f32, AppError>;
}
