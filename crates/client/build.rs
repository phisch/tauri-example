use std::process::Command;

fn main() {
    let frontend_directory = "../frontend";
    println!("cargo:rerun-if-changed={}", frontend_directory);

    Command::new("trunk")
        .current_dir(frontend_directory)
        .args(["build", "--release"])
        .spawn()
        .unwrap_or_else(|err| panic!("Failed to execute the frontend build: {}", err));

    tauri_build::build();
}
