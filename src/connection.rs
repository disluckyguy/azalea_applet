use std::{error::Error, fs, path::Path, sync::mpsc::channel};

use bytes::{Buf, BytesMut};
use iced::{
    futures::{SinkExt, Stream},
    stream,
};
use serde::{Serialize, de::DeserializeOwned};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{UnixListener, UnixStream},
};

use crate::{
    RuntimeMessage,
    runtime::Plugin,
    tokio_runtime, unique_id,
};

pub fn subscribe() -> impl Stream<Item = RuntimeMessage> {
    stream::channel(100, |mut tx| async move {
        let (sender, receiver) = channel();
        let mut output = tx.clone();
        tokio_runtime().spawn(async move {
            let path = Path::new("/tmp/sock");
            if path.exists() {
                fs::remove_file(path).unwrap();
            }

            let listener = UnixListener::bind(path).unwrap();

            while let Ok((stream, _)) = listener.accept().await {
                stream.peer_cred().unwrap().pid().unwrap();
                let id = unique_id();
                let sender = sender.clone();

                let (updates_sender, updates_receiver) = channel();
                let plugin = Plugin { sender: updates_sender, view: None };
                tx.send(RuntimeMessage::New(plugin, id)).await.unwrap();
                tokio_runtime().spawn(async move {
                    let mut connection = Connection::new(stream);

                    loop {
                        while let Ok(message) = updates_receiver.recv() {
                            connection.write_frame(message).await.unwrap();
                            if let Ok(Some(message)) = connection.read_frame().await {
                                sender.send((message, id)).unwrap();
                            }
                        }
                    }
                });
            }
        });
        tokio_runtime().spawn(async move {
            while let Ok((message, id)) = receiver.recv() {
                output.send(RuntimeMessage::Request(message, id)).await.unwrap()
                
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

    pub async fn read_frame<T>(&mut self) -> Result<Option<T>, Box<dyn Error>>
    where
        T: Serialize + DeserializeOwned,
    {
        loop {
            if let Ok(frame) = self.parse_frame().await {
                return Ok(Some(frame));
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await.unwrap() {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }

    async fn parse_frame<T>(&mut self) -> Result<T, Box<dyn Error>>
    where
        T: Serialize + DeserializeOwned,
    {
        self.stream.readable().await.unwrap();

        let mut size_buf = vec![0; 4];
        let _ = self.stream.read_exact(&mut size_buf).await?;
        let size = size_buf.as_slice().get_u32_le();

        println!("size {} {:?}", size, size_buf);

        if size as usize >= self.buffer.remaining() {
            println!("reserving");
            self.buffer.reserve(size as usize + 4);
        }

        println!("reading");
        let num_bytes = self.stream.read_buf(&mut self.buffer).await.unwrap();

        println!("{:?}", num_bytes);
        // if num_bytes < 4 {
        //     return Err("Connection reset by peer".into());
        // }
        if size != num_bytes as u32 {
            return Err("Size doesn't match actual buffer size".into());
        }

        let element: T =
            bincode::serde::decode_from_slice(&self.buffer, bincode::config::standard())
                .unwrap()
                .0;
        self.buffer.advance(num_bytes);

        Ok(element)
    }

    pub async fn write_frame<T>(&mut self, message: T) -> Result<(), Box<dyn Error>>
    where
        T: Serialize + DeserializeOwned,
    {
        self.stream.writable().await.unwrap();
        let mut src = bincode::serde::encode_to_vec(message, bincode::config::standard()).unwrap();
        let len = (src.len() as u32).to_le_bytes();
        src.splice(0..0, len);
        //self.stream.write_u32_le(src.len() as u32).await.unwrap();
        self.stream.write_all(&mut src).await.unwrap();
        self.stream.flush().await.unwrap();
        Ok(())
    }
}
