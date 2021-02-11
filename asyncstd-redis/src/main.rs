use async_std::{
    io,
    prelude::*, // 1
    task, // 2
    net::{TcpListener, ToSocketAddrs}, // 3
};
use async_std::sync::Mutex;
use async_std::net::TcpStream;
use std::sync::Arc;
use std::io::Error;



fn main() -> Result<()> {
	let fut = accept_loop("127.0.0.1:6379");
    task::block_on(fut)
}

async fn accept_loop(address: &str) -> Result<()>
    {
        let mut stream = TcpStream::connect(address).await?;
        println!("{:?}", &stream);
        let dt = stream.write_all(b"get hello2\n").await?;

        let buf = read_until(&mut stream, b'\n').await?;
        println!("{:?}", std::str::from_utf8(&buf).unwrap());
        Ok(())
    }

async fn read_until(r: &mut TcpStream, byte: u8) -> io::Result<Vec<u8>>{
    let mut buffer = Vec::new();
    let mut single = [0; 1];
    loop {
        r.read(&mut single).await?;
        buffer.push(single[0]);
        if single[0] == byte {
            return Ok(buffer);
        }
    }
}