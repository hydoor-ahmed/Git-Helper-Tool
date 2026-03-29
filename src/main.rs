
mod menu;
mod system;
mod git;

use colored::*;
pub const VERSION: &str = "1.0";

fn main() {
    if !system::is_git_installed() {
        menu::clear_screen();
        menu::banner(VERSION);

        let install_msg = match system::get_distro_type().as_str() {
            "arch" => "sudo pacman -S git",
            "fedora" => "sudo dnf install git",
            "debian" | "ubuntu" => "sudo apt install git",
            _ => "Please Install Git and Try Again."
        };

        println!("\n❌ {} is not installed on your system!", "Git".red());
        println!("💡 Please run: {}\n", install_msg.green());
        return;
    }
    
    menu::menu(VERSION);

}
