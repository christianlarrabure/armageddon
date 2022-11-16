mod aliases;
mod logger;
mod prompt;
pub mod welcome;

use socket2::{Domain, Protocol, Socket, TcpKeepalive, Type};
use std::time::Duration;
use tauri::Window;
use tokio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpSocket;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn connect(
    _state: tauri::State<'_, Mutex<super::ArmageddonState>>,
    window: Window,
) -> Result<(), ()> {
    let socket2_socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP)).unwrap();
    let keepalive = TcpKeepalive::new().with_time(Duration::from_secs(10));
    socket2_socket.set_tcp_keepalive(&keepalive).unwrap();

    let mut state = _state.lock().await;

    let socket = TcpSocket::from_std_stream(socket2_socket.into());
    state.socket = Some(socket);
    let socket = state.socket.take().unwrap();

    let addr = "206.72.195.251:4050".parse().unwrap();

    let stream = socket
        .connect(addr)
        .await
        .expect("Failed to connect to telnet server.");

    let (read_half, write_half) = stream.into_split();
    let stream = BufReader::new(read_half);

    let _ = state.sink.insert(write_half);
    let logging = state.logging;

    tauri::async_runtime::spawn(async move { process_stream(stream, window, logging).await });

    Ok(())
}

async fn process_stream(mut stream: BufReader<OwnedReadHalf>, window: Window, logging: bool) {
    while let Ok(received) = stream.fill_buf().await {
        let received = &received.to_vec();
        let received = String::from_utf8_lossy(received);
        stream.consume(received.len());
        if received.len() > 0 {
            println!("{}", received);
            let received = prompt::is_prompt(received, &window);
            if logging == true {
                logger::log(&received).unwrap();
            }
            let received = received.as_bytes();
            window.emit("telnet-message", received).unwrap();
        }
    }
}

pub async fn send_to_sink(sink: &mut OwnedWriteHalf, input: &[u8]) {
    sink.write_all(input)
        .await
        .map_err(|e| {
            println!("ERROR: {}", e);
        })
        .unwrap();
    println!("> {}", String::from_utf8_lossy(input));
}

#[tauri::command]
pub async fn send(
    input: &str,
    _state: tauri::State<'_, Mutex<super::ArmageddonState>>,
) -> Result<(), String> {
    let mut state = _state.lock().await;

    if let Some(sink) = state.sink.as_mut() {
        let input = aliases::transform_with_aliases(input);
        let input = &[input.as_bytes(), b"\r\n"].concat()[..];
        send_to_sink(sink, input).await;
    }

    Ok(())
}
