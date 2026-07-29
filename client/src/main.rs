use std::{io::Write, net::TcpStream};
use zord_shared::Message;

fn main() {
    let mut message = Message {
        address: "".to_string(),
        body: "".to_string(),
    };

    println!("Address: ");
    // std::io::stdout().flush().expect("Failed to flush stdout");
    let mut address = String::new();
    std::io::stdin()
        .read_line(&mut address)
        .expect("Failed to read line");

    println!("Body: ");
    // std::io::stdout().flush().expect("Failed to flush stdout");
    let mut body = String::new();
    std::io::stdin()
        .read_line(&mut body)
        .expect("Failed to read line");

    message.address = address.trim().to_string();
    message.body = body.trim().to_string();

    let ip = "127.0.0.1:4921";
    let mut stream = TcpStream::connect(ip).expect("Failed to connect");

    let message_encoded = message.encode();
    let message_length = message_encoded.len() as u16;

    stream
        .write(&message_length.to_be_bytes())
        .expect("Failed to write to stream");
    stream
        .write(&message_encoded)
        .expect("Failed to write to stream");
}
