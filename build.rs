use std::process::Command;
use std::fs;

fn main() {
    // Run the PowerShell script to generate Stylance SCSS files
    let script_output = Command::new("stylance")
        .arg(".")
        .arg("--output-dir")
        .arg("./style/")
        .output()
        .expect("Failed to run generate_stylance_files.ps1");

    if !script_output.status.success() {
        panic!("Script execution failed! {:?}", script_output);
    }

    // Copy the necessary file
    fs::copy("./src/style/mixins.scss", "./style/stylance/mixins.scss")
        .expect("Failed to copy the file");
}