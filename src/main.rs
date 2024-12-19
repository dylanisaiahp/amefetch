use colored::*;
use std::process::Command;
use sysinfo::System;
use users::get_current_username;

fn main() {
    // Initialize system and refresh data
    let mut sys = System::new_all();
    sys.refresh_all();

    // Tags
    let tags = create_tags(&[
        "OS:", "Kernel:", "Uptime:", "CPU:", "GPU:", "Memory:", "Swap:",
    ]);

    // User and Host
    let user_name = get_current_username()
        .and_then(|os_str| os_str.into_string().ok())
        .unwrap_or_else(|| "Unknown".to_string())
        .purple()
        .bold();
    let host_name = System::host_name()
        .unwrap_or_else(|| "Unknown".to_string())
        .purple()
        .bold();

    // System Info
    let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
    let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
    let arch_type = System::cpu_arch();
    let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
    let cpu = sys.cpus().get(0).map(|c| c.brand()).unwrap_or("Unknown");
    let uptime = format_uptime(System::uptime());
    let gpu = get_gpu();

    // Memory Info
    let total_mem = format_gb(sys.total_memory());
    let used_mem = format_gb(sys.used_memory());
    let total_swap = format_gb(sys.total_swap());
    let used_swap = format_gb(sys.used_swap());

    // Display Output
    println!("{}{}{}", user_name, "@".purple().bold(), host_name);
    println!("{}", "          ".strikethrough());
    println!("{} {} {} {}", tags["OS:"], os_name, os_version, arch_type);
    println!("{} {}", tags["Kernel:"], kernel_version);
    println!("{} {}", tags["Uptime:"], uptime);
    println!("{} {}", tags["CPU:"], cpu);
    println!("{} {}", tags["GPU:"], gpu);
    println!("{} {} / {}", tags["Memory:"], used_mem, total_mem);
    println!("{} {} / {}", tags["Swap:"], used_swap, total_swap);
}

// Helper to create colored tags
fn create_tags(labels: &[&str]) -> std::collections::HashMap<String, colored::ColoredString> {
    labels
        .iter()
        .map(|&label| (label.to_string(), label.to_string().purple().bold()))
        .collect()
}

// Helper to format uptime
fn format_uptime(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    format!("{} hours, {} minutes, {} seconds", hours, minutes, seconds)
}

// Helper to format memory
fn format_gb(bytes: u64) -> String {
    let gb = bytes as f64 / 1024.0 / 1024.0 / 1024.0; // Convert bytes to GB
    format!("{:.2} GB", gb) // 2 decimal places for clarity
}

// Helper to get GPU information
fn get_gpu() -> String {
    let output = Command::new("lspci")
        .output()
        .expect("Failed to execute lspci.");

    let output_str = String::from_utf8(output.stdout).unwrap();
    let err_str = String::from_utf8(output.stderr).unwrap();

    if !err_str.is_empty() {
        return format!("Error: {}", err_str);
    }

    let gpu_info = output_str
        .lines()
        .filter(|line| line.contains("VGA") || line.contains("3D"))
        .map(|line| {
            line.split(':')
                .nth(2)
                .map(|model| model.trim().to_string())
                .unwrap_or_else(|| "Unknown GPU".to_string())
        })
        .collect::<Vec<_>>()
        .join("\n");

    if gpu_info.is_empty() {
        "No GPU found.".to_string()
    } else {
        gpu_info
    }
}
