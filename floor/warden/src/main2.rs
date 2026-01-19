mod capsule;
mod console;
mod kexec;
mod trust;

use capsule::Capsule;
use console::*;
use std::fs;

fn main() -> ! {
    banner();

    mount_minimal();

    let capsules = match Capsule::scan("/capsules") {
        Ok(c) if !c.is_empty() => c,
        _ => {
            warn("No valid capsules found");
            bunker();
        }
    };

    let selection = menu(&capsules);
    let chosen = &capsules[selection];

    info(&format!("Selected capsule: {}", chosen.name));

    if !trust::verify_capsule(chosen) {
        error("Capsule signature verification failed");
        bunker();
    }

    info("Loading tenant via kexec");
    if let Err(e) = kexec::handoff(chosen) {
        error(&format!("kexec failed: {e}"));
        bunker();
    }

    unreachable!();
}

fn mount_minimal() {
    use std::process::Command;

    let mounts = [
        ("proc", "/proc", "proc"),
        ("sysfs", "/sys", "sysfs"),
        ("devtmpfs", "/dev", "devtmpfs"),
        ("tmpfs", "/run", "tmpfs"),
    ];

    for (src, tgt, fstype) in mounts {
        let _ = Command::new("mount")
            .args(["-t", fstype, src, tgt])
            .status();
    }
}

fn bunker() -> ! {
    warn("Entering bunker mode");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
