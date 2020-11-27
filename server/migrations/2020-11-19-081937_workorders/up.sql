CREATE TABLE workorders (
    id BIGINT NOT NULL AUTO_INCREMENT,
    CONSTRAINT `id_unique_workorders` UNIQUE (id),
    CONSTRAINT `id_primary_key_workorders` PRIMARY KEY (id),
    active BOOLEAN NOT NULL DEFAULT 1,
    origin BIGINT NOT NULL,
    CONSTRAINT `fk_origin_store` FOREIGN KEY (origin) REFERENCES stores (id) ON DELETE CASCADE ON UPDATE RESTRICT,
    created INT NOT NULL DEFAULT UNIX_TIMESTAMP(),
    quoted INT,
    workorder_status INT NOT NULL DEFAULT 0,
    travel_status INT NOT NULL DEFAULT 0,
    location VARCHAR(15),
    customer BIGINT NOT NULL,
    CONSTRAINT `fk_wo_customer_id` FOREIGN KEY (customer) REFERENCES customers (id) ON DELETE CASCADE ON UPDATE RESTRICT,
    device BIGINT NOT NULL,
    CONSTRAINT `fk_device_id` FOREIGN KEY (device) REFERENCES devices (id) ON DELETE CASCADE ON UPDATE RESTRICT,
    brief VARCHAR(144) NOT NULL
);