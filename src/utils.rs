//! # Utility Support And Visual Formatting Module
//!
//! This File Contains Helper Functions To Enhance The User Interface.
//! It Handles The ASCII Banner Rendering And Terminal Width Calculation.
//! It Manages The Status Dashboard To Show RealTime Git Information.
//! It Includes Tools For Text Truncation And Random Security Tip Generation.

use crate::git;
use colored::Colorize;
use rand::seq::IndexedRandom;
use terminal_size::{Width, terminal_size};

pub fn banner(version: &str) {
    let banner_text = r"
              __________________     ______  __    ______                    
              __  ____/__(_)_  /_    ___  / / /_______  /____________________
              _  / __ __  /_  __/    __  /_/ /_  _ \_  /___  __ \  _ \_  ___/
              / /_/ / _  / / /_      _  __  / /  __/  / __  /_/ /  __/  /    
              \____/  /_/  \__/      /_/ /_/  \___//_/  _  .___/\___//_/     
                                                        /_/ @7yd.o           

";

    let terminal_width = if let Some((Width(w), _)) = terminal_size() {
        w as usize
    } else {
        80
    };

    // * Banner
    for line in banner_text.lines() {
        let line_len = line.len();
        if line_len < terminal_width {
            let padding = (terminal_width - line_len) / 2;
            println!("{}{}", " ".repeat(padding), line.cyan().bold());
        } else {
            println!("{}", line.cyan().bold());
        }
    }

    let line1 = format!("=== Git Helper Tool v{} | By Error404 ===", version);
    let line2 = "-------------------------------------------";
    let line3 = get_random_hint();

    let p1 = (terminal_width.saturating_sub(line1.len())) / 2;
    println!("{}{}", " ".repeat(p1), line1.green().bold());

    let p2 = (terminal_width.saturating_sub(line2.len())) / 2;
    println!("{}{}", " ".repeat(p2), line2.black().bold());

    let p3 = (terminal_width.saturating_sub(line3.len())) / 2;
    println!("{}{}", " ".repeat(p3), line3.bright_purple());

    // * Status Dashboard Box
    for line in status_dashboard().lines() {
        let line_len = line.chars().count();
        if line_len < terminal_width {
            let padding = (terminal_width - line_len) / 2;
            println!("{}{}", " ".repeat(padding), line.cyan().bold());
        } else {
            println!("{}", line.cyan().bold());
        }
    }

    println!();
}

pub fn get_random_hint() -> &'static str {
    let hints = [
        // * Advanced Git Workflow
        "💡 Hint: Use 'git commit --amend' to fix that annoying typo in your last message.",
        "💡 Hint: 'git push --force-with-lease' is the safer way to force push. Protect your history!",
        "💡 Hint: Use 'git stash' to save your work-in-progress and switch branches quickly.",
        "💡 Hint: 'git log --graph --oneline --all' shows your project history like a subway map.",
        "💡 Hint: Try 'git clean -fd' to remove all untracked files and folders in a blink.",
        // * Cybersecurity & Pro Dev
        "🛡️ Security: A leaked API key can cost thousands. Always use '.env' and '.gitignore'.",
        "🛡️ Security: Periodically run 'npm audit' or 'cargo audit' to find vulnerable packages.",
        "🛡️ Security: Never trust user input. Always sanitize data before saving to MongoDB.",
        "🛡️ Security: Use SSH with a Passphrase. It's an extra layer of defense for your keys.",
        "🛡️ Security: Check your 'authorized_keys' file regularly for any unknown entries.",
        // * Engineering
        "🚀 Tip: Write code for the next developer, even if that developer is 'future you'.",
        "🚀 Tip: Small, frequent commits are much easier to debug than one giant 'mega-commit'.",
        "🚀 Tip: Use descriptive commit messages. 'Fixed stuff' is the enemy of clarity.",
        "🚀 Tip: Automation is key. If you do it more than thrice, write a script for it!",
        "🚀 Tip: Keep your 'main' branch deployable at all times. Experiment in feature branches.",
    ];

    let mut rng = rand::rng();
    hints.choose(&mut rng).unwrap_or(&"Keep coding! 🚀")
}

pub fn status_dashboard() -> String {
    // * Don't Touch The Spaces in This Box :)
    format!(
        "
  ┌──────────────── Status Dashboard ────────────────┐
🌿 Branch: {}.           📝 Changes: {}
 🕒 Last Commit: {}
  └──────────────────────────────────────────────────┘",
        git::get_branch(),
        git::get_changes(),
        git::get_commit()
    )
}

pub fn truncate_text(text: &str, max_width: usize) -> String {
    let char_count = text.chars().count();

    if char_count <= max_width {
        return text.to_string();
    }

    let truncated: String = text.chars().take(max_width - 3).collect(); // * - 3 for => ...

    format!("{}...", truncated)
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}
