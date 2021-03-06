use async_std::{
    io,
    prelude::*, // 1
    task, // 2
    net::{TcpListener, ToSocketAddrs}, // 3
};
use async_std::sync::Mutex;
use async_std::net::TcpStream;
use async_std::io::Result;
use std::sync::{Arc};
use lib::{connectionpool, connect};
use lib::connect::RedisClient;

extern crate num_cpus;

#[async_std::main]
async fn main() -> Result<()> {
    use connect;
	let pool = connectionpool::ConnectionPool::create("127.0.0.1:6379".to_string(), None, num_cpus::get()).await?;
    let mut conn = pool.get().await;
    //client.write(conn);
    //println!("{:?}", client.write());
    Ok(())
    //task::block_on(conn)
}
/*
async fn accept_loop(address: &str) -> Result<()>
    {
        let _poll = connectionpool::ConnectionPool::create("127.0.0.1:6379".to_string(), None, num_cpus::get()).await?;
        /*let mut conn = poll.get().await;
        conn.write_all(b"get hello3\n").await?;
        let mut buffer = Vec::new();
        let mut buf = vec![0; 15];
        conn.read(&mut buf).await?;
        buffer.append(&mut buf);
        /*conn.read(&mut buf).await?;
        buffer.push(buf[0]);
        conn.read(&mut buf).await?;
        buffer.push(buf[0]);*/
        println!("{:?}", buffer);
        //println!("{:?}", conn.read(&mut buf).await?);
        //let mut buf2 = vec![];
        //println!("{:?}", conn.read(&mut buf2).await?);
        //buf.pop();
        //buf.pop();
        //let mut buf2 = read_until(&mut conn, b'\n').await?;
        //conn.read(&mut buf2).await?;
        //println!("{:?}", buf2);*/
        Ok(())
    }
*/
async fn read_until(r: &mut TcpStream, byte: u8) -> io::Result<Vec<u8>>{
    let mut buffer = Vec::new();
    let mut single = [0; 1];
    loop {
        r.read(&mut single).await?;
        buffer.push(single[0]);
        if single[0] == byte {
            println!("{:?}", buffer);
            return Ok(buffer);
        }
    }
}

async fn parse_string(start: &[u8], stream: &mut TcpStream) -> io::Result<String> {
    if start == b"$-1\r\n" {
        Ok("".to_string())
    } else {
        let num = String::from_utf8_lossy(&start[1..])
            .trim()
            .parse::<usize>()
            .unwrap();
        let mut buf = vec![0u8; num + 2]; // add two to catch the final \r\n from Redis
        stream.read(&mut buf).await?;

        buf.pop(); //Discard the last \r\n
        buf.pop();
        Ok(String::from_utf8_lossy(&buf).parse().unwrap())
    }
}