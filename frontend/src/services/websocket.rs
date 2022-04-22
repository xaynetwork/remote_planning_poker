use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;

pub struct WebsocketService {
    sender: Sender<String>,
}

impl WebsocketService {
    pub fn new() -> Self {
        let ws = WebSocket::open("ws://localhost:3000/websocket").unwrap();
        let (mut ws_write, mut ws_read) = ws.split();

        let (sender, mut receiver) = futures::channel::mpsc::channel::<String>(1000);

        spawn_local(async move {
            while let Some(s) = receiver.next().await {
                ws_write.send(Message::Text(s)).await.unwrap();
            }
        });

        spawn_local(async move {
            while let Some(msg) = ws_read.next().await {
                match msg {
                    Ok(Message::Text(data)) => {
                        log::debug!("[yew] text from websocket: {}", data);

                        // TODO: pass them to an output stream
                    }
                    Ok(Message::Bytes(b)) => {
                        let decoded = std::str::from_utf8(&b);
                        if let Ok(val) = decoded {
                            log::debug!("[yew] bytes from websocket: {}", val);
                        }
                    }
                    Err(e) => {
                        log::error!("[yew] ws error: {:?}", e)
                    }
                }
            }
            log::debug!("[yew] WebSocket Closed");
        });

        Self { sender }
    }

    pub fn send_message(&self, msg: String) {
        if let Err(err) = self.sender.clone().try_send(msg) {
            log::error!("[yew] send_message error: {:?}", err);
        }
    }
}
