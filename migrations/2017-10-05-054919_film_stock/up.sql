CREATE TABLE film_stocks (
   id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
   box_name VARCHAR NOT NULL,
   box_speed integer,
   brand_id UUID REFERENCES brands NOT NULL,
   film_format_id UUID REFERENCES film_formats NOT NULL
);

CREATE INDEX ON film_stocks (box_name);
CREATE INDEX ON film_stocks (box_speed);
CREATE INDEX ON film_stocks (brand_id);
CREATE INDEX ON film_stocks (film_format_id);

INSERT INTO film_stocks (box_name, box_speed, brand_id, film_format_id)
VALUES
  (
    'Portra',
    400,
    (SELECT id FROM brands WHERE name = 'Kodak'),
    (SELECT id from film_formats WHERE designation = '135')
  ), (
    'Portra',
    800,
    (SELECT id FROM brands WHERE name = 'Kodak'),
    (SELECT id from film_formats WHERE designation = '135')
  ), (
    'Tri-X',
    400,
    (SELECT id FROM brands WHERE name = 'Kodak'),
    (SELECT id from film_formats WHERE designation = '135')
  ), (
    'T-MAX',
    400,
    (SELECT id FROM brands WHERE name = 'Kodak'),
    (SELECT id from film_formats WHERE designation = '135')
  ), (
    'Gold',
    400,
    (SELECT id FROM brands WHERE name = 'Kodak'),
    (SELECT id from film_formats WHERE designation = '135')
  ), (
    'Ektar',
    100,
    (SELECT id FROM brands WHERE name = 'Kodak'),
    (SELECT id from film_formats WHERE designation = '135')
  ), (
    'Delta 3200',
    3200,
    (SELECT id FROM brands WHERE name = 'Ilford'),
    (SELECT id from film_formats WHERE designation = '135')
  ), (
    'Delta 3200',
    3200,
    (SELECT id FROM brands WHERE name = 'Ilford'),
    (SELECT id from film_formats WHERE designation = '120')
  ), (
    'HP5 Plus',
    400,
    (SELECT id FROM brands WHERE name = 'Ilford'),
    (SELECT id from film_formats WHERE designation = '135')
  ), (
    'FP4 Plus',
    125,
    (SELECT id FROM brands WHERE name = 'Ilford'),
    (SELECT id from film_formats WHERE designation = '135')
  );
