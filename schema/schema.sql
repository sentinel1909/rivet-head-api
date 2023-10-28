-- create the diary table

CREATE TABLE IF NOT EXISTS
diary (
  id uuid NOT NULL,
  PRIMARY KEY (id),
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ,
  band TEXT NOT NULL,
  album TEXT NOT NULL,
  thoughts TEXT NOT NULL
);