use std::{io::Write, net::TcpStream};
use zord_shared::Message;

fn main() {
    let ip = "127.0.0.1:4921";
    let mut stream = TcpStream::connect(ip).expect("Failed to connect");

    let message = Message {
        address: "mary".to_string(),
        body: "Hello mary you have hemroids".to_string(),
    };

    let message_encoded = message.encode();
    let message_length = message_encoded.len() as u16;

    stream
        .write(&message_length.to_be_bytes())
        .expect("Failed to write to stream");
    stream
        .write(&message_encoded)
        .expect("Failed to write to stream");
}
