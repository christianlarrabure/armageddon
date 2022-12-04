#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;

use tokio::sync::Mutex;

use tauri::Manager;

use telnet::server::ArmageddonServer;

mod commands;
mod config;
mod settings;
mod telnet;

pub struct ArmageddonState {
    pub ip: String,
    pub logging: bool,
    pub server: Arc<Mutex<Option<ArmageddonServer>>>,
}

#[tokio::main]
async fn main() {
    let data = ArmageddonState {
        ip: String::from("ginka.armageddon.org:4050"),
        logging: true,
        server: Arc::new(Mutex::new(None)),
    };
    let data = Mutex::new(data);
    tauri::Builder::default()
        .setup(|app| {
            app.manage(data);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            telnet::connect,
            telnet::send,
            telnet::welcome::init,
            settings::get_config,
            commands::prompt::set_prompt,
            commands::hashtag_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
