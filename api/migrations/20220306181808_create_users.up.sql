CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(40) UNIQUE NOT NULL,
    password VARCHAR(60),
    admin BOOLEAN DEFAULT false
);
