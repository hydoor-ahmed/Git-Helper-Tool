use std::io::{self, Write};

use crate::{git, utils};
use colored::*;
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
    let line3 = utils::get_random_hint();

    let p1 = (terminal_width.saturating_sub(line1.len())) / 2;
    println!("{}{}", " ".repeat(p1), line1.green().bold());

    let p2 = (terminal_width.saturating_sub(line2.len())) / 2;
    println!("{}{}", " ".repeat(p2), line2.black().bold());

    let p3 = (terminal_width.saturating_sub(line3.len())) / 2;
    println!("{}{}", " ".repeat(p3), line3.bright_purple());

    for line in utils::status_dashboard().lines() {
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

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn get_input(prompt: &str) -> Option<String> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("❌ Field To Read.");

    if input.trim() == "0" {
        None
    } else {
        Some(input.trim().to_string())
    }
}

fn pause() {
    print!("\n⌨️ Press Enter to continue...");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut String::new());
}

pub fn menu(version: &str) {
    loop {
        let mut input = String::new();

        clear_screen();
        banner(version);

        print!("\n1. New Repo 🗞️\n2. Update Exist Repo ♻️\n0. Exit 🚪\n> ");
        io::stdout().flush().expect("Buffer Error.");
        io::stdin().read_line(&mut input).expect("Try Again.");

        let input: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Please Enter Valid Number.");
                continue;
            }
        };

        match input {
            1 => {
                clear_screen();
                banner(version);
                println!("{}", "Type '0' to Back.".black().bold().underline());
                let repo_url = match get_input("🔗 Repo Url: ") {
                    Some(url) => url,
                    None => continue,
                };

                let repo_commit_message = match get_input("💬 Commit Message: ") {
                    Some(text) => text,
                    None => continue,
                };

                if git::create_new_repo(&repo_url, &repo_commit_message) {
                    println!("✅ Everything Uploaded Successfully!")
                } else {
                    println!("\n⚠️ Something Went Wrong During the Process.");
                }
                pause();
            }
            2 => {
                clear_screen();
                banner(version);
                println!("{}", "Type '0' to Back.".black().bold().underline());

                let repo_commit_message = match get_input("💬 Commit Message: ") {
                    Some(text) => text,
                    None => continue,
                };

                if git::fast_push(&repo_commit_message) {
                    println!("✅ Everything Uploaded Successfully!")
                } else {
                    println!("\n⚠️ Something Went Wrong During the Process.");
                }
                pause();
            }
            0 => {
                println!("\nSee You Later 👋🏼");
                break;
            }
            _ => {
                print!("❌ Please Enter Valid Choice.");
            }
        }
    }
}
