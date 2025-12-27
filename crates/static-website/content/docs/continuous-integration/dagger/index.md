# Dagger

[https://dagger.io](Dagger) allows us to define our build using Rust. It's also based around docker so we can re-use our devcontainer.

## The Prompt

We'll use AI to setup our build. Use the following prompt

```rust
use anyhow::Result;
use clap::{Parser, Subcommand};
use dagger_sdk::{
    Container, DirectoryDockerBuildOptsBuilder, HostDirectoryOptsBuilder, Query, Service,
};
use eyre::eyre;
use url::Url;

const POSTGRES_IMAGE: &str = "postgres:16-alpine";
const DB_PASSWORD: &str = "password";
const DB_USER: &str = "postgres";
const DB_NAME: &str = "postgres";

#[derive(Parser)]
#[command(name = "infrastructure")]
#[command(about = "Dagger pipeline for migrations and the web server")]
struct Cli {
    /// Optional tag to publish the migration image (e.g. docker-daemon:local/dbmate:latest)
    #[arg(long)]
    migrations_tag: Option<String>,
    /// Optional tag to publish the web image (e.g. docker-daemon:local/web:latest)
    #[arg(long)]
    web_tag: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the migration and web containers
    Build,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build => {
            let database_url = default_database_url();
            dagger_sdk::connect(|client| async move {
                run_build(
                    &client,
                    &database_url,
                    cli.migrations_tag.as_deref(),
                    cli.web_tag.as_deref(),
                )
                .await
                .map_err(|e| eyre!(e))?;
                Ok(())
            })
            .await?;
        }
    }

    Ok(())
}

async fn run_build(
    client: &Query,
    database_url: &str,
    migrations_tag: Option<&str>,
    web_tag: Option<&str>,
) -> Result<()> {
    let pipeline_db_url = pipeline_database_url(database_url);

    let workspace = client.host().directory_opts(
        ".",
        HostDirectoryOptsBuilder::default()
            .exclude(vec!["target", "dagger-cache"])
            .build()?,
    );

    let devcontainer_ctx = client.host().directory(".devcontainer");

    let postgres = postgres_service(client);
    run_migrations(client, &pipeline_db_url, &postgres).await?;

    let dev_image = devcontainer_ctx.docker_build_opts(
        DirectoryDockerBuildOptsBuilder::default()
            .dockerfile("Dockerfile")
            .build()?,
    );

    let builder = dev_image
        .with_directory("/workspace", workspace)
        .with_workdir("/workspace")
        .with_user("root")
        .with_service_binding("postgres", postgres.clone())
        .with_env_variable("DATABASE_URL", pipeline_db_url.clone());

    let builder = generate_client_and_assets(builder, &pipeline_db_url);
    let builder = compile_web_server(builder);

    build_migration_container(client, migrations_tag).await?;
    build_web_container(client, builder, web_tag).await?;

    Ok(())
}

fn generate_client_and_assets(builder: Container, database_url: &str) -> Container {
    builder
        .with_exec(vec!["chmod", "-R", "u+rw", "/workspace/crates/clorinde"])
        .with_exec(vec![
            "clorinde",
            "live",
            "--serialize",
            "true",
            "-q",
            "./crates/db/queries/",
            "-d",
            "crates/clorinde",
            database_url,
        ])
        .with_exec(vec![
            "cargo",
            "build",
            "-p",
            "web-islands",
            "--target",
            "wasm32-unknown-unknown",
            "--release",
        ])
        .with_exec(vec![
            "wasm-bindgen",
            "target/wasm32-unknown-unknown/release/web_islands.wasm",
            "--target",
            "web",
            "--out-dir",
            "crates/web-assets/dist",
        ])
        .with_exec(vec![
            "tailwind-extra",
            "-i",
            "./crates/web-assets/input.css",
            "-o",
            "./crates/web-assets/dist/tailwind.css",
        ])
}

fn compile_web_server(builder: Container) -> Container {
    builder.with_exec(vec![
        "cargo",
        "build",
        "--release",
        "-p",
        "web-server",
        "--target",
        "x86_64-unknown-linux-musl",
    ])
}

async fn build_migration_container(client: &Query, tag: Option<&str>) -> Result<()> {
    let migrations = client.host().directory("crates/db/migrations");

    let image = client
        .container()
        .from("ghcr.io/amacneil/dbmate:2")
        .with_workdir("/db")
        .with_directory("/db/migrations", migrations);

    if let Some(tag) = tag {
        image.publish(tag).await?;
        println!("✅ migration image published to {tag}");
    } else {
        image.id().await?;
        println!("✅ migration container built");
    }

    Ok(())
}

async fn build_web_container(client: &Query, builder: Container, tag: Option<&str>) -> Result<()> {
    let binary = builder.file("/workspace/target/x86_64-unknown-linux-musl/release/web-server");
    let assets = builder.directory("/workspace/crates/web-assets/dist");
    let images = builder.directory("/workspace/crates/web-assets/images");

    let container = client
        .container()
        .with_user("1001")
        .with_file("/web-server", binary)
        .with_directory("/workspace/crates/web-assets/dist", assets)
        .with_directory("/workspace/crates/web-assets/images", images)
        .with_entrypoint(vec!["./web-server"]);

    if let Some(tag) = tag {
        container.publish(tag).await?;
        println!("✅ web image published to {tag}");
    } else {
        container.id().await?;
        println!("✅ web container built");
    }

    Ok(())
}

fn postgres_service(client: &Query) -> Service {
    client
        .container()
        .from(POSTGRES_IMAGE)
        .with_env_variable("POSTGRES_PASSWORD", DB_PASSWORD)
        .with_env_variable("POSTGRES_USER", DB_USER)
        .with_env_variable("POSTGRES_DB", DB_NAME)
        .with_exposed_port(5432)
        .as_service()
}

async fn run_migrations(client: &Query, database_url: &str, postgres: &Service) -> Result<()> {
    let migration_root = client.host().directory("crates/db");

    client
        .container()
        .from("ghcr.io/amacneil/dbmate:2")
        .with_workdir("/db")
        .with_directory("/db", migration_root)
        .with_env_variable("DATABASE_URL", database_url)
        .with_service_binding("postgres", postgres.clone())
        .with_exec(vec!["ls", "-l", "/db/migrations"])
        .with_exec(vec![
            "dbmate",
            "-d",
            "/db/migrations",
            "-u",
            database_url,
            "wait",
        ])
        .with_exec(vec![
            "dbmate",
            "-d",
            "/db/migrations",
            "-u",
            database_url,
            "up",
        ])
        .with_exec(vec![
            "dbmate",
            "-d",
            "/db/migrations",
            "-u",
            database_url,
            "status",
        ])
        .with_exec(vec![
            "sh",
            "-c",
            "psql \"$DATABASE_URL\" -Atc \"select to_regclass('public.accounts')\" | grep -q accounts",
        ])
        .sync()
        .await?;

    Ok(())
}

fn default_database_url() -> String {
    format!("postgres://{DB_USER}:{DB_PASSWORD}@postgres:5432/{DB_NAME}?sslmode=disable")
}

fn pipeline_database_url(original: &str) -> String {
    if let Ok(mut parsed) = Url::parse(original) {
        if let Some(host) = parsed.host_str() {
            if matches!(
                host,
                "host.docker.internal" | "localhost" | "127.0.0.1" | "::1"
            ) {
                let _ = parsed.set_host(Some("postgres"));
                // Always point to the service port exposed in the pipeline.
                let _ = parsed.set_port(Some(5432));
            }
        }
        parsed.to_string()
    } else {
        original.to_string()
    }
}
```