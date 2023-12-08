-- Your SQL goes here
CREATE TABLE account
(
    id         Serial PRIMARY KEY,
    created_at DATE    NOT NULL,
    verified   boolean NOT NULL DEFAULT false
);

CREATE TABLE api_key
(
    id         SERIAL PRIMARY KEY,
    key        VARCHAR(255) NOT NULL,
    expires    DATE      NOT NULL,
    created_at DATE      NOT NULL DEFAULT CURRENT_DATE,
    user_id    INT       NOT NULL,
    revoked    BOOLEAN   NOT NULL,
    CONSTRAINT FK_UserApiKey FOREIGN KEY (user_id) REFERENCES account (id)
);