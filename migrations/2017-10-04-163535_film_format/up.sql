CREATE TABLE film_formats (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  designation VARCHAR NOT NULL UNIQUE,
  stock_size_value float,
  stock_size_unit VARCHAR
);

CREATE UNIQUE INDEX ON film_formats (designation);

INSERT INTO film_formats (designation, stock_size_value, stock_size_unit)
VALUES ('116', 70, 'mm'),
  ('120', 60.96, 'mm'),
  ('126', 35.0, 'mm'),
  ('127', 46.0, 'mm'),
  ('135', 35.0, 'mm'),
  ('220', 60.96, 'mm'),
  ('616', 70, 'mm'),
  ('620', 60.96, 'mm');
