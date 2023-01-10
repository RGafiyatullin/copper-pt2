ALTER TABLE users
ADD CONSTRAINT cons_users_nickname_unique
UNIQUE (nickname)