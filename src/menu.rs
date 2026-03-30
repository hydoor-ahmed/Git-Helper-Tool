//! # User Interface And Menu Navigation System
//!
//! This File Handles Everything The User Sees And Interacts With.
//! It Displays The Main Banner And The Dashboard For Git Operations.
//! It Manages User Input And Allows Navigating Between Different Features.
//! It Includes Functions To Clear The Screen And Handle Back Navigation Safely.


use std::io::{self, Write};
use crate::{git, utils};
use colored::Colorize;

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

        utils::clear_screen();
        utils::banner(version);

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
                utils::clear_screen();
                utils::banner(version);
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
                utils::clear_screen();
                utils::banner(version);
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
