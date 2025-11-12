use super::{
    apply,
    init::{ensure_namespace, ensure_stackapp_crd},
};
use crate::operator::crd::StackApp;
use anyhow::{anyhow, Context, Result};
use kube::{Client, ResourceExt};
use std::fs;

pub async fn install(installer: &crate::cli::Installer) -> Result<()> {
    println!("🔌 Connecting to the cluster...");
    let client = Client::try_default().await?;
    println!("✅ Connected");

    let manifest_raw = fs::read_to_string(&installer.manifest).with_context(|| {
        format!(
            "Failed to read manifest at {}",
            installer.manifest.display()
        )
    })?;

    let stack_app: StackApp =
        serde_yaml::from_str(&manifest_raw).context("Failed to parse StackApp manifest")?;

    let namespace = stack_app
        .namespace()
        .ok_or_else(|| anyhow!("StackApp manifest is missing metadata.namespace"))?;

    let app_name = stack_app.name_any();

    ensure_stackapp_crd(&client).await?;
    ensure_namespace(&client, &namespace).await?;

    apply::apply(&client, &manifest_raw, None)
        .await
        .context("Failed to apply StackApp manifest")?;

    println!(
        "🚀 Applied StackApp `{}` in namespace `{}`",
        app_name, namespace
    );

    Ok(())
}
