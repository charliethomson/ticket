CREATE TABLE IF NOT EXISTS workorders (
    id INT NOT NULL AUTO_INCREMENT,
    CONSTRAINT `id_unique_workorders` UNIQUE (id),
    CONSTRAINT `id_primary_key_workorders` PRIMARY KEY (id),
    active BOOLEAN NOT NULL,
    origin INT NOT NULL,
    CONSTRAINT `fk_origin_store` FOREIGN KEY (origin) REFERENCES stores (id) ON DELETE CASCADE ON UPDATE RESTRICT,
    travel_status VARCHAR(40) NOT NULL,
    created INT NOT NULL,
    quoted INT,
    workorder_status VARCHAR(40) NOT NULL,
    customer INT NOT NULL,
    CONSTRAINT `fk_wo_customer_id` FOREIGN KEY (customer) REFERENCES customers (id) ON DELETE CASCADE ON UPDATE RESTRICT,
    device INT NOT NULL,
    CONSTRAINT `fk_device_id` FOREIGN KEY (device) REFERENCES devices (id) ON DELETE CASCADE ON UPDATE RESTRICT,
    brief VARCHAR(144) NOT NULL
);
CREATE TABLE IF NOT EXISTS customers (
    id INT NOT NULL AUTO_INCREMENT,
    CONSTRAINT `id_unique_customers` UNIQUE (id),
    CONSTRAINT `id_primary_key_customers` PRIMARY KEY (id),
    customer_name VARCHAR(40) NOT NULL,
    phone_number VARCHAR(15) NOT NULL,
    email_address VARCHAR(50) NOT NULL,
    store_id INT NOT NULL,
    CONSTRAINT `fk_store_id` FOREIGN KEY (store_id) REFERENCES stores (id) ON DELETE CASCADE ON UPDATE RESTRICT
);
CREATE TABLE IF NOT EXISTS stores (
    id INT NOT NULL AUTO_INCREMENT,
    CONSTRAINT `id_unique_stores` UNIQUE (id),
    CONSTRAINT `id_primary_key_stores` PRIMARY KEY (id),
    store_name VARCHAR(30) NOT NULL,
    contact_name VARCHAR(30) NOT NULL,
    phone_number VARCHAR(15) NOT NULL,
    email_address VARCHAR(50) NOT NULL,
    address VARCHAR(50) NOT NULL,
    city VARCHAR(50) NOT NULL,
    state VARCHAR(50) NOT NULL,
    zip INT NOT NULL
);
CREATE TABLE IF NOT EXISTS devices (
    id INT NOT NULL AUTO_INCREMENT,
    CONSTRAINT `id_unique_devices` UNIQUE (id),
    CONSTRAINT `id_primary_key_devices` PRIMARY KEY (id),
    serial_no VARCHAR(30) NOT NULL,
    device_name VARCHAR(30) NOT NULL,
    customer INT NOT NULL,
    CONSTRAINT `fk_customer_id` FOREIGN KEY (customer) REFERENCES customers (id) ON DELETE CASCADE ON UPDATE RESTRICT,
    password TEXT
);
CREATE TABLE notes (
    note_id INT NOT NULL AUTO_INCREMENT,
    CONSTRAINT `pk_note_id` PRIMARY KEY (note_id),
    wo_key INT NOT NULL,
    contents TEXT NOT NULL,
    user INT NOT NULL,
    CONSTRAINT `fk_user_id` FOREIGN KEY (user) REFERENCES users (id) ON DELETE CASCADE ON UPDATE RESTRICT,
    posted INT NOT NULL,
    next_update INT
);
CREATE TABLE users (
    id INT NOT NULL AUTO_INCREMENT,
    CONSTRAINT `pk_user_id` PRIMARY KEY (id),
    name VARCHAR(30) NOT NULL,
    phone_number VARCHAR(15)
)