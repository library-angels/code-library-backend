ALTER TABLE users ADD COLUMN role_id INTEGER;
ALTER TABLE users ADD FOREIGN KEY (role_id) REFERENCES roles;

DO
$$
DECLARE
    f record;
BEGIN
    FOR f IN SELECT * FROM users_roles
    LOOP
	    UPDATE users SET role_id = f.role_id WHERE id = f.user_id;
    END LOOP;
END;
$$;

ALTER TABLE users ALTER COLUMN role_id SET NOT NULL;
DROP TABLE users_roles;
