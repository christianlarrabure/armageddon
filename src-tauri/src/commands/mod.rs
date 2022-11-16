use ansi_term::Colour::Green;
use tauri::Window;

pub mod help;
pub mod prompt;
pub mod whois;

pub fn tell_player(input: String, window: Window) {
    let tagline = String::from(format!("{}", Green.paint("[CLIENT] ")));
    let tagline = tagline.as_bytes();
    let message = [b"\r\n", tagline, input.as_bytes(), b"\r\n"].concat();
    window.emit("telnet-message", message).unwrap();
}
