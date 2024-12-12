use serde::de::DeserializeOwned;
use std::io::Read;
use std::marker::PhantomData;
use std::net::TcpStream;

pub struct MessageReader<T> {
    pub socket: TcpStream,
    buffer: [u8; 1024],
    size: usize,
    _phantom: PhantomData<T>,
}

impl<T: DeserializeOwned> MessageReader<T> {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            socket,
            buffer: [0; 1024],
            size: 0,
            _phantom: Default::default(),
        }
    }

    pub fn read(&mut self) -> Option<std::io::Result<T>> {
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
                let msg: T = serde_json::from_slice(message).unwrap();
                self.buffer.copy_within(position + 1.., 0);
                size -= position + 1;
                return Some(Ok(msg));
            }
        }
    }
}
