use std::process::Command;

fn main() {
    println!(
        "Hunter: {}",
        String::from_utf8(Command::new("id").output().unwrap().stdout).unwrap()
    );
}
