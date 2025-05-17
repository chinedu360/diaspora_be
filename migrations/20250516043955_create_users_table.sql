-- Add migration script here
CREATE TABLE users (
    id uuid NOT NULL,
    PRIMARY KEY (id),
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    email VARCHAR(255) UNIQUE NOT NULL,
    phone_number VARCHAR(20),
    password_hash TEXT NOT NULL,
    country_of_residence VARCHAR(50),
    profile_picture_url TEXT,
    default_mode VARCHAR(20) DEFAULT 'sender' CHECK (default_mode IN ('sender', 'traveler')),
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW()
);