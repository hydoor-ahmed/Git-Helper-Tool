use std::fmt::format;

use rand::seq::{IndexedRandom};

use crate::git;

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
    "🚀 Tip: Keep your 'main' branch deployable at all times. Experiment in feature branches."
];

  let mut rng = rand::rng();
  hints.choose(&mut rng).unwrap_or(&"Keep coding! 🚀")
}

pub fn status_dashboard() -> String {
  format!("
  ┌──────────────── Status Dashboard ────────────────┐
🌿 Branch: {}.           📝 Changes: {}
 🕒 Last Commit: {}
  └──────────────────────────────────────────────────┘", git::get_branch(), git::get_changes(), git::get_commit())
}

pub fn truncate_text(text: &str, max_width: usize) -> String {
  let char_count = text.chars().count();

  if char_count <= max_width {
    return text.to_string();
  }

  let truncated: String = text.chars()
  .take(max_width - 3)
  .collect(); // * - 3 for => ...

  format!("{}...", truncated)
}
