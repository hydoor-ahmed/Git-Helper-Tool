use std::process::Command;
use std::fs;

pub fn is_git_installed() -> bool {
  Command::new("git").arg("--version").output().is_ok()
}

pub fn get_distro_type() -> String {
  let os_info = fs::read_to_string("/etc/os-release").unwrap_or_default();

  if os_info.contains("ID=arch") || os_info.contains("ID_LIKE=arch") {
    "💡 Please run: sudo pacman -S git".to_string()
  } else if os_info.contains("ID=debian") || os_info.contains("ID_LIKE=debian") {
    "💡 Please run: sudo apt install git".to_string()
  } else {
    "Please Install git.".to_string()
  }
}