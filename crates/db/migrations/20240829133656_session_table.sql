-- migrate:up
CREATE TABLE session (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    pubkey TEXT NOT NULL,
    message TEXT NOT NULL,
    is_used BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);

-- migrate:down
DROP TABLE session;