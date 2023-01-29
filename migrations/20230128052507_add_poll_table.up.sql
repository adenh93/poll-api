CREATE TABLE poll (
  id uuid NOT NULL,
  name TEXT NOT NULL,
  description TEXT,
  end_date timestamptz NOT NULL,
  created_at timestamptz NOT NULL,
  PRIMARY KEY (id)
);
