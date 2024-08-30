-- migrate:up
CREATE TABLE message (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    recipient TEXT NOT NULL,
    sender TEXT NOT NULL,
    message TEXT NOT NULL,
    seen BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);

-- migrate:down
DROP TABLE message;
