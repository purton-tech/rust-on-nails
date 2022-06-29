--! world(word_id) { id, message }
SELECT 
    id, randomnumber
FROM 
    World
WHERE
    id = $1