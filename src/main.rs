use std::process::Command;

mod request;

fn main() {
    let output = Command::new("git")
        .arg("diff")
        .arg("--cached")
        .output()
        .expect("Failed to execute git diff");

    let diff = String::from_utf8_lossy(&output.stdout);
    let message = request::request(diff.to_string()); 

    match message {
        Ok(msg) => {
            let _ = Command::new("git")
                .arg("commit")
                .arg("-m")
                .arg(msg)
                .spawn()
                .expect("Failed to execute git commit")
                .wait()
                .expect("Failed to wait on git commit");
        }
        Err(e) => {
            eprintln!("Error generating commit message: {}", e);
            std::process::exit(1);
        }
    }
}