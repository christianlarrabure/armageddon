use crate::telnet::prompt;
use socket2::{Domain, Protocol, Socket, TcpKeepalive, Type};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::Window;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpSocket;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

#[derive(PartialEq, Debug, Clone)]
pub enum ArmageddonServerStatus {
    CONNECTED,
    DISCONNECTED,
}

use ArmageddonServerStatus::{CONNECTED, DISCONNECTED};

#[derive(Debug, Clone)]
pub struct ArmageddonServerInternalPayload {
    pub status: Option<ArmageddonServerStatus>,
    pub message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ArmageddonServer {
    pub sink: Option<broadcast::Sender<String>>,
}

impl ArmageddonServer {
    pub async fn new(window: Window) -> Self {
        let (tx2, _) = broadcast::channel::<String>(1);
        let server = ArmageddonServer { sink: Some(tx2) };
        server
    }

    pub async fn listen(&self, addr: &str, window: Window) {
        let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP)).unwrap();
        let keepalive = TcpKeepalive::new().with_time(Duration::from_secs(10));
        socket.set_tcp_keepalive(&keepalive).unwrap();

        let socket = TcpSocket::from_std_stream(socket.into());
        let addr: SocketAddr = addr.parse().expect("Invalid IP address.");
        let stream = socket
            .connect(addr)
            .await
            .expect("Failed to connect to telnet server.");

        let tx2 = self.sink.clone().unwrap();
        let tx2 = tx2.clone();
        let mut rx2 = tx2.subscribe();

        let (read_half, mut write_half) = stream.into_split();

        let mut reader = BufReader::new(read_half);

        self.listen_to_input(rx2, write_half).await;
        self.listen_armageddon(reader, window.clone()).await;
    }

    async fn listen_armageddon(&self, reader: BufReader<OwnedReadHalf>, window: Window) {
        let mut reader = reader;
        let window = window;
        tokio::spawn(async move {
            while let Ok(received) = reader.fill_buf().await {
                let received = &received.to_vec();
                let received = String::from_utf8_lossy(received);
                reader.consume(received.len());
                if received.len() == 0 {
                    break;
                }
                let window = window.clone();
                let received = String::from_utf8_lossy(received.as_bytes());
                let received = prompt::is_prompt(received, &window);
                let received = received;
                window.emit("telnet-message", received.as_bytes()).unwrap();
            }
        });
    }

    async fn listen_to_input(
        &self,
        mut rx: broadcast::Receiver<String>,
        mut write_half: OwnedWriteHalf,
    ) {
        tokio::spawn(async move {
            while let Ok(input) = rx.recv().await {
                let input = &[input.as_bytes(), b"\r\n"].concat()[..];
                write_half
                    .write_all(input)
                    .await
                    .map_err(|error| {
                        println!("ERROR: {}", error);
                    })
                    .unwrap();
            }
        });
    }
}
