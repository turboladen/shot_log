CREATE TABLE user_cameras (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES users NOT NULL,
  camera_id UUID REFERENCES cameras NOT NULL,
  roll_prefix VARCHAR NOT NULL,
  serial_number VARCHAR,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE (user_id, camera_id, roll_prefix)
);

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON user_cameras
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

CREATE INDEX ON user_cameras (user_id);
CREATE INDEX ON user_cameras (camera_id);
CREATE INDEX ON user_cameras (user_id, camera_id);
