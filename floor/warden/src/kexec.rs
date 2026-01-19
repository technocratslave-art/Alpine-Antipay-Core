use crate::capsule::Capsule;
use std::process::Command;

pub fn handoff(c: &Capsule) -> Result<(), String> {
    let kernel = format!("{}/{}", c.path, c.kernel);
    let initrd = format!("{}/{}", c.path, c.initrd);
    let cmdline = c.cmdline.clone().unwrap_or_default();

    let status = Command::new("kexec")
        .args([
            "-l",
            &kernel,
            "--initrd",
            &initrd,
            "--command-line",
            &cmdline,
        ])
        .status()
        .map_err(|e| e.to_string())?;

    if !status.success() {
        return Err("kexec -l failed".into());
    }

    Command::new("sync").status().ok();
    Command::new("kexec")
        .arg("-e")
        .status()
        .map_err(|e| e.to_string())?;

    Ok(())
}
