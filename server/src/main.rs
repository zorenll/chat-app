use std::{io::Read, io::Write, net::TcpListener};
use zord_shared::Message;

// Stream write protocol Packet1: Message size; Packet2: Message
fn main() {
    let ip = "127.0.0.1:4921";
    let listener = TcpListener::bind(ip).expect("Failed to bind to port");

    for stream in listener.incoming() {
        let mut stream = stream.expect("Failed to stream man");

        let mut message_length: [u8; 2] = [0; 2];
        stream
            .read(&mut message_length)
            .expect("Failed to read from stream");
        let message_length: u16 = u16::from_be_bytes(message_length);
        let message_length: usize = usize::from(message_length);

        let mut message: Vec<u8> = vec![0; message_length];
        stream
            .read_exact(&mut message)
            .expect("Failed to read message");
        if message.len() != message_length {
            eprintln!("Did not read full message");
        }

        let message = Message::decode(message).expect("Failed to decode message");

        println!("To: {}\nText: {}", message.address, message.body);
    }
}
