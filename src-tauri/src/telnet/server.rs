use crate::telnet::prompt;
use socket2::{Domain, Protocol, Socket, TcpKeepalive, Type};
use std::net::SocketAddr;
use std::time::Duration;
use tauri::Window;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpSocket;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;

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
    pub channel: Sender<ArmageddonServerInternalPayload>,
    pub sink: Sender<String>,
}

impl ArmageddonServer {
    pub async fn new() -> Self {
        let (tx, _) = broadcast::channel::<ArmageddonServerInternalPayload>(10);
        let (tx2, _) = broadcast::channel::<String>(10);
        let server = ArmageddonServer {
            channel: tx,
            sink: tx2,
        };
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

        let channel = self.channel.clone();
        let mut rx = channel.subscribe();
        let tx2 = self.sink.clone();
        let mut rx2 = tx2.subscribe();

        tokio::spawn(async move {
            let (read_half, mut write_half) = stream.into_split();

            let mut reader = BufReader::new(read_half);
            let window = window.clone();

            loop {
                tokio::select! {
                  result = reader.fill_buf() => {
                    let received: &[u8] = result.unwrap();
                    let received = &received.to_vec();

                    if received.len() > 0 {
                      let window = window.clone();
                      reader.consume(received.len());
                      let received = String::from_utf8_lossy(&received);
                      let received = prompt::is_prompt(received, &window);
                      let received = received;
                      let payload = ArmageddonServerInternalPayload {
                        status: Some(CONNECTED),
                        message: Some(received)
                      };
                      channel.send(payload).unwrap();
                    }else{
                      let payload = ArmageddonServerInternalPayload {
                        status: Some(DISCONNECTED),
                        message: None,
                      };
                      channel.send(payload).unwrap();
                    }
                  }

                  result = rx.recv() => {
                    let received: ArmageddonServerInternalPayload = result.unwrap();
                    let window: Window = window.clone();

                    if received.message.is_some() {
                      let msg = received.message.unwrap();
                      println!("{}", msg);
                      window.emit("telnet-message", msg.as_bytes()).unwrap();
                    }

                    if received.status.is_some() {
                      let status: ArmageddonServerStatus = received.status.unwrap();
                      if status == CONNECTED {
                        window.emit("armageddon-connection", true).unwrap();
                      }else{
                        window.emit("armageddon-connection", false).unwrap();
                      }
                    }
                  }

                  result = rx2.recv() => {
                    let input: String = result.unwrap();
                    let input = &[input.as_bytes(), b"\r\n"].concat()[..];
                    write_half.write_all(input)
                      .await
                      .map_err(|error|  {
                        println!("ERROR: {}", error);
                      })
                      .unwrap();
                  }
                };
            }
        });
    }
}
