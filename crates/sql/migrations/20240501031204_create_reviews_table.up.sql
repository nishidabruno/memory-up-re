CREATE TABLE reviews (
  id TEXT NOT NULL PRIMARY KEY,
  deck_id TEXT NOT NULL,
  flashcard_id TEXT NOT NULL,
  quality INTEGER,
  created_at TEXT,
  FOREIGN KEY(deck_id) REFERENCES decks(id)
  FOREIGN KEY(flashcard_id) REFERENCES flashcards(id)
)