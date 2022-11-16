use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const DEFAULT: &str = r#"{
  "terminalBackground": "111827FF",
  "terminalForeground": "FFFFFFFF",
  "aliases": {
    "eyebrow": "emote lifts ^me eyebrow curiously."
  },
  "statBars": {
    "hp": {
      "show": true,
      "background": "B91C1CFF",
      "foreground": "FFFFFFFF",
      "emptyBackground": "1F2937FF"
    },
    "mana": {
      "show": true,
      "background": "1D4ED8FF",
      "foreground": "FFFFFFFF",
      "emptyBackground": "1F2937FF"
    },
    "stamina": {
      "show": true,
      "background": "047857FF",
      "foreground": "FFFFFFFF",
      "emptyBackground": "1F2937FF"
    },
    "stun": {
      "show": true,
      "background": "334155FF",
      "foreground": "FFFFFFFF",
      "emptyBackground": "1F2937FF"
    },
    "focus": {
      "show": true,
      "background": "6D28D9FF",
      "foreground": "FFFFFFFF",
      "emptyBackground": "1F2937FF"
    }
  }
}"#;

fn open_or_create() -> File {
    let file_exists = File::open("settings.json");
    if file_exists.is_ok() {
        return File::open("settings.json").unwrap();
    } else {
        let mut file = File::create("settings.json").unwrap();
        let _ = file.write_all(DEFAULT.as_bytes());
        let _ = file.sync_all().unwrap();
        return File::open("settings.json").unwrap();
    }
}

#[tauri::command]
fn get_settings() -> Value {
    let file = open_or_create();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let value: Value = serde_json::from_str(&contents.as_str()).unwrap();
    value
}

#[tauri::command]
pub fn get_config(field: String) -> Value {
    let value = get_settings();
    value[field].clone()
}
