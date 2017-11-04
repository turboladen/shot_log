CREATE TABLE brands (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX ON brands (name);

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON brands
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

INSERT INTO brands (name)
VALUES ('Kodak'),
  ('Fuji'),
  ('Rollei'),
  ('Ilford'),
  ('Minolta'),
  ('Olympus'),
  ('Pentax'),
  ('Yashica'),
  ('Meopta')
  ;
