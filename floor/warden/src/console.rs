use std::io::{self, Write};

pub fn banner() {
    println!();
    println!("╔══════════════════════════════════╗");
    println!("║   ANTIPAY SOVEREIGN FLOOR        ║");
    println!("║   Rust Warden (PID 1)            ║");
    println!("╚══════════════════════════════════╝");
    println!();
}

pub fn info(msg: &str) {
    println!("[+] {}", msg);
}

pub fn warn(msg: &str) {
    println!("[!] {}", msg);
}

pub fn error(msg: &str) {
    println!("[x] {}", msg);
}

pub fn menu<T: std::fmt::Display>(items: &[T]) -> usize {
    println!("Available capsules:");
    for (i, item) in items.iter().enumerate() {
        println!("  [{}] {}", i, item);
    }

    print!("Select capsule: ");
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();

    input.trim().parse().unwrap_or(0)
}
