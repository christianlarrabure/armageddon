mod aliases;
mod logger;
mod prompt;
pub mod server;
pub mod welcome;
use tauri::Window;
use tokio;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn connect(
    _state: tauri::State<'_, Mutex<super::ArmageddonState>>,
    window: Window,
) -> Result<(), ()> {
    let window = window.clone();
    let server = server::ArmageddonServer::new().await;
    let state = _state.lock().await;
    let state_server = state.server.clone();
    let mut state_server = state_server.lock().await;
    let server = state_server.insert(server);
    server.listen("206.72.195.251:4050", window).await;
    Ok(())
}
#[tauri::command]
pub async fn send(
    input: &str,
    _state: tauri::State<'_, Mutex<super::ArmageddonState>>,
) -> Result<(), String> {
    let state = _state.lock().await;
    let mut server = state.server.lock().await;

    let server = server.as_mut();
    if server.is_none() {
        println!("Tried to send input but there is no server.");
    } else {
        let server = server.unwrap();
        let sink = server.sink.clone();

        let input = aliases::transform_with_aliases(input);
        sink.send(input)
            .map_err(|error| {
                println!("Error: {}", error);
            })
            .unwrap();
    }
    Ok(())
}
