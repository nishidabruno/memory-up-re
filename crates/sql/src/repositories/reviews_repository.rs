use async_trait::async_trait;
use domain::entities::{MonthlyReviewStats, QualityWiseReview, Review};
use sqlx::sqlite::SqlitePool;
use sqlx::{query, Row};
use utils::AppError;
use uuid::Uuid;

use domain::repositories::ReviewsRepository;

pub struct ReviewsRepositorySql {
    pool: SqlitePool,
}

#[async_trait]
impl ReviewsRepository<SqlitePool> for ReviewsRepositorySql {
    fn new(pool: SqlitePool) -> Self {
        Self { pool: pool.clone() }
    }

    async fn save(&self, r: Review) -> Result<(), AppError> {
        query("INSERT INTO reviews (id, deck_id, flashcard_id, quality, created_at) VALUES (?1, ?2, ?3, ?4, datetime('now'))")
          .bind(r.id)
          .bind(r.deck_id)
          .bind(r.flashcard_id)
          .bind(r.quality)
          .execute(&self.pool)
          .await?;

        Ok(())
    }

    async fn find_reviews_by_deck_id(&self, deck_id: Uuid) -> Result<Vec<Review>, AppError> {
        // TODO: use fetch_many instead?
        let rows = query("SELECT * FROM reviews WHERE deck_id = ?1")
            .bind(deck_id)
            .fetch_all(&self.pool)
            .await?;

        let reviews = rows
            .iter()
            .map(|row| Review {
                id: row.get("id"),
                deck_id: row.get("deck_id"),
                flashcard_id: row.get("flashcard_id"),
                quality: row.get("quality"),
            })
            .collect();

        Ok(reviews)
    }

    async fn get_total_reviews_by_month(&self) -> Result<Vec<MonthlyReviewStats>, AppError> {
        // TODO: use fetch_many instead?
        let rows = query(
            r#"
            SELECT 
            strftime('%Y-%m', created_at) as month,
            COUNT(*) as review_count
        FROM reviews
        GROUP BY month
        ORDER BY month;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let stats: Vec<MonthlyReviewStats> = rows
            .iter()
            .map(|row| MonthlyReviewStats {
                month: row.get("month"),
                count: row.get("review_count"),
            })
            .collect();

        Ok(stats)
    }

    async fn get_total_reviews_by_quality(&self) -> Result<Vec<QualityWiseReview>, AppError> {
        // TODO: use fetch_many instead?
        let row = query(
            r#"
            SELECT
                SUM(CASE WHEN quality = 0 THEN 1 ELSE 0 END) AS dont_remember,
                SUM(CASE WHEN quality = 3 THEN 1 ELSE 0 END) AS hard,
                SUM(CASE WHEN quality = 4 THEN 1 ELSE 0 END) AS medium,
                SUM(CASE WHEN quality = 5 THEN 1 ELSE 0 END) AS easy
            FROM
                reviews;
          "#,
        )
        .fetch_one(&self.pool)
        .await?;

        let dont_remember = row.get("dont_remember");
        let hard = row.get("hard");
        let medium = row.get("medium");
        let easy = row.get("easy");

        let reviews = vec![
            QualityWiseReview {
                quality: "Não lembro".to_string(),
                count: dont_remember,
            },
            QualityWiseReview {
                quality: "Difícil".to_string(),
                count: hard,
            },
            QualityWiseReview {
                quality: "Médio".to_string(),
                count: medium,
            },
            QualityWiseReview {
                quality: "Fácil".to_string(),
                count: easy,
            },
        ];

        Ok(reviews)
    }

    async fn get_correct_percentage(&self, deck_id: Uuid) -> Result<f32, AppError> {
        // TODO: use fetch_many instead?
        let row = query(
            r#"
            SELECT 
            deck_id,
            (CAST(SUM(CASE WHEN quality IN (4, 5) THEN 1 ELSE 0 END) AS FLOAT) / COUNT(*)) * 100 AS correct_percentage
          FROM 
            reviews
          WHERE
            deck_id = ?1
          
          "#,
        )
        .bind(deck_id)
        .fetch_one(&self.pool)
        .await?;

        let correct_percentage = row.get("correct_percentage");

        Ok(correct_percentage)
    }
}
