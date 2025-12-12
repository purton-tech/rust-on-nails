# Encryption Helpers

Use these helpers to encrypt/decrypt text columns with a symmetric key stored in `encryption.root_key`. They are written as DBMate migrations and can be chained after the Auth/Teams/RBAC steps.

**What this covers**
- Runtime, application-layer encryption: data is encrypted/decrypted in your SQL.
- Complements (does not replace) encryption at rest provided by your database/storage.
- Useful to reduce blast radius from accidental dumps or “SELECT *” snooping while still relying on storage-level encryption for disks and backups.

## Migration

```sql
-- migrate:up
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE OR REPLACE FUNCTION encrypt_text(data text) RETURNS text AS $$
DECLARE
    key text;
    encrypted text;
BEGIN
    key := current_setting('encryption.root_key', true);

    IF key IS NULL THEN
        RETURN data;
    ELSE
        BEGIN
            encrypted := pgp_sym_encrypt(data, key, 'compress-algo=1, cipher-algo=aes256');
            RETURN encrypted;
        EXCEPTION WHEN others THEN
            RETURN SQLERRM;
        END;
    END IF;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION decrypt_text(data text) RETURNS text AS $$
DECLARE
    key text;
    decrypted text;
BEGIN
    key := current_setting('encryption.root_key', true);

    IF key IS NULL THEN
        RETURN data;
    ELSE
        BEGIN
            decrypted := pgp_sym_decrypt(data::bytea, key);
            RETURN decrypted;
        EXCEPTION WHEN others THEN
            RETURN data;
        END;
    END IF;
END;
$$ LANGUAGE plpgsql;

-- migrate:down
DROP FUNCTION IF EXISTS decrypt_text(text);
DROP FUNCTION IF EXISTS encrypt_text(text);
DROP EXTENSION IF EXISTS pgcrypto;
```

## Usage

- Insert: `INSERT INTO customers (email_encrypted) VALUES (encrypt_text('alice@example.com'));`
- Update: `UPDATE customers SET email_encrypted = encrypt_text('new@example.com') WHERE id = 1;`
- Select: `SELECT decrypt_text(email_encrypted) AS email FROM customers;`

Set the key per session/transaction so pgcrypto can decrypt:

```sql
SET LOCAL encryption.root_key = 'base64-or-env-derived-key';
```

If `encryption.root_key` is absent, the helpers return the input unchanged. This keeps local development simple while letting staging/production supply a real key through Postgres settings or `ALTER SYSTEM`.

## Generate a key with OpenSSL

Create a 256-bit key and store it in your secret manager or environment:

```sh
openssl rand -base64 32
```

Set that value on the session before running queries (for example via `SET LOCAL encryption.root_key = '...';`) or configure it as a Postgres setting so the functions can encrypt/decrypt transparently.
