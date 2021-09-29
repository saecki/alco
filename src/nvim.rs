use async_std::task::spawn;
use nvim_rs::create::async_std::new_path;
use nvim_rs::rpc::handler::Dummy;

use std::fs;
use std::path::PathBuf;

pub async fn reload_neovim() -> anyhow::Result<()> {
    let instances: Vec<_> = fs::read_dir("/tmp")?
        .into_iter()
        .filter_map(Result::ok)
        .filter(|d| d.metadata().map(|m| m.is_dir()).unwrap_or(false))
        .filter(|d| d.file_name().to_str().map(|s| s.starts_with("nvim")).unwrap_or(false))
        .map(|d| d.path().join("0"))
        .collect();

    if instances.is_empty() {
        return Ok(());
    }

    reload_instances(instances).await?;

    Ok(())
}

async fn reload_instances(instances: Vec<PathBuf>) -> anyhow::Result<()> {
    let tasks = instances
        .into_iter()
        .map(|p| {
            spawn(async move {
                let (nvim, j) = new_path(&p, Dummy::new()).await?;
                nvim.command("lua require(\"colors\").setup()").await?;
                nvim.command("redraw!").await?;
                nvim.command("redrawstatus!").await?;
                nvim.command("redrawtabline").await?;
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
