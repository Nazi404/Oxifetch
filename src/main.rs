use sysinfo::System;
fn main() {
    let mut system = System::new_all();
    system.refresh_all();

    let os = System::name().unwrap_or("Unknown".to_string());
    let kernel = System::kernel_version().unwrap_or("Unknown".to_string());
    let total_ram = system.total_memory() / 1024 / 1024;
    let used_ram = system.used_memory() / 1024 / 1024;
    let uptime = System::uptime() as f64 / 60.0;
    let cpu_len = system.cpus().len();
    let shell = std::env::var("SHELL").unwrap_or("Unknown".to_string());
    let terminal = std::env::var("TERM").unwrap_or("Unknown".to_string());
    let locale = std::env::var("LANG")
        .or_else(|_|std::env::var("LC_ALL"))
        .or_else(|_|std::env::var("LC_MESSAGES"))
        .unwrap_or("Unknown".to_string());

    println!("OS: {}", os);
    println!("Kernel: {}", kernel);
    println!("Ram {}M / {}M",used_ram,total_ram);
    println!("Uptime: {}", uptime);
    println!("CPUs: {}", cpu_len);
    println!("Shell: {}", shell);
    println!("Terminal: {}", terminal);
    println!("Locale: {}", locale);
}
