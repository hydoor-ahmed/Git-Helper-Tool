
mod menu;
mod system;
mod git;

fn main() {
    if !system::is_git_installed() {
        menu::clear_screen();
        menu::banner();
        println!("\n❌ Git is not installed on your system!");
        println!("{}", system::get_distro_type());
        return;
    }
    
    menu::menu();

}
