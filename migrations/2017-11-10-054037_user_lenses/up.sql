CREATE TABLE user_lenses (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES users NOT NULL,
  lens_id UUID REFERENCES lenses NOT NULL,
  serial_number VARCHAR,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON user_lenses
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

CREATE INDEX ON user_lenses (user_id);
CREATE INDEX ON user_lenses (lens_id);
CREATE INDEX ON user_lenses (user_id, lens_id);
