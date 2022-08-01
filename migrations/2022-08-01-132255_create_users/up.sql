-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE users(
    id VARCHAR(36) DEFAULT uuid_generate_v4(),
    email VARCHAR(255) NOT NULL UNIQUE,
    pass VARCHAR(255) NOT NULL,
    confirmed BOOLEAN DEFAULT FALSE NOT NULL,

    CONSTRAINT pk_users_id PRIMARY KEY(id)

);