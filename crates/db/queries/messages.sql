--: Message()

--! insert: Message
INSERT INTO
    message (sender, recipient, message)
VALUES
    (:sender, :recipient, :message)
RETURNING *;


--! seen
UPDATE message
SET seen = TRUE, updated_at = now()
WHERE id = :id;