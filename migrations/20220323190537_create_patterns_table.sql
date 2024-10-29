-- Create Patterns Table
CREATE TABLE patterns(
    id uuid NOT NULL,
    name TEXT NOT NULL,
    device TEXT NOT NULL,
    data json NOT NULL,
    user_id uuid NOT NULL,
    created_at timestamptz NOT NULL,
    PRIMARY KEY (id)
);