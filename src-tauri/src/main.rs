#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tokio::{
    net::{tcp::OwnedWriteHalf, TcpSocket},
    sync::Mutex,
};
mod commands;
mod config;
mod settings;
mod telnet;

pub struct ArmageddonState {
    pub ip: String,
    pub sink: Option<OwnedWriteHalf>,
    pub logging: bool,
    pub socket: Option<TcpSocket>,
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(ArmageddonState {
            ip: String::from("ginka.armageddon.org:4050"),
            sink: None,
            logging: true,
            socket: None,
        }))
        .invoke_handler(tauri::generate_handler![
            telnet::connect,
            telnet::send,
            telnet::welcome::init,
            settings::get_config,
            commands::prompt::set_prompt,
            commands::help::pcommand_help,
            commands::whois::pcommand_whois
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
