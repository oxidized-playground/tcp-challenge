use crate::{net, ErrorMessage};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub sender: String,
    pub content: String,
}

#[async_trait]
trait ALTENChatter {
    async fn write_message(&mut self, message: &Message) -> Result<(), ErrorMessage>;
    async fn read_message(&mut self) -> Result<Message, ErrorMessage>;
}

#[async_trait]
impl ALTENChatter for net::client::Client {
    async fn write_message(&mut self, _message: &Message) -> Result<(), ErrorMessage> {
        todo!();
    }

    async fn read_message(&mut self) -> Result<Message, ErrorMessage> {
        todo!()
    }
}
