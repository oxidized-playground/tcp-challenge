use std::time;

mod net;

type ErrorMessage = String;

#[tokio::main]
async fn main() -> Result<(), ErrorMessage> {
    main_loop().await
}

async fn main_loop() -> Result<(), ErrorMessage> {
    let remote_address = "playground.foxlabs.nl:8080";
    let mut client = net::client::Client::new(remote_address).await?;

    loop {
        // TCP challenge, send a message via the client interface to the server. Can you get your
        // message displayed on the screen?
        // Do use '.await?' to ensure the function gets executed.

        let message = r#"{}"#;

        // After sending, receive the reply from the server to print, it will tell you if you miss something in your message
        let reply = client.read().await?;
        println!("Received: {:?}", reply);

        // Extra challenge, can you send some struct?
        // Modify the json_client in src/net/json_client.rs to serialize and deserialize messages
        // let msg = net::json_client::Message {
        //     sender: String::from("Your-Name"),
        //     content: String::from("AAAA"),
        // };

        // client.write_message(&msg).await?;
        // let reply = client.read_message().await?;
        // println!("Received: {:?}", reply);

        let ten_seconds = time::Duration::from_secs(10);
        tokio::time::sleep(ten_seconds).await;
    }
}
