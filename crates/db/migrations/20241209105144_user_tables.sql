-- migrate:up
CREATE TABLE users (
    id SERIAL PRIMARY KEY, 
    email VARCHAR NOT NULL UNIQUE, 
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

INSERT INTO users(email) VALUES('test1@test1.com');
INSERT INTO users(email) VALUES('test2@test1.com');
INSERT INTO users(email) VALUES('test3@test1.com');

-- migrate:down
DROP TABLE users;