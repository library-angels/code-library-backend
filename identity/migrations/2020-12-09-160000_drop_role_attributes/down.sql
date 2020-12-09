ALTER TABLE roles ADD COLUMN access_manage_books BOOLEAN;
ALTER TABLE roles ADD COLUMN access_manage_roles BOOLEAN;

UPDATE roles SET access_manage_books = false, access_manage_roles = false WHERE name = 'User';
UPDATE roles SET access_manage_books = true, access_manage_roles = false WHERE name = 'Manager';
UPDATE roles SET access_manage_books = true, access_manage_roles = true WHERE name = 'Administrator';

ALTER TABLE roles ALTER COLUMN access_manage_books SET NOT NULL;
ALTER TABLE roles ALTER COLUMN access_manage_roles SET NOT NULL;
