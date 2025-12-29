use crate::structs::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn run(app: App) {
        let dest = app.path;
        // println!("Launching {} with dest '{}'", app.name.as_str(), dest);
        if dest.ends_with(".AppImage") {
                run_appimage(&dest).unwrap();
                // println!("trying to run appimage")
        } else if dest.ends_with(".sh") {
                run_bash(&dest).unwrap();
                // println!("trying to run bash script")
        } else if dest.ends_with(".desktop") {
                run_desktop(&dest).unwrap();
                // println!("trying to run .desktop")
        } else {
                run_misc(&dest).unwrap();
                // println!("trying to run misc")
        }
}

fn run_misc(dest: &str) -> Result<(), Box<dyn std::error::Error>> {
        let _ = Command::new(&dest)
                .stdout(Stdio::null()) // ignore stdout
                .stderr(Stdio::null()) // ignore stderr
                .spawn()?; // launches asynchronously
        Ok(())
}

fn run_bash(dest: &str) -> Result<(), Box<dyn std::error::Error>> {
        let _ = Command::new("bash")
                .arg(dest)
                .stdout(Stdio::null()) // ignore stdout
                .stderr(Stdio::null()) // ignore stderr
                .spawn()?; // launches asynchronously
        Ok(())
}

fn run_appimage(app_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let _path = Path::new(&app_path);

        let _child = Command::new(app_path).spawn()?; // launches asynchronously
        Ok(())
}

fn run_desktop(desktop_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(desktop_path);
        // if !path.exists() {
        //         return Err(format!("Desktop file not found: {}", desktop_path).into());
        // }

        // Open the file
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // Find the Exec= line
        let mut exec_command: Option<String> = None;
        for line in reader.lines() {
                let line = line?;
                if line.starts_with("Exec=") {
                        // Remove "Exec=" and strip arguments placeholders like %f, %u
                        let cmd = line[5..]
                                .split_whitespace()
                                .filter(|s| !s.starts_with('%'))
                                .collect::<Vec<&str>>()
                                .join(" ");
                        exec_command = Some(cmd);
                        break;
                }
        }

        let exec_command = exec_command.ok_or("No Exec line found in .desktop file")?;

        // Launch the command
        let _child = Command::new("sh").arg("-c").arg(exec_command).spawn()?; // async

        // // Wait if desired: child.wait()?;
        // println!(".desktop application launched successfully!");
        Ok(())
}
