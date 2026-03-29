use std::{process::Command, vec};

pub fn run_git_sequence(commands: Vec<Vec<String>>) -> bool {
    for args in commands {
        println!("🚀 Running: git {}...", args.join(" "));

        let status = Command::new("git")
            .args(&args)
            .status()
            .expect("Field To Execute Git Command");

        if !status.success() {
          println!("❌ Error: Command 'git {:?}' Field.", args);
          return false;
        }
    }
  
  true
}

pub fn create_new_repo(url: &str, message: &str) -> bool {
  let commands = vec![
    vec!["init".to_string(), ".".to_string()],
    vec!["add".to_string(), ".".to_string()],
    vec!["commit".to_string(), "-m".to_string(), message.to_string()],
    vec!["branch".to_string(), "-M".to_string(), "main".to_string()],
    vec!["remote".to_string(), "add".to_string(), "origin".to_string(), url.to_string()],
    vec!["push".to_string(), "-u".to_string(), "origin".to_string(), "main".to_string()],
  ];

  run_git_sequence(commands)
}

pub fn fast_push(message: &str) -> bool {
  let commands = vec![
    vec!["add".to_string(), ".".to_string()],
    vec!["commit".to_string(), "-m".to_string(), message.to_string()],
    vec!["push".to_string(), "origin".to_string(), "main".to_string()],
  ];

  run_git_sequence(commands)
}
