CREATE TABLE film_formats (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  designation VARCHAR NOT NULL UNIQUE,
  stock_size_value float,
  stock_size_unit VARCHAR,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX ON film_formats (designation);

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON film_formats
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

INSERT INTO film_formats (designation, stock_size_value, stock_size_unit)
VALUES ('116', 70, 'mm'),
  ('120', 60.96, 'mm'),
  ('126', 35.0, 'mm'),
  ('127', 46.0, 'mm'),
  ('135', 35.0, 'mm'),
  ('220', 60.96, 'mm'),
  ('616', 70, 'mm'),
  ('620', 60.96, 'mm');
