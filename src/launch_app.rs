use crate::structs::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn run(app: App) {
        let dest = app.path;
        if dest.ends_with(".AppImage") {
                let _ = run_appimage(&dest);
        } else if dest.ends_with(".sh") {
                let _ = run_bash(&dest);
        } else if dest.ends_with(".desktop") {
                let _ = run_desktop(&dest);
        } else {
                let _ = run_misc(&dest);
        }
}

fn run_misc(dest: &str) -> Result<(), Box<dyn std::error::Error>> {
        let _ = Command::new(dest)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?; // Fire-and-forget
        Ok(())
}

fn run_bash(dest: &str) -> Result<(), Box<dyn std::error::Error>> {
        let _ = Command::new("bash")
                .arg(dest)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?;
        Ok(())
}

fn run_appimage(app_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let _ = Command::new(app_path)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?;
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

        let _ = Command::new("sh")
                .arg("-c")
                .arg(exec_command)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?; // async fire-and-forget

        Ok(())
}
