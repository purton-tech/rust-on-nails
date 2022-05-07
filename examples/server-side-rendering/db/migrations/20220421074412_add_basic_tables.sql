-- migrate:up

CREATE TABLE vaults (
    id SERIAL PRIMARY KEY, 
    user_id INT NOT NULL, 
    name VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE secrets (
    id SERIAL PRIMARY KEY, 
    vault_id INT NOT NULL, 
    name VARCHAR NOT NULL,
    secret VARCHAR NOT NULL,
    name_blind_index VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TABLE service_accounts (
    id SERIAL PRIMARY KEY, 
    user_id INT NOT NULL, 
    vault_id INT, 
    name VARCHAR NOT NULL,
    encrypted_ecdh_private_key VARCHAR NOT NULL,
    ecdh_public_key VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE service_account_secrets (
    id SERIAL PRIMARY KEY, 
    service_account_id INT NOT NULL, 
    name VARCHAR NOT NULL,
    secret VARCHAR NOT NULL,
    name_blind_index VARCHAR NOT NULL,
    ecdh_public_key VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE users_vaults (
    user_id INT NOT NULL, 
    vault_id INT NOT NULL, 
    ecdh_public_key VARCHAR NOT NULL,
    encrypted_vault_key VARCHAR NOT NULL
);
CREATE TABLE organisations (
    id SERIAL PRIMARY KEY, 
    name VARCHAR,
    created_by_user_id INT NOT NULL
);

CREATE TABLE organisation_users (
    user_id INT NOT NULL, 
    organisation_id INT NOT NULL,
    is_admin BOOLEAN NOT NULL DEFAULT false,
    PRIMARY KEY (user_id, organisation_id)
);

CREATE TABLE invitations (
    id SERIAL PRIMARY KEY, 
    organisation_id INT NOT NULL, 
    email VARCHAR NOT NULL,
    invitation_selector VARCHAR NOT NULL,
    invitation_verifier_hash VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
   CONSTRAINT fk_organisation
      FOREIGN KEY(organisation_id) 
	  REFERENCES organisations(id)
);

-- migrate:down
DROP TABLE vaults;
DROP TABLE secrets;
DROP TABLE service_accounts;
DROP TABLE service_account_secrets;
DROP TABLE users_vaults;
DROP TABLE organisation_users;
DROP TABLE invitations;
DROP TABLE organisations;
