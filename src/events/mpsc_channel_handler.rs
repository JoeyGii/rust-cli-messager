use std::sync::mpsc::{channel, Receiver, Sender};

use crate::model::models::Message;

pub struct Channel {
    pub sender: Sender<String>,
    pub receiver: Receiver<String>,
}
impl Channel {
    pub fn create_channel() -> Channel {
        //event channel
        let (sender, receiver): (Sender<String>, Receiver<String>) = channel();
        let channel = Channel {
            sender: sender,
            receiver: receiver,
        };
        channel
    }
}

#[tokio::main]
pub async fn print_kafka_messages_to_ui(
    receiver: Receiver<String>,
) -> Result<Message, Box<dyn std::error::Error>> {
    let received_payloads: Message = serde_json::from_str(&receiver.recv().unwrap()).unwrap();
    Ok(received_payloads)
}
