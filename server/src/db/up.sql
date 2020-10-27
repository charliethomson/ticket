CREATE TABLE users (
    id BIGINT NOT NULL AUTO_INCREMENT,
    CONSTRAINT `pk_user_id` PRIMARY KEY (id),
    google_id BINARY(32) NOT NULL,
    CONSTRAINT `unq_google_id` UNIQUE (google_id),
    first_name VARCHAR(25) NOT NULL,
    last_name VARCHAR(25) NOT NULL,
    email VARCHAR(50) NOT NULL
);
CREATE TABLE notes (
    note_id BIGINT NOT NULL AUTO_INCREMENT,
    CONSTRAINT `pk_note_id` PRIMARY KEY (note_id),
    wo_key INT NOT NULL,
    contents TEXT NOT NULL,
    user BIGINT NOT NULL,
    CONSTRAINT `fk_user_id` FOREIGN KEY (user) REFERENCES users (id) ON DELETE CASCADE ON UPDATE RESTRICT,
    posted int NOT NULL,
    next_update int
);
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
CREATE TABLE customers (
    id BIGINT NOT NULL AUTO_INCREMENT,
    CONSTRAINT `id_unique_customers` UNIQUE (id),
    CONSTRAINT `id_primary_key_customers` PRIMARY KEY (id),
    first_name VARCHAR(20) NOT NULL,
    last_name VARCHAR(20) NOT NULL,
    phone_number VARCHAR(15) NOT NULL,
    email_address VARCHAR(50) NOT NULL
);
CREATE TABLE devices (
    id BIGINT NOT NULL AUTO_INCREMENT,
    CONSTRAINT `id_unique_devices` UNIQUE (id),
    CONSTRAINT `id_primary_key_devices` PRIMARY KEY (id),
    serial_no VARCHAR(30) NOT NULL,
    device_name VARCHAR(30) NOT NULL,
    customer BIGINT NOT NULL,
    CONSTRAINT `fk_customer_id` FOREIGN KEY (customer) REFERENCES customers (id) ON DELETE CASCADE ON UPDATE RESTRICT,
    password TEXT
);
CREATE TABLE workorders (
    id BIGINT NOT NULL AUTO_INCREMENT,
    CONSTRAINT `id_unique_workorders` UNIQUE (id),
    CONSTRAINT `id_primary_key_workorders` PRIMARY KEY (id),
    active BOOLEAN NOT NULL,
    origin BIGINT NOT NULL,
    CONSTRAINT `fk_origin_store` FOREIGN KEY (origin) REFERENCES stores (id) ON DELETE CASCADE ON UPDATE RESTRICT,
    created INT NOT NULL,
    quoted INT,
    workorder_status INT NOT NULL,
    travel_status INT NOT NULL,
    location VARCHAR(15),
    customer BIGINT NOT NULL,
    CONSTRAINT `fk_wo_customer_id` FOREIGN KEY (customer) REFERENCES customers (id) ON DELETE CASCADE ON UPDATE RESTRICT,
    device BIGINT NOT NULL,
    CONSTRAINT `fk_device_id` FOREIGN KEY (device) REFERENCES devices (id) ON DELETE CASCADE ON UPDATE RESTRICT,
    brief VARCHAR(144) NOT NULL
);