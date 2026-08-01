use std::{io::Write, net::TcpStream};
use zord_shared::{Message, send_message, toml_parser};

fn input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}
fn main() {
    let config = toml_parser("client/config.toml");

    let mut ip = String::new();

    let mut username = String::new();

    for variable in config {
        match variable.0.as_str() {
            "server_ip" => ip = format!("{}", variable.1.trim_matches('"')),
            "server_port" => ip = format!("{}:{}", ip, variable.1.trim_matches('"')),
            "username" => username = variable.1.trim_matches('"').to_string(),
            _ => eprintln!("Incorrect config file"),
        };
    }

    let mut stream = TcpStream::connect(&ip).expect("Failed to connect");

    // send_message(&mut stream, username.clone().into_bytes());

    loop {
        print!("To: ");
        std::io::stdout().flush().expect("Failed to flush");
        let input_to = input();

        print!("Body: ");
        std::io::stdout().flush().expect("Failed to flush");
        let input_body = input();

        let message = Message {
            from: username.to_string(),
            to: input_to.trim().to_string(),
            body: input_body.trim().to_string(),
        };

        send_message(&mut stream, message.encode());

        stream.flush().expect("Failed to flush");
    }
}
