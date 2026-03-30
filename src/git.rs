use std::{fmt::format, process::Command};

use crate::utils;

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
        vec![
            "remote".to_string(),
            "add".to_string(),
            "origin".to_string(),
            url.to_string(),
        ],
        vec![
            "push".to_string(),
            "-u".to_string(),
            "origin".to_string(),
            "main".to_string(),
        ],
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

pub fn get_branch() -> String {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .expect("Field To Execute Git Command.");

    if output.status.success() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        "Not a Git Repo.".to_string()
    }
}

pub fn get_commit() -> String {
    let output = Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--format=%s (%cr)")
        .output()
        .expect("Field To Execute Git Command.");

    if output.status.success() {
        let commit_output = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if commit_output.is_empty() {
            "No Commits Yet!".to_string()
        } else {
          utils::truncate_text(&commit_output, 34)
        }
    } else {
        "Not a Git Repo or No Commits.".to_string()
    }
}

pub fn get_changes() -> String {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()
        .expect("Field To Execute Git Command.");

    if output.status.success() {
        let changes_output = String::from_utf8_lossy(&output.stdout);
        let count = changes_output.lines().count();

        if count == 0 {
            "Clean ✅".to_string()
        } else {
            format!("{} Files", count)
        }
    } else {
        "Not a Git Repo or No Commits.".to_string()
    }
}

