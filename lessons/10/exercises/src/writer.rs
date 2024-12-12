use serde::Serialize;
use std::marker::PhantomData;
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub struct MessageWriter<T, W> {
    stream: W,
    _phantom: PhantomData<T>,
}

impl<T: Serialize, W: AsyncWrite + Unpin> MessageWriter<T, W> {
    pub fn new(stream: W) -> Self {
        Self {
            stream,
            _phantom: Default::default(),
        }
    }

    pub async fn send(&mut self, msg: T) -> anyhow::Result<()> {
        let serialized = serde_json::to_vec(&msg)?;
        self.stream.write_all(&serialized).await?;
        self.stream.write_all(b"\n").await?;
        self.stream.flush().await?;
        Ok(())
    }

    #[allow(unused)]
    pub fn inner(&self) -> &W {
        &self.stream
    }

    #[allow(unused)]
    pub fn into_inner(self) -> W {
        self.stream
    }
}
