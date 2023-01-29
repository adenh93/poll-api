CREATE TABLE poll_vote (
  id uuid NOT NULL,
  poll_id uuid NOT NULL,
  choice_id uuid NOT NULL,
  ip_address inet NOT NULL,
  created_at timestamptz NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (poll_id) REFERENCES poll (id),
  FOREIGN KEY (choice_id) REFERENCES poll_choice (id)
);
