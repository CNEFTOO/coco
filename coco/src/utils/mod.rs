use colored::*;

mod cel;
mod log;
pub use cel::*;
// Remove unused import


pub fn info(message: &str) {
    println!("[*] {}", message.bright_blue());
}

pub fn error(message: &str) {
    println!("[-] {}", message.bright_red());
}

pub fn success(message: &str) {
    println!("[+] {}", message.bright_green());
}

pub fn debug(message: &str) {
    println!("[D] {}", message.bright_yellow());
}

pub fn warn(message: &str) {
    println!("[!] {}", message.bright_yellow());
}

pub const DEFAULT_THREADS: usize = 10;
pub const DEFAULT_TIMEOUT: u32 = 30;

pub fn show_banner() {
    let banner = r#"
     ██████╗ ██████╗  ██████╗ ██████╗ 
    ██╔════╝██╔═══██╗██╔════╝██╔═══██╗
    ██║     ██║   ██║██║     ██║   ██║
    ██║     ██║   ██║██║     ██║   ██║
    ╚██████╗╚██████╔╝╚██████╗╚██████╔╝
     ╚═════╝ ╚═════╝  ╚═════╝ ╚═════╝ 
    "#;
    println!("{}", banner.bright_cyan());
    println!("{}", "一个基于CEL的漏洞检测工具".bright_yellow());
    println!();
}