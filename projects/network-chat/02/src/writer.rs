use crate::messages::ServerToClientMsg;
use crate::ChatResult;
use serde::Serialize;
use std::io::Write;
use std::net::TcpStream;

pub struct MessageWriter<'a> {
    socket: &'a TcpStream,
}

impl<'a> MessageWriter<'a> {
    pub fn new(socket: &'a TcpStream) -> Self {
        Self { socket }
    }

    pub fn send<T: Serialize>(&mut self, msg: T) -> ChatResult<()> {
        let data = serde_json::to_vec(&msg)?;
        self.socket.write_all(&data)?;
        self.socket.write(b"\n")?;
        self.socket.flush()?;
        Ok(())
    }
}
