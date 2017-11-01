use std::process::{self, Command, ExitStatus, Output};

// Move a file (just an mv alias)
// fn mv_file(source_name: &str, destination_name: &str) {
//     let output = process::Command::new("mv")
//         .arg(source_name)
//         .arg(destination_name)
//         .output()
//         .expect("failed to execute process");
//     println!("status: {} stderr: {} stdout: {}",
//              output.status,
//              String::from_utf8_lossy(&output.stderr),
//              String::from_utf8_lossy(&output.stdout));
// }

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
    exec_shell("rm -rf css js");
    // Command::new("rm")
    //     .current_dir("./")
    //     .args(vec!["-rf", "css"]);
    println!("Deleting stale css directory.");

    exec_shell("rm -rf css js");
    // Command::new("rm")
    //     .current_dir("./")
    //     .args(vec!["-rf", "js"]);
    println!("Deleting stale js directory.");

    exec_shell("cd ~/rust/tuxedo/public && mv css ~/rust/tuxedo");
    // Command::new("mv")
    //     .current_dir("./public")
    //     .args(vec!["css", "../"])
    //     .status()
    //     .unwrap();
    println!("Moving new stylesheet folder to root.");

    exec_shell("cd ~/rust/tuxedo/public && mv js ~/rust/tuxedo");
    // Command::new("mv")
    //     .current_dir("./public")
    //     .args(vec!["js", "../"])
    //     .status()
    //     .unwrap();
    println!("Moving new script folder to root.");

    exec_shell("cd ~/rust/tuxedo/public && mv index.html ~/rust/tuxedo/views/index.html.hbs");
    // Command::new("mv")
    //     .current_dir("./public")
    //     .args(vec!["index.html", "../views/index.html.hbs"])
    //     .status()
    //     .unwrap();
    println!("Replacing index handlebars in view folder.");
}
