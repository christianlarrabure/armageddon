use std::{sync::Mutex, fs::File, io::Read};

use ansi_term::Colour::Green;
use tauri::Window;
use serde_json;
use mlua::prelude::*;

pub mod prompt;

use crate::{config::{SCRIPTS_FOLDER}, ArmageddonState};

pub fn tell_player(input: String, window: Window) {
    let tagline = String::from(format!("{}", Green.paint("[CLIENT] ")));
    let tagline = tagline.as_bytes();
    let message = [b"\r\n", tagline, input.as_bytes(), b"\r\n"].concat();
    window.emit("telnet-message", message).unwrap();
}

#[tauri::command]
pub async fn hashtag_command(verb: String, args: String, window: Window) {
    let hashtag_tree = File::open([SCRIPTS_FOLDER, "scripts.json"].concat());
    if let Err(error) = hashtag_tree {
        println!("ERROR: {}", error);
        tell_player("You need to have a scripts.json file in your scripts folder.".to_string(), window.clone());
        return;
    }
    let mut hashtag_tree = hashtag_tree.unwrap();

    let mut hashtag_tree_string = vec![];
    if let Err(error) = hashtag_tree.read_to_end(hashtag_tree_string.as_mut()) {
        println!("ERROR: {}", error);
        tell_player("Failed to read scripts.json.".to_string(), window.clone());
        return;
    }

    let hashtag_tree_string = String::from_utf8_lossy(&hashtag_tree_string);

    let hashtag_commands: serde_json::Value = serde_json::from_str(&hashtag_tree_string).unwrap();
    
    let command = hashtag_commands.get(&verb);

    if command.is_none() {
        tell_player("That hashtag command does not exist.".to_string(), window.clone());
        return;
    }

    let command = command.unwrap();

    let command_path = command.get("src");

    if command_path.is_none() {
        tell_player("The command is badly configured. It misses a src key-value.".to_string(), window.clone());
        return;
    }

    let command_path = command_path.unwrap();

    let script = File::open(command_path.to_string());

    if let Err(error) = script {
        let current_dir = std::env::current_dir();
        tell_player(format!("Failed to open script {} (current directory is {}) with error: {}", command_path.to_string(), current_dir.unwrap().display(), error), window.clone());
        return;
    }

    let mut script = script.unwrap();
    let mut script_contents = vec![];
    if let Err(error) = script.read_to_end(script_contents.as_mut()) {
        tell_player(format!("Failed to read script with error: {}", error), window.clone());
        return;        
    }

    unsafe {
        let lua = Lua::new();
        let globals = lua.globals();
        globals.set("verb", verb.clone());
        globals.set("args", args);
    
        {
            let window = window.clone();
            let tell_player = lua.create_function(move |_, message: String| {
                let message = [b"\r\n", message.as_bytes(), b"\r\n"].concat();
                window.emit("telnet-message", message).unwrap();
                Ok(())
            });
        
            globals.set("tell", tell_player.unwrap());
        }
    
        let script_result = lua.load(&script_contents).exec().unwrap();
        }
}