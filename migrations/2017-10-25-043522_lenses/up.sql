CREATE TABLE lenses (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  model VARCHAR NOT NULL,
  focal_length_min_value float NOT NULL,
  focal_length_min_unit VARCHAR NOT NULL,
  focal_length_max_value float,
  focal_length_max_unit VARCHAR,
  aperture_max float NOT NULL,
  aperture_min float,
  element_count integer,
  group_count integer,
  filter_thread_diameter_value float,
  filter_thread_diameter_unit VARCHAR,
  notes VARCHAR,
  brand_id UUID REFERENCES brands NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX ON lenses (model);
CREATE INDEX ON lenses (focal_length_min_value);
CREATE INDEX ON lenses (aperture_max);
CREATE INDEX ON lenses (brand_id);

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON lenses
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

INSERT INTO lenses (
  model,
  focal_length_min_value, focal_length_min_unit,
  aperture_min, aperture_max,
  element_count, group_count,
  filter_thread_diameter_value, filter_thread_diameter_unit,
  brand_id
)
VALUES
(
  'MD W.ROKKOR',
  28.0, 'mm',
  2.8, 22.0,
  7, 7,
  49.0, 'mm',
  (SELECT id FROM brands WHERE name = 'Minolta')
  ), (
  'MC ROKKOR-PF',
  55.0, 'mm',
  1.7, 16.0,
  6, 5,
  52.0, 'mm',
  (SELECT id FROM brands WHERE name = 'Minolta')
  ), (
  'MD ROKKOR',
  50.0, 'mm',
  1.4, 16.0,
  7, 5,
  55.0, 'mm',
  (SELECT id FROM brands WHERE name = 'Minolta')
  ), (
  'MD ROKKOR',
  50.0, 'mm',
  1.7, 16.0,
  6, 5,
  49.0, 'mm',
  (SELECT id FROM brands WHERE name = 'Minolta')
  ), (
  'MC MACRO ROKKOR-QF',
  50.0, 'mm',
  3.5, 22.0,
  6, 4,
  55.0, 'mm',
  (SELECT id FROM brands WHERE name = 'Minolta')
  ), (
  'MD TELE ROKKOR-X',
  135.0, 'mm',
  2.8, 22.0,
  4, 4,
  55.0, 'mm',
  (SELECT id FROM brands WHERE name = 'Minolta')
  ), (
  'MC TELE ROKKOR-HF',
  30.0, 'cm',
  4.5, 22.0,
  6, 6,
  72.0, 'mm',
  (SELECT id FROM brands WHERE name = 'Minolta')
)
