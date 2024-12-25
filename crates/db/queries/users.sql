--: User()

--! get_users : User
SELECT 
    id, 
    email
FROM users;

-- 👇 add `create_user` query
--! create_user
INSERT INTO 
    users (email)
VALUES
    (:email);