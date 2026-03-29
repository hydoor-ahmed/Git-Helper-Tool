
mod menu;
mod system;
mod git;
mod models;

fn main() {
    if !system::is_git_installed() {
        println!("\n❌ Git is not installed on your system!");
        println!("💡 Please run: sudo pacman -S git");
        return;
    }
    
    menu::menu();

}
