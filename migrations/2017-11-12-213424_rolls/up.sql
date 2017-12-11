CREATE TABLE rolls (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  film_stock_id UUID REFERENCES film_stocks NOT NULL,
  user_camera_id UUID REFERENCES user_cameras NOT NULL,
  display_id VARCHAR NOT NULL,
  loaded_at DATE NOT NULL DEFAULT CURRENT_DATE,
  finished_at DATE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE (display_id)
);

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON rolls
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

CREATE INDEX ON rolls (film_stock_id);
CREATE INDEX ON rolls (user_camera_id);
CREATE INDEX ON rolls (display_id);
