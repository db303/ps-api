-- Create Patterns TB303 Table
CREATE TABLE patterns_tb303(
    pattern_id uuid,
    user_id uuid NOT NULL REFERENCES users (user_id),
    author TEXT NOT NULL,
    title TEXT NOT NULL,
    efx_notes TEXT NOT NULL,
    waveform TEXT NOT NULL,
    cutoff_frequency INTEGER NOT NULL,
    resonance INTEGER NOT NULL,
    env_mod INTEGER NOT NULL,
    decay INTEGER NOT NULL,
    accent INTEGER NOT NULL,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    PRIMARY KEY (pattern_id)
);
