use std::{fs::File, io::Read, io::Write, net::TcpStream};

// Message protocol
// Field             Size
// ------------------------------
// From length  u16
// From         UTF-8 bytes
// To length    u16
// To           UTF-8 bytes
// Body length  u16
// Body         UTF-8 bytes

pub struct Message {
    pub from: String,
    pub to: String,
    pub body: String,
}

impl Message {
    pub fn encode(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();

        let from_length = self.from.len();
        let from_length = from_length as u16;
        let to_length = self.to.len();
        let to_length = to_length as u16;

        let body_length = self.body.len();
        let body_length = body_length as u16;

        out.extend_from_slice(&from_length.to_be_bytes());
        out.extend_from_slice(self.from.as_bytes());

        out.extend_from_slice(&to_length.to_be_bytes());
        out.extend_from_slice(self.to.as_bytes());

        out.extend_from_slice(&body_length.to_be_bytes());
        out.extend_from_slice(self.body.as_bytes());

        return out;
    }

    pub fn decode(data: Vec<u8>) -> Result<Message, &'static str> {
        let mut from_length = [0; 2];
        from_length.copy_from_slice(&data[0..2]);
        let from_length: u16 = u16::from_be_bytes(from_length);
        let from_end: usize = usize::from(from_length + 2);

        let mut from: Vec<u8> = Vec::new();
        from.extend_from_slice(&data[2..from_end]);
        let from = String::from_utf8(from).unwrap();

        let mut to_length = [0; 2];
        let to_start: usize = 2 + from_end;
        to_length.copy_from_slice(&data[from_end..to_start]);
        let to_length: u16 = u16::from_be_bytes(to_length);
        let to_end = to_length + from_end as u16 + 2;
        let to_end: usize = usize::from(to_end);

        let mut to: Vec<u8> = Vec::new();
        to.extend_from_slice(&data[to_start..to_end]);
        let to = String::from_utf8(to).unwrap();

        let mut body_length = [0, 0];
        body_length.copy_from_slice(&data[to_end..to_end + 2]);
        // let body_length: u16 = u16::from_be_bytes(body_length);
        // let body_length: usize = usize::from(body_length + 2);
        // ^~~~~~~~~~~~~~~~removed for now can reuse when more input fields are added
        let body_start: usize = to_end + 2;

        let mut body: Vec<u8> = Vec::new();

        body.extend_from_slice(&data[body_start..]);

        let body = String::from_utf8(body).unwrap();

        let message = Message { from, to, body };

        Ok(message)
    }
}

pub fn tokenize(input: String, seperator: char) -> Vec<String> {
    let mut out = vec![String::new()];

    let mut index = 0;
    for char in input.chars() {
        if char != seperator {
            out[index].push(char);
        } else {
            out.push(String::new());
            index += 1;
        }
    }

    return out;
}

pub fn toml_parser(file_path: &'static str) -> Vec<(String, String)> {
    let mut config = File::open(file_path).expect("Failed to open file");

    let mut contents = String::new();
    config
        .read_to_string(&mut contents)
        .expect("Failed to read file");

    contents = contents.replace("\n", " ");

    let contents = tokenize(contents.trim().to_string(), ' ');

    let mut variables: Vec<(String, String)> = vec![];

    let mut index = 0;
    for word in &contents {
        let mut variable = (String::new(), String::new());
        if word == "=" {
            variable.0 = contents[index - 1].to_string();
            variable.1 = contents[index + 1].to_string();
            variables.push(variable);
        }
        index += 1;
    }

    return variables;
}

pub fn send_message(stream: &mut TcpStream, message: Vec<u8>) {
    let message_length = message.len() as u16;
    stream
        .write(&message_length.to_be_bytes())
        .expect("Failed to write to stream");
    stream.write(&message).expect("Failed to write to stream");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_test() {
        let message = Message {
            from: "zoren".to_string(),
            to: "mom".to_string(),
            body: "I got a tattoo".to_string(),
        };

        let message = message.encode();

        let message = Message::decode(message).expect("Failed to decode message");
        assert_eq!(message.from, "zoren".to_string());
        assert_eq!(message.to, "mom".to_string());
        assert_eq!(message.body, "I got a tattoo".to_string());
    }

    #[test]
    fn tokenizer_test() {
        let string = "word1 word2 word3".to_string();

        let strings = tokenize(string, ' ');

        assert_eq!(
            strings,
            [
                "word1".to_string(),
                "word2".to_string(),
                "word3".to_string()
            ]
        );
    }
}
