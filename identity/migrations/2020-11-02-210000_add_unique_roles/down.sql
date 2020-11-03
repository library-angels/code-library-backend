ALTER TABLE roles DROP CONSTRAINT roles_name_key;
DELETE FROM roles WHERE name='User';
DELETE FROM roles WHERE name='Manager';
DELETE FROM roles WHERE name='Administrator';
