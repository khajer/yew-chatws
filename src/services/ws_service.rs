use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
use gloo::console::log;
use reqwasm::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;

pub fn connect_websocket() {
    log!("connect web socket");
    let ws = WebSocket::open("ws://127.0.0.1:8080/ws").unwrap();
    let (mut write, mut read) = ws.split();
    let (in_tx, mut in_rx) = futures::channel::mpsc::channel::<String>(1000);

    let msg = "test";
    if let Ok(_) = in_tx.clone().try_send(serde_json::to_string(&msg).unwrap()) {
        log!("message sent successfully");
    }

    spawn_local(async move {
        while let Some(s) = in_rx.next().await {
            log!(format!("got event from channel! {}", s));
            write.send(Message::Text(s)).await.unwrap();
        }
    });
    spawn_local(async move {
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(data)) => {
                    check_data_input(data);
                }
                Ok(Message::Bytes(b)) => {
                    let decoded = std::str::from_utf8(&b);
                    if let Ok(val) = decoded {
                        log!("from websocket: {}", val);
                    }
                }
                Err(_) => {
                    log!("ws: websocket error");
                }
            }
        }
        log!("WebSocket Closed");
    });
}
fn check_data_input(data: String) {
    log!("from websocket: {}", data);

    todo!("parse cmd ");
}
