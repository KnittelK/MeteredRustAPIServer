-- Your SQL goes here
ALTER TABLE account
    ADD COLUMN username VARCHAR(16) NOT NULL,
    ADD COLUMN password VARCHAR(255) NOT NULL,
    ADD COLUMN email VARCHAR(255) NOT NULL;

CREATE UNIQUE INDEX idx_account_username ON account(username);
CREATE UNIQUE INDEX idx_account_email ON account(email);