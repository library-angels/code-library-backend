ALTER TABLE roles ADD UNIQUE (name);
INSERT INTO roles (name, access_manage_books, access_manage_roles) VALUES ('User', False, False);
INSERT INTO roles (name, access_manage_books, access_manage_roles) VALUES ('Manager', True, False);
INSERT INTO roles (name, access_manage_books, access_manage_roles) VALUES ('Administrator', True, True);
