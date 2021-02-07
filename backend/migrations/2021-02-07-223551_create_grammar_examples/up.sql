CREATE TABLE grammars (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  meaning_en TEXT NOT NULL
);

CREATE TABLE examples (
  id SERIAL PRIMARY KEY,
  english TEXT,
  korean TEXT,
  grammar_id INT
);
