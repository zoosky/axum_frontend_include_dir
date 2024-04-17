use std::process::Command;

fn main() {
    let output = Command::new("npm")
        .args(["run", "build"])
        .current_dir("frontend")
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        panic!("Command executed with failing error code");
    }
}