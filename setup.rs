use std::process::{Command, ExitStatus, Output};

pub fn exec_shell(arg: &str) -> Output {
    Command::new("sh")
        .arg("-c")
        .arg(arg)
        .output()
        .expect("failed to execute process")
}

pub fn exec_shell_with_output(arg: &str) -> ExitStatus {
    Command::new("sh")
        .arg("-c")
        .arg(arg)
        .status()
        .expect("failed to execute process")
}

fn main() {
    println!("cargo:rerun-if-changed=./app/elm/Main.elm");
    println!("cargo:rerun-if-changed=./app/static/index.html.hbs");
    println!("Running brunch builder...");

    let brunch = Command::new("brunch")
        .current_dir("app/")
        .arg("build")
        .output()
        .expect("Failed to execute brunch build process.");

    // status exit code 0 for success
    println!("status: {}", brunch.status);
    println!("stdout: {}", String::from_utf8_lossy(&brunch.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&brunch.stderr));

    exec_shell("rm -rf css js");
    println!("Deleted stale css directory.");

    exec_shell("rm -rf css js");
    println!("Deleted stale js directory.");

    exec_shell("cd ~/rust/tuxedo/public && mv css ~/rust/tuxedo");
    println!("Moved brunch stylesheet folder to root.");

    exec_shell("cd ~/rust/tuxedo/public && mv js ~/rust/tuxedo");
    println!("Moved brunch script folder to root.");

    exec_shell("cd ~/rust/tuxedo/public && mv index.html ~/rust/tuxedo/views/index.html.hbs");
    println!("Replaced index handlebar template in view folder.");
}
