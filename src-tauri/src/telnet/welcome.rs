use super::super::config::version::get_software_name;
use ansi_term::Colour::{White, Yellow};
use tauri::Window;

#[tauri::command]
pub async fn init(window: Window) {
    // welcome message
    let mut messages = Vec::new();

    let mut intro = String::new();
    let software_name = get_software_name();
    intro = format!("Welcome to {}!", White.bold().paint(software_name));
    messages.push(intro);

    let mut w1 = String::new();
    w1 = format!("");
    messages.push(w1);

    let mut instructions = String::new();
    instructions = format!("Type {} to connect to the game. If this is your first time booting the client, make sure to type {} upon logging in to set your prompt.", Yellow.paint("#connect"), Yellow.paint("#prompt"));
    messages.push(instructions);

    for message in messages.iter() {
        window
            .emit("telnet-message", [message.as_bytes(), b"\r\n"].concat())
            .unwrap();
    }
}
