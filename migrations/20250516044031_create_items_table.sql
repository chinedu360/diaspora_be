-- Add migration script here
CREATE TABLE items (
    id uuid NOT NULL,
    PRIMARY KEY (id),
    user_id uuid REFERENCES users(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    weight FLOAT CHECK (weight > 0),
    dimensions JSONB NOT NULL,
    origin_country VARCHAR(50),
    destination_country VARCHAR(50),
    price DECIMAL(10, 2),
    pickup_required BOOLEAN DEFAULT FALSE,
    status VARCHAR(50),
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

