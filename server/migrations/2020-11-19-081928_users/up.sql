CREATE TABLE users (
    id BIGINT NOT NULL AUTO_INCREMENT,
    CONSTRAINT `pk_user_id` PRIMARY KEY (id),
    google_id BINARY(32),
    CONSTRAINT `unq_google_id` UNIQUE (google_id),
    portal_id BIGINT,
    CONSTRAINT `unq_portal_id` UNIQUE (portal_id),
    CONSTRAINT `or_google_portal` CHECK (
        portal_id IS NOT NULL
        OR google_id IS NOT NULL
    ),
    first_name VARCHAR(25) NOT NULL,
    last_name VARCHAR(25) NOT NULL,
    email_address VARCHAR(50) NOT NULL,
    CONSTRAINT `unq_user_email` UNIQUE (email_address)
);