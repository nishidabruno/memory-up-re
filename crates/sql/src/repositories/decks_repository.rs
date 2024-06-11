use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;
use sqlx::{query, Row};
use utils::AppError;
use uuid::Uuid;

use domain::entities::Deck;
use domain::repositories::DecksRepository;

pub struct DecksRepositorySql {
    pool: SqlitePool,
}

#[async_trait]
impl DecksRepository<SqlitePool> for DecksRepositorySql {
    fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    async fn save(&self, d: &Deck) -> Result<(), AppError> {
        query("INSERT INTO decks (id, title) VALUES (?1, ?2)")
            .bind(d.id)
            .bind(&d.title)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn list_all(&self) -> Result<Vec<Deck>, AppError> {
        let decks: Vec<Deck> = sqlx::query_as("SELECT * FROM decks")
            .fetch_all(&self.pool)
            .await?;

        Ok(decks)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Deck, AppError> {
        let row = query("SELECT * FROM decks WHERE id = ?1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        let deck = Deck {
            id: row.get("id"),
            title: row.get("title"),
        };

        Ok(deck)
    }
}
