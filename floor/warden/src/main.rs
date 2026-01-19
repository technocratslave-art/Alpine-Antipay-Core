use std::fs;
use std::process::Command;

fn sh(cmd: &str, args: &[&str]) {
    let ok = Command::new(cmd).args(args).status().ok().map(|s| s.success()).unwrap_or(false);
    if !ok {
        eprintln!("[floor] FAIL: {} {:?}", cmd, args);
        halt();
    }
}

fn write(path: &str, s: &str) {
    if fs::write(path, s).is_err() {
        eprintln!("[floor] FAIL: write {}", path);
        halt();
    }
}

fn halt() -> ! {
    // Try to sync, then halt hard.
    let _ = Command::new("sync").status();
    let _ = Command::new("poweroff").arg("-f").status();
    let _ = Command::new("halt").arg("-f").status();
    loop {}
}

fn main() {
    eprintln!("[floor] antipay-floor pid1 starting");

    // Minimal mounts
    sh("mount", &["-t", "proc", "proc", "/proc"]);
    sh("mount", &["-t", "sysfs", "sysfs", "/sys"]);
    sh("mount", &["-t", "devtmpfs", "devtmpfs", "/dev"]);
    sh("mount", &["-t", "tmpfs", "tmpfs", "/run"]);

    // Hard bans: no network. If you want, you can also remove modules for net entirely.
    // (Floor should ship without ip/ifconfig; keep it boring.)

    // Example: consume Floor facts (for now, unsigned placeholder)
    // Later: replace with CBOR + Ed25519 verify + rollback checks.
    let facts_path = "/run/floor/facts.json";
    fs::create_dir_all("/run/floor").ok();
    write(
        facts_path,
        r#"{"form_factor":"tablet","arch":"aarch64","platform_id":"dev-dummy"}"#,
    );
    eprintln!("[floor] facts written to {}", facts_path);

    // Load kexec target (dummy tenant kernel+initrd paths staged by build)
    // For bring-up: place these at /tenant/vmlinuz and /tenant/initrd in initramfs.
    // Later: they come from a signed capsule selection.
    if !fs::metadata("/tenant/vmlinuz").is_ok() {
        eprintln!("[floor] missing /tenant/vmlinuz");
        halt();
    }
    if !fs::metadata("/tenant/initrd").is_ok() {
        eprintln!("[floor] missing /tenant/initrd");
        halt();
    }

    // kexec into Tenant
    eprintln!("[floor] loading tenant via kexec");
    sh(
        "kexec",
        &[
            "-l",
            "/tenant/vmlinuz",
            "--initrd=/tenant/initrd",
            "--command-line=console=tty0 floor_facts=/run/floor/facts.json",
        ],
    );

    eprintln!("[floor] executing kexec (Floor disappears now)");
    sh("sync", &[]);
    sh("kexec", &["-e"]);

    halt();
}
