#![forbid(unsafe_code)]

use paperio::{
    data::{CommandMessage, Message},
    strategy::Strategy,
};

use std::{
    io::{stdin, stdout, BufRead, BufReader, Read, Write},
    net::TcpStream,
};

fn read_message(reader: &mut impl BufRead) -> Option<Message> {
    let mut line = String::new();
    reader.read_line(&mut line).ok()?;
    serde_json::from_str(&line).ok()
}

fn run(reader: impl Read, mut writer: impl Write) {
    let mut reader = BufReader::new(reader);

    let Some(Message::StartGame(_)) = read_message(&mut reader) else {
        panic!("expected the first message to be 'start_game'");
    };

    let mut strategy = Strategy::new();
    while let Some(Message::Tick(tick_params)) = read_message(&mut reader) {
        let direction = strategy.on_tick(tick_params);
        writeln!(
            writer,
            "{}",
            serde_json::to_string(&CommandMessage {
                command: direction,
                debug: "".to_string(),
            })
            .unwrap()
        )
        .unwrap();
        writer.flush().unwrap();
    }
}

pub fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if let Some(port_str) = args.get(1) {
        let port = port_str.parse::<u16>().expect("args[1] should be a u16");
        let stream = TcpStream::connect(format!("localhost:{}", port))
            .expect("failed to connect to tcp socket");
        let cloned_stream = stream.try_clone().unwrap();
        run(stream, cloned_stream);
    } else {
        run(stdin(), stdout());
    }
}
