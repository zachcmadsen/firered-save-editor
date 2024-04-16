use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=.git/HEAD");

    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    let commit_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=COMMIT_HASH={}", commit_hash);
}
