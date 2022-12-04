use std::{fs::File, io::Read, path::Path};
use tokio::sync::Mutex;

use ansi_term::Colour::Green;
use mlua::prelude::*;
use serde_json;
use tauri::Window;
use tokio::sync::broadcast::Sender;

pub mod prompt;
pub mod topics;

use crate::config::SCRIPTS_FOLDER;

use self::topics::TopicController;

pub fn tell_player(input: String, window: Window) {
    let tagline = String::from(format!("{}", Green.paint("[CLIENT] ")));
    let tagline = tagline.as_bytes();
    let message = [b"\r\n", tagline, input.as_bytes(), b"\r\n"].concat();
    window.emit("telnet-message", message).unwrap();
}

pub fn get_hashtag_commands() -> Result<serde_json::Value, String> {
    let hashtag_tree = File::open([SCRIPTS_FOLDER, "scripts.json"].concat());
    if let Err(error) = hashtag_tree {
        return Err("You need to have a scripts.json in your scripts folder.".to_string());
    }
    let mut hashtag_tree = hashtag_tree.unwrap();

    let mut hashtag_tree_string = vec![];
    if let Err(error) = hashtag_tree.read_to_end(hashtag_tree_string.as_mut()) {
        return Err("Failed to read scripts.json.".to_string());
    }

    let hashtag_tree_string = String::from_utf8_lossy(&hashtag_tree_string);

    let hashtag_commands: serde_json::Value = serde_json::from_str(&hashtag_tree_string).unwrap();
    Ok(hashtag_commands)
}

pub async fn get_sink(
    _state: tauri::State<'_, Mutex<crate::ArmageddonState>>,
) -> Result<Option<Sender<String>>, String> {
    let state = _state.clone();
    let state = state.lock().await;
    let server = state.server.clone();
    let server = server.clone();
    let server = server.lock().await;
    let server = server.as_ref();
    if server.is_none() {
        return Ok(None);
    }
    let server = server.unwrap();
    let sink = server.sink.clone();
    Ok(sink)
}

#[tauri::command]
pub async fn hashtag_command(
    verb: String,
    args: String,
    window: Window,
    _state: tauri::State<'_, Mutex<crate::ArmageddonState>>,
) -> Result<(), String> {
    let _state = _state.clone();
    let sink = get_sink(_state.clone()).await;
    let sink = sink.unwrap();
    let hashtag_commands: Result<serde_json::Value, String> = get_hashtag_commands();
    if let Err(error) = hashtag_commands {
        tell_player(format!("ERROR: {}", error), window.clone());
        return Ok(());
    }
    let hashtag_commands = hashtag_commands.unwrap();
    let command = hashtag_commands.get(&verb);

    if command.is_none() {
        tell_player(
            "That hashtag command does not exist.".to_string(),
            window.clone(),
        );
        return Ok(());
    }

    let command = command.unwrap();

    let command_path = command.get("src");

    if command_path.is_none() {
        tell_player(
            "The command is badly configured. It misses a src key-value.".to_string(),
            window.clone(),
        );
        return Ok(());
    }

    let command_path = command_path.unwrap();

    let command_path = [SCRIPTS_FOLDER, &command_path.as_str().unwrap()].concat();

    let command_path = Path::new(&command_path);

    let script = File::open(command_path);

    if let Err(error) = script {
        let current_dir = std::env::current_dir();
        tell_player(
            format!(
                "Failed to open script {} (current directory is {}) with error: {}",
                command_path.display(),
                current_dir.unwrap().display(),
                error
            ),
            window.clone(),
        );
        return Ok(());
    }

    let mut script = script.unwrap();
    let mut script_contents = vec![];
    if let Err(error) = script.read_to_end(script_contents.as_mut()) {
        tell_player(
            format!("Failed to read script with error: {}", error),
            window.clone(),
        );
        return Ok(());
    }

    let lua = Lua::new();
    let globals = lua.globals();
    globals.set("verb", verb.clone()).unwrap();
    globals.set("argstr", args.clone()).unwrap();

    {
        let arg_table = lua.create_table().unwrap();
        let mut arg_count = 0;
        for arg in args.split(" ") {
            arg_count += 1;
            arg_table.set(arg_count, arg).unwrap();
        }
        globals.set("args", arg_table).unwrap();
    }

    {
        let window = window.clone();
        let tell_player = lua.create_function(move |_, message: String| {
            let message = [b"\r\n", message.as_bytes(), b"\r\n"].concat();
            window.emit("telnet-message", message).unwrap();
            Ok(())
        });

        globals.set("tell", tell_player.unwrap()).unwrap();
    }

    {
        let send_to_armageddon = lua.create_function(move |_, message: String| {
            let sink = sink.clone();
            if sink.is_none() {
                return Ok(false);
            }
            let sink = sink.unwrap();
            let result = sink.send(message);
            if result.is_err() {
                return Ok(false);
            }
            Ok(true)
        });
        globals.set("send", send_to_armageddon.unwrap()).unwrap();
    }

    let _ = lua.load(&script_contents).exec().unwrap();
    Ok(())
}
