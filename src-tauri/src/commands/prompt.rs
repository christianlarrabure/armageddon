use crate::config::prompt::PROMPT;
use crate::ArmageddonState;
use tauri::Window;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn set_prompt(
    _state: tauri::State<'_, Mutex<ArmageddonState>>,
    window: Window,
) -> Result<(), String> {
    let state = _state.lock().await;
    let mut server = state.server.lock().await;

    if server.is_none() {
        super::tell_player("You are not connected to the game.".to_string(), window);
    } else {
        let server = server.as_mut().unwrap();
        let input = String::from(format!("prompt {}", PROMPT));
        let sink = &server.sink.clone().unwrap();
        sink.send(input).unwrap();
        super::tell_player("You have set your prompt.".to_string(), window)
    }

    Ok(())
}
