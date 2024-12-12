use serde::Serialize;
use std::io::Write;
use std::marker::PhantomData;

pub struct MessageWriter<T, W> {
    sink: W,
    _phantom: PhantomData<T>,
}

impl<W: Write, T: Serialize> MessageWriter<T, W> {
    pub fn new(write: W) -> Self {
        Self {
            sink: write,
            _phantom: Default::default(),
        }
    }

    pub fn write(&mut self, message: T) -> anyhow::Result<()> {
        // Serialize the data
        let serialized = serde_json::to_vec(&message)?;

        // Write size
        let size = serialized.len() as u32;
        self.sink.write_all(&size.to_le_bytes())?;

        // Write data
        self.sink.write_all(&serialized)?;
        self.sink.flush()?;
        Ok(())
    }

    pub fn inner(&self) -> &W {
        &self.sink
    }

    pub fn into_inner(self) -> W {
        self.sink
    }
}
