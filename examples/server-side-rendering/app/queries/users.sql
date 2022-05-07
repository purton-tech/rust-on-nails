--! example_query(id) {id, email, reset_password_selector?}*
SELECT 
    id, email, reset_password_selector
FROM 
    users
WHERE
    users.id < $1