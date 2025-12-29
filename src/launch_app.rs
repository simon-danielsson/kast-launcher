use crate::structs::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn run(app: App) {
        let dest = app.path;
        if dest.ends_with(".AppImage") {
                run_appimage(&dest).unwrap();
        } else if dest.ends_with(".sh") {
                run_bash(&dest).unwrap();
        } else if dest.ends_with(".desktop") {
                run_desktop(&dest).unwrap();
        } else {
                run_misc(&dest).unwrap();
        }
}

fn run_misc(dest: &str) -> Result<(), Box<dyn std::error::Error>> {
        let _ = Command::new(&dest)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?;
        Ok(())
}

fn run_bash(dest: &str) -> Result<(), Box<dyn std::error::Error>> {
        let _ = Command::new("bash")
                .arg(dest)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?;
        Ok(())
}

fn run_appimage(app_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let _ = Command::new(app_path).spawn()?;
        Ok(())
}

fn run_desktop(desktop_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(desktop_path);
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut exec_command: Option<String> = None;
        for line in reader.lines() {
                let line = line?;
                if line.starts_with("Exec=") {
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

        let _ = Command::new("sh").arg("-c").arg(exec_command).spawn()?; // async

        Ok(())
}
