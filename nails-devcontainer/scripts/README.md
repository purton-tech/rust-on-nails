# Scripts

This directory contains wrapper scripts for commands that need repo-specific configuration and secrets-derived environment variables before calling the underlying tool.

Use these scripts when a command needs to work from a clean, non-interactive shell, including coding agents and automation.

## Why these exist

Shell aliases and Bash functions are convenient for interactive use, but they are not a reliable interface for agents or other non-interactive tooling.

The scripts in this folder are the stable command surface for repo-specific operations.

## Current scripts

- `dev-env`: loads `/workspace/.env`, exports `DATABASE_URL`, and validates that the database connection settings are present.
- `psql`: opens `psql` using the repo's configured `DATABASE_URL`.
- `dbmate`: runs `dbmate` with the repo's configured `DATABASE_URL` and `crates/db/migrations` path.
- `clorinde`: runs one-shot Clorinde code generation against the repo's configured `DATABASE_URL`, reading from `crates/db/queries` and writing to `crates/db-gen`.

## How this fits with other tools

- `.devcontainer/.bash_aliases` may provide short interactive wrappers around these scripts.
- `Justfile` may provide convenience recipes that call these scripts.
- `AGENTS.md` should prefer these scripts over shell-specific helpers.

## Rule of thumb

If a command contains real repo logic or environment setup, put it here.

If a command is mainly a shortcut or multi-step workflow, put it in `Justfile`.
