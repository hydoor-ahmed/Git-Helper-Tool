use std::io::{self, Write};

use crate::git;

pub fn banner() {
    const BANNER: &str = r"
              __________________     ______  __    ______                    
              __  ____/__(_)_  /_    ___  / / /_______  /____________________
              _  / __ __  /_  __/    __  /_/ /_  _ \_  /___  __ \  _ \_  ___/
              / /_/ / _  / / /_      _  __  / /  __/  / __  /_/ /  __/  /    
              \____/  /_/  \__/      /_/ /_/  \___//_/  _  .___/\___//_/     
                                                        /_/ @7yd.o           
  ";
    println!("{}", BANNER);
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn new_repo() -> bool {
    clear_screen();
    banner();

    let mut repo_url = String::new();
    let mut repo_commit_message = String::new();

    print!("\n🔗 Repo Url: ");
    io::stdout().flush().expect("Buffer Error.");
    io::stdin().read_line(&mut repo_url).expect("Try Again.");

    print!("\n💬 Commit Message: ");
    io::stdout().flush().expect("Buffer Error.");
    io::stdin()
        .read_line(&mut repo_commit_message)
        .expect("Try Again.");

    git::create_new_repo(&repo_url.trim(), &repo_commit_message.trim())
}

fn update_repo() -> bool {
    clear_screen();
    banner();

    let mut repo_commit_message = String::new();

    print!("\n💬 Commit Message: ");
    io::stdout().flush().expect("Buffer Error.");
    io::stdin()
        .read_line(&mut repo_commit_message)
        .expect("Try Again.");

    git::fast_push(&repo_commit_message.trim())
}

pub fn menu() {
    loop {
        let mut input = String::new();

        clear_screen();
        banner();

        print!("\n1. New Repo.\n2. Update Exist Repo.\n0. Exit.\n> ");
        io::stdout().flush().expect("Buffer Error.");
        io::stdin().read_line(&mut input).expect("Try Again.");

        let input: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Enter Valid Number.");
                continue;
            }
        };

        match input {
            1 => {
                if new_repo() {
                    println!("✅ Everything uploaded successfully!")
                } else {
                    println!("\n⚠️ Something went wrong during the process.");
                }
            }
            2 => {
                if update_repo() {
                    println!("✅ Everything uploaded successfully!")
                } else {
                    println!("\n⚠️ Something went wrong during the process.");
                }
            }
            0 => {
                println!("\nSee You Later 👋🏼");
                break;
            }
            _ => {
                print!("Please Enter Valid Choice.");
            }
        }
    }
}
