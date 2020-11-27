CREATE TABLE stores (
    id BIGINT NOT NULL AUTO_INCREMENT,
    CONSTRAINT `id_unique_stores` UNIQUE (id),
    CONSTRAINT `id_primary_key_stores` PRIMARY KEY (id),
    contact_name VARCHAR(30) NOT NULL,
    phone_number VARCHAR(15) NOT NULL,
    email_address VARCHAR(50) NOT NULL,
    address VARCHAR(50) NOT NULL,
    city VARCHAR(50) NOT NULL,
    state VARCHAR(2) NOT NULL,
    zip INT NOT NULL
);