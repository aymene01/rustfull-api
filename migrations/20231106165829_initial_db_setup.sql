-- Add migration script here
CREATE TABLE IF NOT EXISTS quotes (
  id UUID PRIMARY KEY,
  author varchar NOT NULL,
  quote TEXT NOT NULL,
  inserted_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL,
  UNIQUE (author, quote)
);