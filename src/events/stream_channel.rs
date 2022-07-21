use std::sync::mpsc::{channel, Receiver, Sender};

use crate::model::models::Message;

pub struct NewChannel {
    pub sender: Sender<String>,
    pub receiver: Receiver<String>,
}
impl NewChannel {
    pub fn create_channel() -> NewChannel {
        //event channel
        let (sender, receiver): (Sender<String>, Receiver<String>) = channel();
        let channel = NewChannel {
            sender: sender,
            receiver: receiver,
        };
        channel
    }
}

#[tokio::main]
pub async fn publish_kafka_messages_to_ui(
    receiver: Receiver<String>,
) -> Result<Message, Box<dyn std::error::Error>> {
    let received_payloads: Message = serde_json::from_str(&receiver.recv().unwrap()).unwrap();
    Ok(received_payloads)
}
