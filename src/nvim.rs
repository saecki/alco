use async_std::task::spawn;
use nvim_rs::create::async_std::new_unix_socket;
use nvim_rs::rpc::handler::Dummy;

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub async fn reload_neovim(file: impl AsRef<Path>) -> anyhow::Result<()> {
    let instances: Vec<_> = fs::read_dir("/tmp")?
        .into_iter()
        .filter_map(Result::ok)
        .filter(|d| d.metadata().map(|m| m.is_dir()).unwrap_or(false))
        .filter(|d| {
            d.file_name()
                .to_str()
                .map(|s| s.starts_with("nvim"))
                .unwrap_or(false)
        })
        .map(|d| d.path().join("0"))
        .collect();

    if instances.is_empty() {
        return Ok(());
    }

    let file = Arc::new(file.as_ref().to_owned());
    reload_instances(instances, file).await?;

    Ok(())
}

async fn reload_instances(instances: Vec<PathBuf>, file: Arc<PathBuf>) -> anyhow::Result<()> {
    let tasks = instances
        .into_iter()
        .map(|p| {
            let f = Arc::clone(&file);

            spawn(async move {
                let (nvim, j) = new_unix_socket(&p, Dummy::new()).await?;
                nvim.command(&format!("source {}", f.display())).await?;
                nvim.command("redraw!").await?;
                nvim.command("redrawstatus!").await?;
                nvim.command("redrawtabline").await?;
                nvim.command("silent! AirlineRefresh").await?;
                j.cancel().await;

                Ok::<(), anyhow::Error>(())
            })
        })
        .collect::<Vec<_>>();

    for t in tasks.into_iter() {
        t.await?;
    }

    Ok(())
}
