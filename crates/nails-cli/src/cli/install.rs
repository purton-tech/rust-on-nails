use anyhow::Result;
use kube::Client;

pub async fn install(_installer: &crate::cli::Installer) -> Result<()> {
    println!("🔌 Connecting to the cluster...");
    let _client = Client::try_default().await?;
    println!("✅ Connected");

    Ok(())
}
