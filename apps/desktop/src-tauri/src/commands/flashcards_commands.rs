use domain::entities::Flashcard;
use domain::repositories::FlashcardsRepository;
use domain::repositories::ReviewsRepository;
use domain::use_cases::flashcards::CreateFlashcardUseCase;
use domain::use_cases::flashcards::FindFlashcardByIdUseCase;
use domain::use_cases::flashcards::FindFlashcardsByDeckIdUseCase;
use domain::use_cases::flashcards::FindFlashcardsLeftForReviewUseCase;
use domain::use_cases::flashcards::FindNextFlashcardForReviewUseCase;
use domain::use_cases::flashcards::FlashcardInputDTO;
use domain::use_cases::flashcards::GetTotalFlashcardsByDeckUseCase;
use domain::use_cases::flashcards::ReviewFlashcardUseCase;
use domain::use_cases::flashcards::UpdateFlashcardUseCase;
use domain::use_cases::reviews::CreateReviewUseCase;
use sql::FlashcardsRepositorySql;
use sql::ReviewsRepositorySql;
use tauri::State;
use utils::AppError;

use crate::utils::AppState;
use utils::parse_str_uuid;

#[tauri::command]
pub async fn create_flashcard(
    front: String,
    back: String,
    deck_id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let deck_id = parse_str_uuid(&deck_id)?;
    let input_dto = FlashcardInputDTO {
        front,
        back,
        deck_id,
    };
    let flashcards_repository = FlashcardsRepositorySql::new(state.pool.clone());
    let create_flashcard_use_case = CreateFlashcardUseCase::new(Box::new(flashcards_repository));

    create_flashcard_use_case.execute(input_dto).await?;

    Ok(())
}

#[tauri::command]
pub async fn update_flashcard(
    id: String,
    front: String,
    back: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let id = parse_str_uuid(&id)?;
    let flashcards_repository = FlashcardsRepositorySql::new(state.pool.clone());
    let update_flashcard_use_case = UpdateFlashcardUseCase::new(Box::new(flashcards_repository));

    update_flashcard_use_case.execute(id, front, back).await?;

    Ok(())
}

#[tauri::command]
pub async fn find_flashcard_by_id(
    id: String,
    state: State<'_, AppState>,
) -> Result<Flashcard, AppError> {
    let flashcard_id = parse_str_uuid(&id)?;
    let flashcards_repository = FlashcardsRepositorySql::new(state.pool.clone());
    let find_flashcard_by_id_use_case =
        FindFlashcardByIdUseCase::new(Box::new(flashcards_repository));

    let flashcard = find_flashcard_by_id_use_case.execute(flashcard_id).await?;

    Ok(flashcard)
}

#[tauri::command]
pub async fn find_flashcards_by_deck_id(
    deck_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<Flashcard>, AppError> {
    let deck_id = parse_str_uuid(&deck_id)?;
    let flashcards_repository = FlashcardsRepositorySql::new(state.pool.clone());
    let find_flashcards_deck_id_use_case =
        FindFlashcardsByDeckIdUseCase::new(Box::new(flashcards_repository));

    let flashcards = find_flashcards_deck_id_use_case.execute(deck_id).await?;

    Ok(flashcards)
}

#[tauri::command]
pub async fn find_flashcard_for_review(
    deck_id: String,
    state: State<'_, AppState>,
) -> Result<Flashcard, AppError> {
    let deck_id = parse_str_uuid(&deck_id)?;
    let flashcards_repository = FlashcardsRepositorySql::new(state.pool.clone());
    let find_flashcard_for_review_use_case =
        FindNextFlashcardForReviewUseCase::new(Box::new(flashcards_repository));

    let flashcard = find_flashcard_for_review_use_case.execute(deck_id).await?;

    Ok(flashcard.unwrap())
}

#[tauri::command]
pub async fn review_flashcard(
    deck_id: String,
    flashcard_id: String,
    quality: i32,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let flashcard_id = parse_str_uuid(&flashcard_id)?;
    let flashcards_repository = FlashcardsRepositorySql::new(state.pool.clone());
    let review_flashcard_use_case = ReviewFlashcardUseCase::new(Box::new(flashcards_repository));

    review_flashcard_use_case
        .execute(flashcard_id, quality)
        .await?;

    // register review
    let deck_id = parse_str_uuid(&deck_id)?;
    let reviews_repository = ReviewsRepositorySql::new(state.pool.clone());
    let create_review = CreateReviewUseCase::new(Box::new(reviews_repository));
    create_review
        .execute(deck_id, flashcard_id, quality)
        .await?;

    Ok(())
}

#[tauri::command]
pub async fn get_total_flashcards_by_deck(
    deck_id: String,
    state: State<'_, AppState>,
) -> Result<i32, AppError> {
    let deck_id = parse_str_uuid(&deck_id)?;
    let flashcards_repository = FlashcardsRepositorySql::new(state.pool.clone());
    let get_total_flashcards_by_deck_usecase =
        GetTotalFlashcardsByDeckUseCase::new(Box::new(flashcards_repository));

    let num_flashcards = get_total_flashcards_by_deck_usecase
        .execute(deck_id)
        .await?;

    Ok(num_flashcards)
}

#[tauri::command]
pub async fn find_flashcards_left_for_review(
    deck_id: String,
    state: State<'_, AppState>,
) -> Result<i32, AppError> {
    let deck_id = parse_str_uuid(&deck_id)?;
    let flashcards_repository = FlashcardsRepositorySql::new(state.pool.clone());
    let find_flashcards_left_for_review_usecase =
        FindFlashcardsLeftForReviewUseCase::new(Box::new(flashcards_repository));

    let num_flashcards = find_flashcards_left_for_review_usecase
        .execute(deck_id)
        .await?;

    Ok(num_flashcards)
}
