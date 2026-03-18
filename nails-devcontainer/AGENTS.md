# Rules and Guidelines

This is a [Rust on Nails](https://rust-on-nails.com/) project using Rust to build a full stack web application.

The workspace root is `/workspace`.
Environment variables are loaded from `/workspace/.env`.

## Tech Stack

* Axum              # Handles all the applications routes and actions https://github.com/tokio-rs/axum
* Clorinde          # Generates a Rust crate from `.sql` files with type-checked Postgres queries https://halcyonnouveau.github.io/clorinde/
* Dioxus rsx! macro # Used to create UI components and pages on the server side. https://dioxuslabs.com/
* Daisy UI          # Tailwind components https://daisyui.com/
* daisy_rsx         # A rust crate that implements the Daisy UI components in rsx!
* DbMate            # Database Migrations https://github.com/amacneil/dbmate
* Postgres          # Database
* Earthly           # Build system for production. https://earthly.dev/

## Folder: crates/db/migrations

* To create a new migration run `./scripts/dbmate new migration-name`. Always use `dbmate new` so timestamps are correct.
* All of the `dbmate` migrations are stored in the `migrations` folder.
* Use `./scripts/dbmate ...` for migrations, `./scripts/psql ...` for direct `psql` access, and `./scripts/clorinde` for one-shot Clorinde code generation. These scripts load `/workspace/.env`, set `DATABASE_URL`, and point tools at the repo's database paths.
* When adding a new enum value (e.g., `ALTER TYPE ... ADD VALUE`), do not use the new value in the same migration transaction. Split into a follow-up migration before inserting rows that reference the new enum value.

## Folder: crates/db/queries

* Here is where we manage all interaction with the database.
* Any schema change in migrations must be reflected in the corresponding query files.
Clorinde will fail compilation if queries are out of sync.
* All of the `.sql` files are in a folder called `queries`.
* Each `.sql` file should group queries by aggregate/root entity (e.g., `users.sql`)
* All the database CRUD operation are in these files.
* Clorinde generates a dedicated Rust crate `db-gen` which is re-exported from `crates/db/lib.rs`.
* Do not use `SELECT *`; always specify columns explicitly.

## Folder: crates/db-gen

* After modifying any `.sql` file in the `crates/db/queries` folder, the Clorinde code generator must be run.
* To run `clorinde` and generate the database code use `./scripts/clorinde`
* Do not edit code in this folder, it will just be overwritten on the next code generation.

### Clorinde SQL Guidelines

* Never embed raw SQL in Rust code; all SQL must live in crates/db/queries/*.sql and be executed only via generated Clorinde query functions.
* **Struct Definitions**: Add `--: StructName` before queries to define return types
* **Query Naming**: Use `--! query_name` to name queries
* **Parameters**: Parameters are inferred automatically; do not declare them manually
* **Intervals**: Use `($1 || ' days')::INTERVAL` for dynamic intervals
* **Optional Fields**: Use `field_name?` for nullable fields when required

## Variables

`project_name=web`

## Folder: crates/${project_name}-assets

* Any images that are needed by the application are stored in a sub folder called images
* Also the tailwind config is stored here.
* The user will run `just tailwind` this will watch the tailwind `input.css` and src files for any changes.
* When changes occur the resulting `tailwind.css` is stored in a `dist` folder.
* There is a `build.rs` it uses a crate called `cache-busters` that sees the images and css files.
* It takes the hash of the files and creates a struct that gives us the ability to access the images by name in a typesafe way.
* For example the `tailwind.css` will be exported as `${project_name}::files::tailwind_css` in the app and we reference it by calling `${project_name}::files::tailwind.name`.

## Folder: crates/${project_name}-islands

This crate implements client-side interactivity using an Islands Architecture.
Use it for UI behavior that cannot be handled with server-side rendering.

It is compiled to WebAssembly and the output is deployed to the frontend via the assets crate.

Build commands (for reference only, not required for most changes):

- Compile to WASM:
  `cargo build -p ${project_name}-islands --target wasm32-unknown-unknown`

- Generate JS bindings:
  `wasm-bindgen target/wasm32-unknown-unknown/release/${project_name}_islands.wasm --target web --out-dir crates/${project_name}-assets/dist`

Do not run these commands unless explicitly required.

## Folder: crates/${project_name}-ui

* Every route has its own folder under `crates/${project_name}-ui`.
* The main page for a route lives in a file called `page.rs` inside that folder.
* Additional components are stored either alongside `page.rs` or in a `components/` folder.
* Shared widgets such as confirmation dialogs live under `components/` at the crate root.
* Each page corresponds to a typed route defined in `crates/${project_name}-ui/routes.rs` and is called from the matching handler in `crates/${project_name}/handlers`.
* We use Tailwind and Daisy UI. Only use Daisy UI colors and when possible the provided Daisy RSX library.
* Buttons can open modals by setting `popover_target` to the modal's `trigger_id`.
* Prefer DaisyUI components over raw Tailwind.
* Avoid custom Tailwind styling unless necessary.
* Do not hardcode colors; rely on theme tokens.

## Folder: crates/${project_name}

* Every route lives in its own folder under `crates/${project_name}/handlers`.
* Handler convention: each route domain in `crates/${project_name}/src/handlers/<domain>/` must use `loaders.rs` for GET handlers, `actions.rs` for POST handlers, and `mod.rs` to re-export both.
* POST endpoints are implemented in `actions.rs` with functions prefixed by `action_`.
* `mod.rs` re-exports the loader and actions and defines the `routes()` helper used by `main.rs`.
* Each loader function fetches data from the database and renders the page.
* Actions call the appropriate database functions before redirecting the browser.

## Earthfile

* We collect all docker containers into one build here.
* When creating new crates or services they may need to be added to this.

## Running the unit tests

* Run tests after any change affecting business logic, database queries, or handlers.
* `cargo test --workspace`
