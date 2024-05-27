-- Create Steps TB303 Table
CREATE TABLE steps_tb303(
    step_id uuid,
    pattern_id uuid NOT NULL REFERENCES patterns_tb303 (pattern_id),
    number INTEGER NOT NULL,
    note TEXT,
    stem TEXT,
    "time" TEXT,
    accent BOOLEAN DEFAULT FALSE,
    slide BOOLEAN DEFAULT FALSE,
    updated_at timestamptz,
    created_at timestamptz,
    PRIMARY KEY (step_id)
);
