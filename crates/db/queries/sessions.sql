--: Session()

--! get_after_date : Session
SELECT
    *
FROM session
WHERE
    pubkey = :pubkey
    AND created_at > :date
ORDER BY created_at DESC;

--! get_latest_by_user : Session
SELECT
    *
FROM session
WHERE
    pubkey = :pubkey
ORDER BY created_at DESC
LIMIT 1;


--! insert
INSERT INTO
    session (pubkey, message)
VALUES
    (:pubkey, :message);


--! set_as_used
UPDATE session
SET is_used = TRUE, updated_at = now()
WHERE id = :id;