CREATE TABLE customers (
    id BIGINT NOT NULL AUTO_INCREMENT,
    CONSTRAINT `id_unique_customers` UNIQUE (id),
    CONSTRAINT `id_primary_key_customers` PRIMARY KEY (id),
    first_name VARCHAR(20) NOT NULL,
    last_name VARCHAR(20) NOT NULL,
    phone_number VARCHAR(15) NOT NULL,
    email_address VARCHAR(50) NOT NULL
);