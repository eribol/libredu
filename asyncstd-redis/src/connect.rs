use crate::connectionpool::ConnectionPool;
use async_std::sync::{MutexGuard, Mutex};
use async_std::net::TcpStream;
use async_std::io::Result;
use futures::AsyncWriteExt;
use std::sync::Arc;
use async_std::net::ToSocketAddrs;
use async_std::io;

#[derive(Debug, Clone)]
pub struct RedisClient{
    pub streamer: Arc<Mutex<TcpStream>>
}

impl RedisClient{
    pub fn write(&self) -> Result<&Self>{
        //self.streamer.write_all(&self.command.as_bytes().to_vec());
        Ok(self)
    }
    pub async fn connect<A>(address: A) -> Result<Self>
        where
            A: ToSocketAddrs,
    {
        let streamer = Arc::new(Mutex::new(
            TcpStream::connect(address)
                .await?,
        ));

        Ok(Self {streamer })
    }
}


