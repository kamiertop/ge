mod app;
mod emoji;
mod ui;

use app::App;
use std::process::Command;

fn main() -> std::io::Result<()> {
    let mut app = App::default();

    ratatui::run(|terminal| app.run(terminal))?;

    if let Some(message) = app.submitted_message() {
        commit(message)?;
    }

    Ok(())
}

fn commit(message: &str) -> std::io::Result<()> {
    let output = Command::new("git")
        .args(["commit", "-m", message])
        .output()?;

    if output.status.success() {
        print!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("git commit failed for message: {message}");
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }

    Ok(())
}
