use std::{fmt::format, io::{self, Write}};

use crate::git;
use colored::*;
use terminal_size::{Width, terminal_size};

pub fn banner(VERSION: &str) {
    let banner_text = r"
              __________________     ______  __    ______                    
              __  ____/__(_)_  /_    ___  / / /_______  /____________________
              _  / __ __  /_  __/    __  /_/ /_  _ \_  /___  __ \  _ \_  ___/
              / /_/ / _  / / /_      _  __  / /  __/  / __  /_/ /  __/  /    
              \____/  /_/  \__/      /_/ /_/  \___//_/  _  .___/\___//_/     
                                                        /_/ @7yd.o           
    ";

    // 1. نجيب عرض التيرمنال الحالي
    let terminal_width = if let Some((Width(w), _)) = terminal_size() {
        w as usize
    } else {
        80 // قيمة افتراضية إذا فشل التحسس
    };

    // 2. نطبع كل سطر بالبنر وهو موصط
    for line in banner_text.lines() {
        let line_len = line.len();
        if line_len < terminal_width {
            // نحسب الفراغ المطلوب من جهة اليسار
            let padding = (terminal_width - line_len) / 2;
            println!("{}{}", " ".repeat(padding), line.cyan().bold());
        } else {
            println!("{}", line.cyan().bold());
        }
    }

    let line1 = format!("=== Git Helper Tool v{} | By Error404 ===", VERSION);
    let line2 = "-------------------------------------------";

    // 4. حساب المسافة (Padding) لكل سطر وطباعته بلونه الخاص
    let p1 = (terminal_width.saturating_sub(line1.len())) / 2;
    println!("{}{}", " ".repeat(p1), line1.green().bold());

    let p2 = (terminal_width.saturating_sub(line2.len())) / 2;
    println!("{}{}", " ".repeat(p2), line2.black().bold());
    
    println!(); // سطر فارغ للترتيب
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("❌ Field To Read.");

    input.trim().to_string()
}

fn pause() {
  print!("\n⌨️ Press Enter to continue...");
  let _ = io::stdout().flush();
  let _ = io::stdin().read_line(&mut String::new());
}

pub fn menu(VERSION: &str) {
    loop {
        let mut input = String::new();

        clear_screen();
        banner(VERSION);

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
                banner(VERSION);
                let repo_url = get_input("🔗 Repo Url: ");
                let repo_commit_message = get_input("💬 Commit Message: ");

                if git::create_new_repo(&repo_url, &repo_commit_message) {
                    println!("✅ Everything Uploaded Successfully!")
                } else {
                    println!("\n⚠️ Something Went Wrong During the Process.");
                }
                pause();
            }
            2 => {
                clear_screen();
                banner(VERSION);
                let repo_commit_message = get_input("💬 Commit Message: ");

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
