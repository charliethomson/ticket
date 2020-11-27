-- Your SQL goes here
CREATE TABLE notes (
    id BIGINT NOT NULL AUTO_INCREMENT,
    CONSTRAINT `pk_note_id` PRIMARY KEY (id),
    workorder_id BIGINT NOT NULL,
    CONSTRAINT `fk_workorder_id` FOREIGN KEY (workorder_id) REFERENCES workorders (id) ON DELETE CASCADE ON UPDATE RESTRICT,
    contents TEXT NOT NULL,
    user BIGINT NOT NULL,
    CONSTRAINT `fk_user_id` FOREIGN KEY (user) REFERENCES users (id) ON DELETE CASCADE ON UPDATE RESTRICT,
    posted int NOT NULL DEFAULT UNIX_TIMESTAMP(),
    next_update int
);