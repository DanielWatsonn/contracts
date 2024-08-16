use std::process::{Command, ExitStatus};

// Removes the husky configuration from .git/config if it exists.
fn main() {
    match unset_git_config("core.hooksPath") {
        Ok(()) => println!("Successfully removed core.hooksPath configuration."),
        Err(e) => eprintln!("Failed to remove core.hooksPath configuration: {}", e),
    }
}

/// Unsets a git configuration key.
fn unset_git_config(key: &str) -> Result<(), String> {
    let status: ExitStatus = Command::new("git")
        .arg("config")
        .arg("--unset")
        .arg(key)
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Failed to unset '{}'. Exit status: {:?}", key, status))
    }
}
