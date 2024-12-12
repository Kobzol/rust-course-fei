use serde::de::DeserializeOwned;
use std::io::{ErrorKind, Read};
use std::marker::PhantomData;

const MAX_MESSAGE_SIZE: u32 = 256;

pub struct MessageReader<T, R> {
    stream: R,
    _phantom: PhantomData<T>,
}

impl<T: DeserializeOwned, R: Read> MessageReader<T, R> {
    pub fn new(read: R) -> Self {
        Self {
            stream: read,
            _phantom: Default::default(),
        }
    }

    pub fn read(&mut self) -> Option<anyhow::Result<T>> {
        // Read message size
        let mut message = [0; 4];
        match self.stream.read_exact(&mut message) {
            Ok(_) => {}
            Err(error) if error.kind() == ErrorKind::UnexpectedEof => {
                return None;
            }
            Err(error) => return Some(Err(error.into())),
        }

        let size = u32::from_le_bytes(message);
        if size > MAX_MESSAGE_SIZE {
            return Some(Err(anyhow::anyhow!("Message too large ({size} bytes)")));
        }

        // Read message
        let mut buffer = vec![0; size as usize];

        if let Err(error) = self.stream.read_exact(&mut buffer) {
            return Some(Err(anyhow::anyhow!("Cannot read message: {error:?}")));
        }

        // Deserialize message from JSON
        match serde_json::from_slice::<T>(&buffer) {
            Ok(msg) => Some(Ok(msg)),
            Err(error) => Some(Err(anyhow::anyhow!(
                "Cannot deserialize message: {error:?}"
            ))),
        }
    }

    pub fn inner(&self) -> &R {
        &self.stream
    }

    pub fn into_inner(self) -> R {
        self.stream
    }
}

impl<T: DeserializeOwned, R: Read> Iterator for MessageReader<T, R> {
    type Item = anyhow::Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.read()
    }
}
