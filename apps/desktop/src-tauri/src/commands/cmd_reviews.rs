use crate::utils::AppState;
use domain::entities::{MonthlyReviewStats, QualityWiseReview, Review};
use domain::repositories::ReviewsRepository;
use domain::use_cases::reviews::{
    FindReviewByDeckIdUseCase, GetCorrectPercentageUseCase, GetTotalReviewsByMonthUseCase,
    GetTotalReviewsByQualityUseCase,
};
use sql::ReviewsRepositorySql;
use tauri::State;
use utils::{parse_str_uuid, AppError};

#[tauri::command]
pub async fn find_reviews_by_deck_id(
    deck_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<Review>, AppError> {
    let deck_id = parse_str_uuid(&deck_id)?;
    let reviews_repository = ReviewsRepositorySql::new(state.pool.clone());
    let reviews_by_deck_id_usecase = FindReviewByDeckIdUseCase::new(Box::new(reviews_repository));
    let reviews_by_deck_id = reviews_by_deck_id_usecase.execute(deck_id).await?;

    Ok(reviews_by_deck_id)
}

#[tauri::command]
pub async fn get_total_reviews_by_month(
    state: State<'_, AppState>,
) -> Result<Vec<MonthlyReviewStats>, AppError> {
    let reviews_repository = ReviewsRepositorySql::new(state.pool.clone());
    let get_total_reviews = GetTotalReviewsByMonthUseCase::new(Box::new(reviews_repository));
    let reviews = get_total_reviews.execute().await?;

    Ok(reviews)
}

#[tauri::command]
pub async fn get_total_reviews_by_quality(
    state: State<'_, AppState>,
) -> Result<Vec<QualityWiseReview>, AppError> {
    let reviews_repository = ReviewsRepositorySql::new(state.pool.clone());
    let get_total_reviews_by_quality_usecase =
        GetTotalReviewsByQualityUseCase::new(Box::new(reviews_repository));
    let total_reviews_by_quality = get_total_reviews_by_quality_usecase.execute().await?;

    Ok(total_reviews_by_quality)
}

#[tauri::command]
pub async fn get_correct_percentage(
    deck_id: String,
    state: State<'_, AppState>,
) -> Result<f32, AppError> {
    let deck_id = parse_str_uuid(&deck_id)?;
    let reviews_repository = ReviewsRepositorySql::new(state.pool.clone());
    let get_correct_percentage_usecase =
        GetCorrectPercentageUseCase::new(Box::new(reviews_repository));
    let correct_percentage = get_correct_percentage_usecase.execute(deck_id).await?;

    Ok(correct_percentage)
}
