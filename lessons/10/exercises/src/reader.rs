use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use tokio::io::{AsyncRead, AsyncReadExt};

pub struct MessageReader<T, R> {
    buffer: Vec<u8>,
    loaded: usize,
    client: R,
    _phantom: PhantomData<T>,
}

impl<T: DeserializeOwned, R: AsyncRead + Unpin> MessageReader<T, R> {
    pub fn new(client: R) -> Self {
        Self {
            buffer: vec![0; 1024],
            loaded: 0,
            client,
            _phantom: Default::default(),
        }
    }

    pub async fn recv(&mut self) -> Option<std::io::Result<T>> {
        loop {
            if let Some(position) = self.buffer[..self.loaded].iter().position(|c| *c == b'\n') {
                let msg = &self.buffer[..position];
                let msg: T = match serde_json::from_slice(msg) {
                    Ok(msg) => msg,
                    Err(error) => return Some(Err(error.into())),
                };
                self.buffer.copy_within(position + 1.., 0);

                self.loaded -= position + 1;
                return Some(Ok(msg));
            }

            assert!(self.loaded < self.buffer.len());
            let read_bytes = match self.client.read(&mut self.buffer[self.loaded..]).await {
                Ok(b) => b,
                Err(err) => return Some(Err(err.into())),
            };
            if read_bytes == 0 {
                break;
            }
            self.loaded += read_bytes;
        }
        None
    }
}
