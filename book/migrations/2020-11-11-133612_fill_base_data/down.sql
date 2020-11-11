-- See following post for difference in-between `DROP FROM`
-- and `TRUNCATE`: https://stackoverflow.com/a/11423886
-- 
TRUNCATE categories,
languages CASCADE;
