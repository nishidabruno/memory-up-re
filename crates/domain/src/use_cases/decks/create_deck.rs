use crate::entities::Deck;
use crate::repositories::DecksRepository;
use utils::AppError;

pub struct CreateDeckUseCase<T> {
    repository: Box<dyn DecksRepository<T>>,
}

impl<T> CreateDeckUseCase<T> {
    pub fn new(repository: Box<dyn DecksRepository<T>>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, title: String) -> Result<(), AppError> {
        let deck = Deck::new(title);

        self.repository.save(&deck).await?;

        Ok(())
    }
}
