CREATE TABLE IF NOT EXISTS public.accounts (
    id CHAR(26) PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    birthday DATE NOT NULL,
    password TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
