use std::io::BufRead;
use std::net::TcpStream;
use std::sync::Arc;

use crate::messages::{ClientToServerMsg, ServerToClientMsg};
use crate::reader::MessageReader;
use crate::writer::MessageWriter;
use crate::{ChatReaderError, ChatResult};

pub fn client_loop(socket: TcpStream) -> ChatResult<()> {
    let socket = Arc::new(socket);
    let socket2 = socket.clone();

    let thread = std::thread::spawn(move || {
        let mut writer = MessageWriter::new(&socket);
        let stdin = std::io::stdin().lock();
        for line in stdin.lines() {
            let line = line?;
            writer.send(ClientToServerMsg::Message(line))?;
        }
        Ok::<_, ChatReaderError>(())
    });

    let mut reader = MessageReader::<ServerToClientMsg>::new(&socket2);
    for msg in reader {
        let msg = msg?;
        match msg {
            ServerToClientMsg::Message(message) => {
                println!("{message}");
            }
        }
    }

    if thread.is_finished() {
        thread.join().unwrap()?;
    }
    Ok(())
}
