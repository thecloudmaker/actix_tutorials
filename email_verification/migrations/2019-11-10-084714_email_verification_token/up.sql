
CREATE TABLE email_verification_token (
    id BYTEA PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);
    