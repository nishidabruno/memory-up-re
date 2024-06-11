CREATE TABLE flashcards (
  id TEXT NOT NULL PRIMARY KEY, 
  front TEXT NOT NULL,
  back TEXT NOT NULL,
  interval INTEGER,
  ease_factor REAL,
  repetitions INTEGER,
  deck_id TEXT NOT NULL,
  created_at TEXT,
  updated_at TEXT,
  FOREIGN KEY(deck_id) REFERENCES decks(id)
)