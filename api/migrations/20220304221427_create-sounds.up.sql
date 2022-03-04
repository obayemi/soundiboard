CREATE TABLE sounds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(30) NOT NULL,
    file_url VARCHAR(100) NOT NULL,
    volume SMALLINT NOT NULL DEFAULT 75
);
