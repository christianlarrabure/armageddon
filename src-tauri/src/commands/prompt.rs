use crate::config::prompt::PROMPT;
use crate::telnet::send_to_sink;
use crate::ArmageddonState;
use tauri::Window;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn set_prompt(
    _state: tauri::State<'_, Mutex<ArmageddonState>>,
    window: Window,
) -> Result<(), String> {
    let mut state = _state.lock().await;

    if let Some(sink) = state.sink.as_mut() {
        let input = String::from(format!("prompt {}", PROMPT));
        let input = input.as_str();
        send_to_sink(sink, &[input.as_bytes(), b"\r\n"].concat()[..]).await;
        super::tell_player("You have set your prompt.".to_string(), window)
    }

    Ok(())
}
