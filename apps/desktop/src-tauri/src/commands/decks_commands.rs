use crate::utils::AppState;
use tauri::State;
use utils::{parse_str_uuid, AppError};

use domain::entities::Deck;
use domain::repositories::DecksRepository;
use domain::use_cases::decks::CreateDeckUseCase;
use domain::use_cases::decks::FindAllDecksUseCase;
use domain::use_cases::decks::FindDeckByIdUseCase;
use sql::DecksRepositorySql;

#[tauri::command]
pub async fn create_deck(title: String, state: State<'_, AppState>) -> Result<bool, AppError> {
    let decks_repository = DecksRepositorySql::new(state.pool.clone());
    let create_deck_use_case = CreateDeckUseCase::new(Box::new(decks_repository));
    create_deck_use_case.execute(title).await?;

    Ok(state.pool.clone().is_closed())
}

#[tauri::command]
pub async fn find_all_decks(state: State<'_, AppState>) -> Result<Vec<Deck>, AppError> {
    let decks_repository = DecksRepositorySql::new(state.pool.clone());
    let find_all_decks_use_case = FindAllDecksUseCase::new(Box::new(decks_repository));
    let decks = find_all_decks_use_case.execute().await?;

    Ok(decks)
}

#[tauri::command]
pub async fn find_deck_by_id(id: String, state: State<'_, AppState>) -> Result<Deck, AppError> {
    let id = parse_str_uuid(&id)?;
    let decks_repository = DecksRepositorySql::new(state.pool.clone());
    let find_deck_by_id_use_case = FindDeckByIdUseCase::new(Box::new(decks_repository));
    let deck = find_deck_by_id_use_case.execute(id).await?;

    Ok(deck)
}
