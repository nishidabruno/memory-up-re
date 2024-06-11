use std::{env, fs};

use sqlx::{migrate, SqlitePool};
use utils::AppError;

pub struct DbManager {
    pool: SqlitePool,
}

impl DbManager {
    pub async fn new() -> Result<Self, AppError> {
        let curr_exe = env::current_exe().expect("Failed to get current executable path");
        let current_dir = curr_exe.parent().expect("Failed to get parent directory");

        let db_path = current_dir.join("memup.db");

        if !db_path.exists() {
            fs::File::create(&db_path).expect("Failed to create database file.");
        }

        let pool = SqlitePool::connect(db_path.to_str().unwrap())
            .await
            .expect("Error connecting to pool");

        migrate!()
            .run(&pool)
            .await
            .expect("Error while running migrations");

        Ok(Self { pool })
    }

    pub fn get_db_instance(&self) -> SqlitePool {
        self.pool.clone()
    }
}
