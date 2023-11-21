use crate::messages::ClientToServerMsg;
use crate::ChatResult;
use serde::de::DeserializeOwned;
use std::io::Read;
use std::marker::PhantomData;
use std::net::TcpStream;

pub struct MessageReader<'a, T> {
    socket: &'a TcpStream,
    buffer: [u8; 1024],
    size: usize,
    _phantom: PhantomData<T>,
}

impl<'a, T: DeserializeOwned> MessageReader<'a, T> {
    pub fn new(socket: &'a TcpStream) -> Self {
        Self {
            socket,
            buffer: [0; 1024],
            size: 0,
            _phantom: Default::default(),
        }
    }

    pub fn read(&mut self) -> Option<ChatResult<T>> {
        let mut size = 0;
        loop {
            let read = match self.socket.read(&mut self.buffer[size..]) {
                Ok(read) => read,
                Err(err) => return Some(Err(err.into())),
            };
            if read == 0 {
                return None;
            }
            size += read;

            // 10 == newline
            if let Some(position) = self.buffer[..size].iter().position(|c| c == &10) {
                let message = &self.buffer[..size][..position];
                let msg: T = match serde_json::from_slice(message) {
                    Ok(msg) => msg,
                    Err(err) => return Some(Err(err.into())),
                };
                self.buffer.copy_within(position + 1.., 0);
                size -= position + 1;
                return Some(Ok(msg));
            }
        }
    }
}

impl<'a, T: DeserializeOwned> Iterator for MessageReader<'a, T> {
    type Item = ChatResult<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.read()
    }
}
