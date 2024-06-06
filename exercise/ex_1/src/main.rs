
// Write a Rust program that gathers system information such as the Rust version, operating system details, and system architecture

use std::process::Command;

fn main() {
    

    // Rust version
    let version_output = Command::new("rustc")
    .arg("--version")
    .output()
    .expect("failed to execute process");
let rust_version = String::from_utf8_lossy(&version_output.stdout);

    println!("Rust version is {}",rust_version);

    println!("--------------------------------");
    // Os version
    let os_version = Command::new("cmd").output().expect("Failed to execute process");
    let os_version = String::from_utf8_lossy(&os_version.stdout);
    println!("OS version is : {}", os_version);

    println!("--------------------------------");

    // System architecture
    let os_architecture =  Command::new("wmic")
    .args(&["OS", "get", "OSArchitecture"])
    .output().expect("Failed to execute process");

      let os_architecture = String::from_utf8_lossy(&os_architecture.stdout);
    println!("OS architecture is : {}", os_architecture);
    
}
