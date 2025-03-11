CREATE TABLE "user" (
    id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    
    password VARCHAR(255) NOT NULL,
    password_encryption_salt UUID NOT NULL DEFAULT gen_random_uuid(),
    token_encryption_salt UUID NOT NULL DEFAULT gen_random_uuid(),

    CONSTRAINT email_and_password_unique_together UNIQUE (email, "password")
);

-- An index is a way to speed up queries
CREATE INDEX username_index ON "user" (username);