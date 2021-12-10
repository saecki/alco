use anyhow::{anyhow, bail};
use shellexpand::tilde;
use yaml_rust::YamlLoader;

use std::fs;
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::process::Command;

pub fn reload_kitty(
    kitty_file: impl AsRef<Path>,
    selector: impl AsRef<Path>,
    socket_file: impl AsRef<Path>,
    scheme_file: impl AsRef<str>,
) -> anyhow::Result<()> {
    let selector_str = fs::read_to_string(selector.as_ref())
        .map_err(|_| anyhow!("Error reading kitty selector"))?;
    let selector = YamlLoader::load_from_str(&selector_str)?.remove(0);

    match super::selector(&selector, scheme_file.as_ref()) {
        Some(s) => {
            fs::copy(tilde(s).as_ref(), kitty_file.as_ref())?;

            let socket_file = socket_file.as_ref();
            if Path::exists(socket_file) {
                let unix_stream = UnixStream::connect(socket_file)?;
                let (pid, _, _) = unix_cred::get_peer_pid_ids(&unix_stream)?;
                if let Some(pid) = pid {
                    Command::new("kill").arg("-s").arg("USR1").arg(pid.to_string()).output()?;
                }
            }

            Ok(())
        }
        None => bail!("Missing mapping in kitty selector"),
    }
}
