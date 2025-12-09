# Auth Schema

Use this Postgres `auth` schema to enable flexible external authentication with JWT claims, typically coming from providers such as Keycloak. The schema expects your application to push verified claims JSON into the `row_level_security.jwt` setting before running queries.

## Full schema

Copy/paste the block below into a migration file to create the schema, table, and helper functions in one shot.

```sql
-- ===========================================
-- SCHEMA
-- ===========================================
CREATE SCHEMA IF NOT EXISTS auth;

-- ===========================================
-- TABLE: auth.users
-- ===========================================
CREATE TABLE IF NOT EXISTS auth.users (
  id           bigserial PRIMARY KEY,       -- tight internal ID
  external_id  text UNIQUE NOT NULL,        -- Keycloak sub
  email        text UNIQUE,
  first_name   text,
  last_name    text,
  created_at   timestamptz NOT NULL DEFAULT now(),
  updated_at   timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_auth_users_external_id ON auth.users (external_id);
CREATE INDEX IF NOT EXISTS idx_auth_users_email       ON auth.users (email);

-- ===========================================
-- TYPE: auth.team_role
-- ===========================================
DO $$
BEGIN
  IF NOT EXISTS (
    SELECT 1
    FROM pg_type t
    JOIN pg_namespace n ON n.oid = t.typnamespace
    WHERE t.typname = 'team_role'
      AND n.nspname = 'auth'
  ) THEN
    CREATE TYPE auth.team_role AS ENUM ('Owner', 'Member');
  END IF;
END;
$$;

-- ===========================================
-- TABLE: auth.teams
-- ===========================================
CREATE TABLE IF NOT EXISTS auth.teams (
  id          bigserial PRIMARY KEY,
  name        text NOT NULL,
  created_by  bigint NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
  created_at  timestamptz NOT NULL DEFAULT now(),
  updated_at  timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_auth_teams_created_by ON auth.teams (created_by);

-- ===========================================
-- TABLE: auth.team_members
-- ===========================================
CREATE TABLE IF NOT EXISTS auth.team_members (
  team_id   bigint NOT NULL REFERENCES auth.teams(id) ON DELETE CASCADE,
  user_id   bigint NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
  role      auth.team_role NOT NULL DEFAULT 'Member',
  joined_at timestamptz NOT NULL DEFAULT now(),

  PRIMARY KEY (team_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_auth_team_members_user ON auth.team_members (user_id);

-- ===========================================
-- FUNCTION: auth.jwt()
-- ===========================================
CREATE OR REPLACE FUNCTION auth.jwt()
RETURNS jsonb
LANGUAGE sql
STABLE
AS $$
  SELECT
    CASE
      WHEN current_setting('row_level_security.jwt', true) IS NULL
        THEN '{}'::jsonb
      ELSE current_setting('row_level_security.jwt', true)::jsonb
    END;
$$;

-- ===========================================
-- FUNCTION: auth.me()
-- ===========================================
CREATE OR REPLACE FUNCTION auth.me(p_team_id bigint DEFAULT NULL)
RETURNS auth.users
LANGUAGE plpgsql
VOLATILE
AS $$
DECLARE
  _claims     jsonb := auth.jwt();
  _sub        text;
  _email      text;
  _first_name text;
  _last_name  text;
  _user       auth.users;
  _team_id    bigint;
  _team_name  text;
BEGIN
  _sub := _claims->>'sub';
  IF _sub IS NULL THEN
    RAISE EXCEPTION 'JWT missing sub claim';
  END IF;

  _email      := _claims->>'email';
  _first_name := COALESCE(_claims->>'given_name', _claims->>'first_name');
  _last_name  := COALESCE(_claims->>'family_name', _claims->>'last_name');

  INSERT INTO auth.users (external_id, email, first_name, last_name)
  VALUES (
    _sub,
    _email,
    _first_name,
    _last_name
  )
  ON CONFLICT (external_id) DO UPDATE
    SET email      = COALESCE(EXCLUDED.email, auth.users.email),
        first_name = COALESCE(EXCLUDED.first_name, auth.users.first_name),
        last_name  = COALESCE(EXCLUDED.last_name, auth.users.last_name),
        updated_at = now()
  RETURNING * INTO _user;

  IF p_team_id IS NOT NULL THEN
    _team_id := p_team_id;
  ELSE
    SELECT tm.team_id
    INTO _team_id
    FROM auth.team_members tm
    WHERE tm.user_id = _user.id
    ORDER BY tm.joined_at
    LIMIT 1;
  END IF;

  IF _team_id IS NULL THEN
    _team_name := format('%s''s team', COALESCE(NULLIF(trim(_first_name), ''), 'My'));

    INSERT INTO auth.teams (name, created_by)
    VALUES (_team_name, _user.id)
    RETURNING id INTO _team_id;

    INSERT INTO auth.team_members (team_id, user_id, role)
    VALUES (_team_id, _user.id, 'Owner')
    ON CONFLICT (team_id, user_id) DO NOTHING;
  ELSE
    INSERT INTO auth.team_members (team_id, user_id, role)
    VALUES (_team_id, _user.id, 'Member')
    ON CONFLICT (team_id, user_id) DO NOTHING;
  END IF;

  RETURN _user;
END;
$$;

-- ===========================================
-- FUNCTION: auth.id()
-- ===========================================
CREATE OR REPLACE FUNCTION auth.id()
RETURNS bigint
LANGUAGE plpgsql
STABLE
AS $$
DECLARE
  _sub text;
  _id  bigint;
BEGIN
  _sub := auth.jwt() ->> 'sub';
  IF _sub IS NULL THEN
    RAISE EXCEPTION 'JWT missing sub claim';
  END IF;

  SELECT u.id
  INTO _id
  FROM auth.users u
  WHERE u.external_id = _sub;

  IF NOT FOUND THEN
    RAISE EXCEPTION 'No auth.users row for sub=%, call auth.me() first', _sub;
  END IF;

  RETURN _id;
END;
$$;
```

The rest of this chapter calls out the important pieces if you want to tweak or extend the schema.

- `auth.users` stores one row per Keycloak subject (`sub`). We keep both the opaque `external_id` and a dense `id` for joins inside the app schema.
- `auth.teams` and `auth.team_members` keep collaborative groups next to the auth data so you can enforce RLS with a single schema.
- The optional indexes make typical lookup paths explicit (Keycloak subjects, email, or team membership).

## Example usage

Once the schema is installed, start each request or job by setting the JWT and calling the helpers:

```sql
BEGIN;

SET LOCAL row_level_security.jwt = $${
  "sub": "1234567890abcdef",
  "email": "daniel@example.com",
  "given_name": "Daniel",
  "family_name": "Purton"
}$$;

SELECT auth.jwt();
SELECT * FROM auth.me();                       -- upsert user + default team
SELECT auth.id();                              -- dense internal id

-- join an existing team in the same transaction
SELECT * FROM auth.me(p_team_id := 42);

COMMIT;
```

## Helper functions

### `auth.jwt()`

```sql
CREATE OR REPLACE FUNCTION auth.jwt()
RETURNS jsonb
LANGUAGE sql    
STABLE
AS $$
  SELECT
    CASE
      WHEN current_setting('row_level_security.jwt', true) IS NULL
        THEN '{}'::jsonb
      ELSE current_setting('row_level_security.jwt', true)::jsonb
    END;
$$;
```

`auth.jwt()` reads the JSON claims payload injected with `SET LOCAL row_level_security.jwt = '<claims>'`. Returning `{}` instead of `NULL` lets calling functions safely destructure the result with `->>` accessors.

### `auth.me()`

```sql
CREATE OR REPLACE FUNCTION auth.me(p_team_id bigint DEFAULT NULL)
RETURNS auth.users
LANGUAGE plpgsql
VOLATILE
AS $$
DECLARE
  _claims     jsonb := auth.jwt();
  _sub        text;
  _email      text;
  _first_name text;
  _last_name  text;
  _user       auth.users;
  _team_id    bigint;
  _team_name  text;
BEGIN
  _sub := _claims->>'sub';
  IF _sub IS NULL THEN
    RAISE EXCEPTION 'JWT missing sub claim';
  END IF;

  _email      := _claims->>'email';
  _first_name := COALESCE(_claims->>'given_name', _claims->>'first_name');
  _last_name  := COALESCE(_claims->>'family_name', _claims->>'last_name');

  INSERT INTO auth.users (external_id, email, first_name, last_name)
  VALUES (
    _sub,
    _email,
    _first_name,
    _last_name
  )
  ON CONFLICT (external_id) DO UPDATE
    SET email      = COALESCE(EXCLUDED.email, auth.users.email),
        first_name = COALESCE(EXCLUDED.first_name, auth.users.first_name),
        last_name  = COALESCE(EXCLUDED.last_name, auth.users.last_name),
        updated_at = now()
  RETURNING * INTO _user;

  IF p_team_id IS NOT NULL THEN
    _team_id := p_team_id;
  ELSE
    SELECT tm.team_id
    INTO _team_id
    FROM auth.team_members tm
    WHERE tm.user_id = _user.id
    ORDER BY tm.joined_at
    LIMIT 1;
  END IF;

  IF _team_id IS NULL THEN
    _team_name := format('%s''s team', COALESCE(NULLIF(trim(_first_name), ''), 'My'));

    INSERT INTO auth.teams (name, created_by)
    VALUES (_team_name, _user.id)
    RETURNING id INTO _team_id;

    INSERT INTO auth.team_members (team_id, user_id, role)
    VALUES (_team_id, _user.id, 'Owner')
    ON CONFLICT (team_id, user_id) DO NOTHING;
  ELSE
    INSERT INTO auth.team_members (team_id, user_id, role)
    VALUES (_team_id, _user.id, 'Member')
    ON CONFLICT (team_id, user_id) DO NOTHING;
  END IF;

  RETURN _user;
END;
$$;
```

`auth.me()` still upserts the user row, but it also ensures the caller belongs to a team. Pass `p_team_id` to join/switch an existing team; otherwise the function reuses the first team membership it finds or creates a personal team named after the user (for example, “Daniel's team”). When you add the RBAC schema, replace this helper with the extended version in that chapter so the return value also includes roles and permissions.

### `auth.id()`

```sql
CREATE OR REPLACE FUNCTION auth.id()
RETURNS bigint
LANGUAGE plpgsql
STABLE
AS $$
DECLARE
  _sub text;
  _id  bigint;
BEGIN
  _sub := auth.jwt() ->> 'sub';
  IF _sub IS NULL THEN
    RAISE EXCEPTION 'JWT missing sub claim';
  END IF;

  SELECT u.id
  INTO _id
  FROM auth.users u
  WHERE u.external_id = _sub;

  IF NOT FOUND THEN
    RAISE EXCEPTION 'No auth.users row for sub=%, call auth.me() first', _sub;
  END IF;

  RETURN _id;
END;
$$;
```

`auth.id()` bridges your request context with database‑side RLS policies. After you call `auth.me()` once, `auth.id()` gives you the dense bigint to use inside policies (`current_setting('row_level_security.user_id') := auth.id()`), auditing tables, or other helper functions.

## Usage notes

- Set `row_level_security.jwt` for every transaction; leaving it `NULL` makes helper functions throw, which protects you from running queries without an authenticated user.
- When you add more profile fields, extend both the table and the `auth.me()` upsert. The conflict handler is already wired to `COALESCE` so optional attributes remain untouched.
- This schema is intentionally slim: keep organization or permission data in separate modules and join using `auth.id()` when building row‑level policies.
