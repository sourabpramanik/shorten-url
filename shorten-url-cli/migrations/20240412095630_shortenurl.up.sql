-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS aliases (
  id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
  alias TEXT NOT NULL UNIQUE,
  url TEXT NOT NULL UNIQUE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  modified_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS alias_index ON aliases (alias);