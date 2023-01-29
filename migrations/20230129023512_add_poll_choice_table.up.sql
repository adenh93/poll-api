CREATE TABLE poll_choice (
  id uuid NOT NULL,
  poll_id uuid NOT NULL,
  name TEXT NOT NULL,
  created_at timestamptz NOT NULL, 
  PRIMARY KEY (id),
  FOREIGN KEY (poll_id) REFERENCES poll (id)
);
