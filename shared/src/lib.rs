use std::{fs::File, io::Read};

// Message protocol
// Field             Size
// ------------------------------
// Address length    u16
// Address           UTF-8 bytes
// Body length       u16
// Body              UTF-8 bytes

pub struct Message {
    pub address: String,
    pub body: String,
}

impl Message {
    pub fn encode(&self) -> Vec<u8> {
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

    pub fn decode(data: Vec<u8>) -> Result<Message, &'static str> {
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
        // let body_length: u16 = u16::from_be_bytes(body_length);
        // let body_length: usize = usize::from(body_length + 2);
        // ^~~~~~~~~~~~~~~~removed for now can reuse when more input fields are added
        let body_start: usize = address_end + 2;

        let mut body: Vec<u8> = Vec::new();

        body.extend_from_slice(&data[body_start..]);

        let body = String::from_utf8(body).unwrap();

        let message = Message { address, body };

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_test() {
        let message = Message {
            address: "mom".to_string(),
            body: "I got a tattoo".to_string(),
        };

        let message = message.encode();

        let message = Message::decode(message).expect("Failed to decode message");
        assert_eq!(message.address, "mom".to_string());
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
