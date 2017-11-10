CREATE TABLE cameras (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  model VARCHAR NOT NULL,
  brand_id UUID REFERENCES brands NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON cameras
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

CREATE INDEX ON cameras (model);
CREATE INDEX ON cameras (brand_id);

INSERT INTO cameras (
  model,
  brand_id
)
VALUES
(
  'SR-T 101',
  (SELECT id FROM brands WHERE name = 'Minolta')
), (
  'XE-1',
  (SELECT id FROM brands WHERE name = 'Minolta')
), (
  'XD-7',
  (SELECT id FROM brands WHERE name = 'Minolta')
), (
  'XD-11',
  (SELECT id FROM brands WHERE name = 'Minolta')
), (
  'Hi-Matic E',
  (SELECT id FROM brands WHERE name = 'Minolta')
), (
  'Hi-Matic F',
  (SELECT id FROM brands WHERE name = 'Minolta')
), (
  'FE',
  (SELECT id FROM brands WHERE name = 'Nikon')
), (
  '35Ti',
  (SELECT id FROM brands WHERE name = 'Nikon')
), (
  'EM',
  (SELECT id FROM brands WHERE name = 'Nikon')
), (
  'N90s',
  (SELECT id FROM brands WHERE name = 'Nikon')
), (
  'XF35',
  (SELECT id FROM brands WHERE name = 'Rollei')
), (
  'K1000',
  (SELECT id FROM brands WHERE name = 'Pentax')
), (
  'OM-1n',
  (SELECT id FROM brands WHERE name = 'Olympus')
), (
  'Stylus Epic DLX',
  (SELECT id FROM brands WHERE name = 'Olympus')
), (
  'S',
  (SELECT id FROM brands WHERE name = 'Zorki')
), (
  '35 GSN',
  (SELECT id FROM brands WHERE name = 'Yashica')
), (
  'Chevron',
  (SELECT id FROM brands WHERE name = 'Kodak')
), (
  'Brownie No. 2',
  (SELECT id FROM brands WHERE name = 'Kodak')
), (
  'Flexaret III',
  (SELECT id FROM brands WHERE name = 'Meopta')
);
