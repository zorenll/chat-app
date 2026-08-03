use shared::{Message, toml_parser};
use std::{io::Read, io::Write, net::TcpListener, net::TcpStream, thread::spawn};

// Stream write protocol Packet1: Message size; Packet2: Message
fn read_message(stream: &mut TcpStream) -> Result<Message, &'static std::io::Error> {
    let mut message_length: [u8; 2] = [0; 2];
    stream
        .read_exact(&mut message_length)
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
    Ok(message)
}

fn handle_client(mut stream: TcpStream) {
    let mut stream = &mut stream;
    let message_result = read_message(&mut stream);

    let message = match message_result {
        Ok(message) => {
            let _ = stream.write_all("received".as_bytes());
            Ok(message)
        }
        Err(e) => Err(e),
    };

    let message = message.unwrap();

    println!(
        "From: {}\nTo: {}\nText: {}",
        message.from, message.to, message.body
    );

    // stream.flush().expect("Failed to flush");
}

fn main() {
    let config = toml_parser("server/config.toml");

    let mut ip = String::new();

    for variable in config {
        match variable.0.as_str() {
            "server_ip" => ip = format!("{}", variable.1.trim_matches('"')),
            "server_port" => ip = format!("{}:{}", ip, variable.1.trim_matches('"')),
            _ => eprintln!("Incorrect config file"),
        };
    }

    let listener = TcpListener::bind(ip).expect("Failed to bind to port");

    loop {
        let (stream, address) = listener.accept().expect("Failed to accept connection");

        println!("New connection from {}", address);

        spawn(move || handle_client(stream));
    }
}
