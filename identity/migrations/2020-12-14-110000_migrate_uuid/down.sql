CREATE SEQUENCE users_id_seq;
ALTER TABLE users ALTER COLUMN id SET DATA TYPE INTEGER USING nextval('users_id_seq'::regclass);
ALTER TABLE users ALTER COLUMN id SET DEFAULT nextval('users_id_seq'::regclass);

ALTER TABLE roles ADD COLUMN id_old UUID;
UPDATE roles SET id_old = id;

ALTER TABLE users DROP CONSTRAINT users_role_id_fkey;

CREATE SEQUENCE roles_id_seq;
ALTER TABLE roles ALTER COLUMN id SET DATA TYPE INTEGER USING nextval('roles_id_seq'::regclass);
ALTER TABLE roles ALTER COLUMN id SET DEFAULT nextval('roles_id_seq'::regclass);

ALTER TABLE users ADD COLUMN role_id_new INTEGER;

UPDATE users SET role_id_new = (SELECT id FROM roles WHERE roles.id_old = users.role_id);

ALTER TABLE roles DROP COLUMN id_old;
ALTER TABLE roles ALTER COLUMN id SET DEFAULT nextval('roles_id_seq'::regclass);
ALTER TABLE users DROP COLUMN role_id;
ALTER TABLE users RENAME COLUMN role_id_new TO role_id;
ALTER TABLE users ALTER COLUMN role_id SET NOT NULL;
ALTER TABLE users ADD FOREIGN KEY (role_id) REFERENCES roles ON UPDATE CASCADE ON DELETE CASCADE;
