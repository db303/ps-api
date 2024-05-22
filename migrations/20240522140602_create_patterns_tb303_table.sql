-- Create Patterns TB303 Table
CREATE TABLE patterns_tb303(
    pattern_id uuid,
    user_id uuid NOT NULL REFERENCES users (user_id),
    author TEXT,
    title TEXT,
    efx_notes TEXT,
    waveform TEXT,
    cutoff_frequency INTEGER DEFAULT 0,
    resonance INTEGER DEFAULT 0,
    env_mod INTEGER DEFAULT 0,
    decay INTEGER DEFAULT 0,
    accent INTEGER DEFAULT 0,
    updated_at timestamptz,
    created_at timestamptz,
    PRIMARY KEY (pattern_id)
);
