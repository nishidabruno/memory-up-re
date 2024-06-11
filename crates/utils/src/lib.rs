use uuid::Uuid;

// TODO: Decide what returns to the user. Returning the real error may create security issues.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    InternalServerError(#[from] std::io::Error),

    #[error("{0}")]
    InternalError(String),

    #[error("Mutex lock failed")]
    MutexLockError,

    // #[error("Internal database error")]
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),

    #[error("Failed to execute embedded migrations")]
    DatabaseMigrationError(#[from] sqlx::migrate::MigrateError),

    #[error("Uuid bad format")]
    UuidBadFormat(#[from] uuid::Error),
}

// We need to serialize the error to a string to return it to the front-end.
impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

// TODO: implement serialize UUID from String instead of using a function to try_parse.
pub fn parse_str_uuid(deck_id: &str) -> Result<Uuid, AppError> {
    let deck_id = uuid::Uuid::try_parse(deck_id)?;

    Ok(deck_id)
}

// struct UniqueID([u8; 16]);
// impl From<uuid::Uuid> for UniqueID {
//     fn from(uuid: uuid::Uuid) -> Self {
//         todo!()
//     }
// }
