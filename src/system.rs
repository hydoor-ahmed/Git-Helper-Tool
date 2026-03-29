use std::process::Command;

pub fn is_git_installed() -> bool {
  Command::new("git").arg("--version").output().is_ok()
}