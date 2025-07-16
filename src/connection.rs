use std::{error::Error, fs, path::Path, time::Duration};

use bytes::{Buf, BytesMut};
use iced::{
    futures::{SinkExt, Stream},
    stream,
};
use serde::{Serialize, de::DeserializeOwned};
use tokio::{
    sync::mpsc::unbounded_channel,
    io::{AsyncReadExt, AsyncWriteExt},
    net::{UnixListener, UnixStream},
};

use crate::{RuntimeMessage, runtime::Plugin, tokio_runtime, unique_id};

pub fn subscribe() -> impl Stream<Item = RuntimeMessage> {
    stream::channel(100, |tx| async move {
        let (sender, mut receiver) = unbounded_channel();
        let mut output = tx.clone();
        tokio_runtime().spawn(async move {
            let path = Path::new("/tmp/sock");
            if path.exists() {
                fs::remove_file(path).unwrap();
            }

            let listener = UnixListener::bind(path).unwrap();
            loop {
                if let Ok((stream, _)) = listener.accept().await {
                    let sender = sender.clone();
                    let mut tx = tx.clone();
                    tokio_runtime().spawn(async move {
                        let id = unique_id();

                        let (updates_sender, mut updates_receiver) = unbounded_channel();
                        let plugin = Plugin {
                            id,
                            sender: updates_sender,
                            view: None,
                        };
                        tx.send(RuntimeMessage::New(plugin, id)).await.unwrap();
                        let mut connection = Connection::new(stream);
                        loop {
                            if !connection.is_open().await {
                                
                                tx.send(RuntimeMessage::Shutdown(id)).await.unwrap();
                                break;
                            }
                            connection.stream.flush().await.unwrap();
                            if let Ok(Some(message)) = tokio::time::timeout(Duration::from_millis(500), updates_receiver.recv()).await {
                                connection.write_frame(message).await.unwrap();
                                if let Ok(Some(message)) = connection.read_frame().await {
                                    sender.send((message, id)).unwrap();
                                }
                            }
                        }
                    });
                }
            }
        });
        tokio_runtime().spawn(async move {
            while let Some((message, id)) = receiver.recv().await {
                output
                    .send(RuntimeMessage::Request(message, id))
                    .await
                    .unwrap()
            }
        });
    })
}

pub struct Connection {
    stream: UnixStream,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: UnixStream) -> Self {
        Self {
            stream,
            buffer: BytesMut::with_capacity(4096),
        }
    }

    pub async fn is_open(&mut self) -> bool {
        match self.stream.write_all(&[0, 0, 0, 0, 0]).await {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    pub async fn read_frame<T>(&mut self) -> Result<Option<T>, Box<dyn Error>>
    where
        T: Serialize + DeserializeOwned,
    {
        loop {
            if let Ok(frame) = self.parse_frame().await {
                return Ok(frame);
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }

    async fn parse_frame<T>(&mut self) -> Result<Option<T>, Box<dyn Error>>
    where
        T: Serialize + DeserializeOwned,
    {
        //self.stream.readable().await?;
        let mut size_buf = vec![0; 4];
        let _ = self.stream.read_exact(&mut size_buf).await?;

        let size = size_buf.as_slice().get_u32_le();

        if size == 0 {
            let mut buf = vec![0u8; 1];
            _ = self.stream.read(&mut buf).await?;
            if buf.get(0).map(|b| b == &0).unwrap_or_default() {
                return Ok(None);
            }
        }

        if size as usize >= self.buffer.remaining() {
            self.buffer.reserve(size as usize + 4);
        }
        let num_bytes = self.stream.read_buf(&mut self.buffer).await?;

        if size != num_bytes as u32 {
            return Err("Size doesn't match actual buffer size".into());
        }

        let element: T =
            bincode::serde::decode_from_slice(&self.buffer, bincode::config::standard())?.0;
        self.buffer.advance(num_bytes);

        Ok(Some(element))
    }

    pub async fn write_frame<T>(&mut self, message: T) -> Result<(), Box<dyn Error>>
    where
        T: Serialize + DeserializeOwned,
    {
        //self.stream.writable().await?;
        let mut src = bincode::serde::encode_to_vec(message, bincode::config::standard())?;
        let len = (src.len() as u32).to_le_bytes();
        src.splice(0..0, len);
        self.stream.write_all(&mut src).await?;
        self.stream.flush().await?;
        Ok(())
    }
}
