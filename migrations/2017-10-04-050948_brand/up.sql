CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE brands (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR NOT NULL
);

CREATE UNIQUE INDEX ON brands (name);

INSERT INTO brands (name)
VALUES ('Kodak'),
  ('Fuji'),
  ('Rollei'),
  ('Ilford');
