use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{str, time};

mod net;

type ErrorMessage = std::string::String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    sender: String,
    content: String,
}

#[async_trait]
trait ALTENChatter {
    async fn write_message(&mut self, message: &Message) -> Result<(), ErrorMessage>;
    async fn read_message(&mut self) -> Result<Message, ErrorMessage>;
}

#[async_trait]
impl ALTENChatter for net::client::Client {
    async fn write_message(&mut self, message: &Message) -> Result<(), ErrorMessage> {
        todo!();
    }

    async fn read_message(&mut self) -> Result<Message, ErrorMessage> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), ErrorMessage> {
    main_loop().await
}

async fn main_loop() -> Result<(), ErrorMessage> {
    let remote_address = "jansuun.tech:80";
    let mut client = net::client::Client::new(remote_address).await?;

    let msg = Message {
        sender: String::from("Your-Name"),
        content: String::from("AAAA"),
    };

    loop {
        // Part 1, send a message via the client interface. Can you get your message on the screen? 
        // Receive the reply from the server to print, it will tell you if you miss something in your message

        // Part 2, implement these functions in the trait
        // client.write_message(&msg).await?;
        // let reply = client.read_message().await?;
        // println!("Received: {:?}", reply);

        let ten_seconds = time::Duration::from_secs(10);
        std::thread::sleep(ten_seconds);
    }
}
