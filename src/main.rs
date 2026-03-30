//! # Main Application Entry And System Check
//!
//! This File Is The Starting Point Of The Program.
//! It Checks If Git Is Installed On Your System Before Running.
//! It Automatically Detects Your Linux Distribution To Show The Correct Install Command.
//! It Connects All Modules Together And Starts The Main User Interface.

mod git;
mod menu;
mod system;
mod utils;

use colored::*;
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    if !system::is_git_installed() {
        let install_msg = match system::get_distro_type().as_str() {
            "arch" => "sudo pacman -S git",
            "fedora" => "sudo dnf install git",
            "debian" | "ubuntu" => "sudo apt install git",
            _ => "Please Install Git and Try Again.",
        };

        println!("\n❌ {} is not installed on your system!", "Git".red());
        println!("💡 Please run: {}\n", install_msg.green());
        return;
    }

    menu::menu(VERSION);
}
