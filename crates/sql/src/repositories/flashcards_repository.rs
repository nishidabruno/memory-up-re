use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;
use sqlx::{query, Row};
use utils::AppError;
use uuid::Uuid;

use domain::entities::Flashcard;
use domain::repositories::FlashcardsRepository;

pub struct FlashcardsRepositorySql {
    pool: SqlitePool,
}

#[async_trait]
impl FlashcardsRepository<SqlitePool> for FlashcardsRepositorySql {
    fn new(pool: SqlitePool) -> Self {
        Self { pool: pool.clone() }
    }

    async fn save(&self, f: Flashcard) -> Result<(), AppError> {
        query("INSERT INTO flashcards (id, deck_id, front, back, ease_factor, interval, repetitions, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, datetime('now'), datetime('now'))")
            .bind(f.id)
            .bind(f.deck_id)
            .bind(f.front)
            .bind(f.back)
            .bind(f.ease_factor)
            .bind(f.interval)
            .bind(f.repetitions)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Flashcard, AppError> {
        let row = query("SELECT * FROM flashcards WHERE id = ?1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        let flashcard = Flashcard {
            id: row.get("id"),
            deck_id: row.get("deck_id"),
            front: row.get("front"),
            back: row.get("back"),
            ease_factor: row.get("ease_factor"),
            interval: row.get("interval"),
            repetitions: row.get("repetitions"),
        };

        Ok(flashcard)
    }

    async fn find_flashcard_by_deck_id(&self, deck_id: Uuid) -> Result<Vec<Flashcard>, AppError> {
        // TODO: use fetch_many instead?
        let rows = query("SELECT * FROM flashcards WHERE deck_id = ?1")
            .bind(deck_id)
            .fetch_all(&self.pool)
            .await?;

        let flashcards = rows
            .iter()
            .map(|row| Flashcard {
                id: row.get("id"),
                front: row.get("front"),
                back: row.get("back"),
                interval: row.get("interval"),
                ease_factor: row.get("ease_factor"),
                repetitions: row.get("repetitions"),
                deck_id: row.get("deck_id"),
            })
            .collect();

        Ok(flashcards)
    }

    async fn update_flashcard(&self, f: &mut Flashcard) -> Result<(), AppError> {
        query("UPDATE flashcards SET interval=?1, ease_factor=?2, repetitions=?3, front=?4, back=?5, updated_at=datetime('now') WHERE id=?6")
            .bind(f.interval)
            .bind(f.ease_factor)
            .bind(f.repetitions)
            .bind(f.front.clone())
            .bind(f.back.clone())
            .bind(f.id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_unique_flashcard_by_quality(
        &self,
        deck_id: Uuid,
        quality: i8,
    ) -> Result<Flashcard, AppError> {
        let row = query("SELECT * FROM flashcards WHERE deck_id = ?1 AND quality = 4 LIMIT 1")
            .bind(deck_id)
            .bind(quality)
            .fetch_one(&self.pool)
            .await?;

        let flashcard = Flashcard {
            id: row.get("id"),
            deck_id: row.get("deck_id"),
            front: row.get("front"),
            back: row.get("back"),
            ease_factor: row.get("ease_factor"),
            interval: row.get("interval"),
            repetitions: row.get("repetitions"),
        };

        Ok(flashcard)
    }

    async fn find_flashcard_for_review(
        &self,
        deck_id: Uuid,
    ) -> Result<Option<Flashcard>, AppError> {
        // SELECT * FROM flashcards WHERE deck_id = ?1 AND DATE('now') >= DATE(updated_at, '+' || interval || ' days') ORDER BY RANDOM() LIMIT 1
        // dia now: 30, last_updated: 10+interval(1) = 11
        let row = query("SELECT * FROM flashcards WHERE deck_id = ?1 AND DATE('now') >= DATE(updated_at, '+' || interval || ' days') ORDER BY RANDOM() LIMIT 1")
            .bind(deck_id)
            .fetch_one(&self.pool)
            .await?;

        let flashcard = Flashcard {
            id: row.get("id"),
            deck_id: row.get("deck_id"),
            front: row.get("front"),
            back: row.get("back"),
            ease_factor: row.get("ease_factor"),
            interval: row.get("interval"),
            repetitions: row.get("repetitions"),
        };

        Ok(Some(flashcard))
    }

    async fn get_total_flashcards_by_deck(&self, deck_id: Uuid) -> Result<i32, AppError> {
        let row = query(
            r#"
            SELECT 
            COUNT(*) AS num_flashcards
            FROM 
            flashcards
            WHERE 
            deck_id = ?
        "#,
        )
        .bind(deck_id)
        .fetch_one(&self.pool)
        .await?;

        let num_flashcards = row.get("num_flashcards");

        Ok(num_flashcards)
    }

    async fn find_flashcards_left_for_review(&self, deck_id: Uuid) -> Result<i32, AppError> {
        let row = query(
            r#"
        SELECT 
          COUNT(*) AS left_for_review_count
        FROM 
          flashcards
        WHERE 
          deck_id = ?
          AND DATE('now') >= DATE(updated_at, '+' || interval || ' days')
    "#,
        )
        .bind(deck_id)
        .fetch_one(&self.pool)
        .await?;

        let num_flashcards = row.get("left_for_review_count");

        Ok(num_flashcards)
    }
}
