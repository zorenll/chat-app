use std::{io::Read, io::Write, net::TcpListener, net::TcpStream};

// Message protocol
// Field             Size
// ------------------------------
// Address length    u16
// Address           UTF-8 bytes
// Body length       u16
// Body              UTF-8 bytes

struct Message {
    address: String,
    body: String,
}

impl Message {
    fn encode(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();

        let address_length = self.address.len();
        let address_length = address_length as u16;

        let body_length = self.body.len();
        let body_length = body_length as u16;

        out.extend_from_slice(&address_length.to_be_bytes());
        out.extend_from_slice(self.address.as_bytes());

        out.extend_from_slice(&body_length.to_be_bytes());
        out.extend_from_slice(self.body.as_bytes());

        return out;
    }

    fn decode(data: Vec<u8>) -> Result<Message, &'static str> {
        let mut address_length = [0; 2];
        address_length.copy_from_slice(&data[0..2]);
        let address_length: u16 = u16::from_be_bytes(address_length);
        let address_start: usize = 2;
        let address_end: usize = usize::from(address_length + 2);

        let mut address: Vec<u8> = Vec::new();
        address.extend_from_slice(&data[address_start..address_end]);
        let address = String::from_utf8(address).unwrap();

        let mut body_length = [0, 0];
        body_length.copy_from_slice(&data[address_end..address_end + 2]);
        let body_length: u16 = u16::from_be_bytes(body_length);
        let body_length: usize = usize::from(body_length + 2);
        let body_start: usize = address_end + 2;

        let mut body: Vec<u8> = Vec::new();

        body.extend_from_slice(&data[body_start..]);

        let body = String::from_utf8(body).unwrap();

        let message = Message { address, body };

        Ok(message)
    }
}

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
