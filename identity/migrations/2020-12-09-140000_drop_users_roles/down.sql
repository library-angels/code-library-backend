CREATE TABLE users_roles (
    id SERIAL,
    user_id INTEGER,
    role_id INTEGER,
    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES users ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles ON UPDATE CASCADE ON DELETE CASCADE
);

DO
$$
DECLARE
    f record;
BEGIN
    FOR f IN SELECT * FROM users
    LOOP
	    INSERT INTO users_roles (user_id, role_id) VALUES (f.id, f.role_id);
    END LOOP;
END;
$$;

ALTER TABLE users_roles ALTER COLUMN user_id SET NOT NULL;
ALTER TABLE users_roles ALTER COLUMN role_id SET NOT NULL;
ALTER TABLE users DROP COLUMN role_id;
