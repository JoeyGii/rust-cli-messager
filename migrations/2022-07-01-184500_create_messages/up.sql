CREATE TABLE message (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
)