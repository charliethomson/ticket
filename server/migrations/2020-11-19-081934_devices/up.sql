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