#![allow(unused_imports)]
use std::{io::{Read, Write}};
use tokio::{io::{AsyncReadExt , AsyncWriteExt}, net::{TcpListener,TcpStream}};
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop {
        let (mut stream , _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let mut buffer = [0u8 ; 1024];
            loop {
                let n = stream.read(&mut buffer).await.unwrap();
                if n == 0 {break;}
                if let Err(e) = stream.write_all(b"+PONG\r\n").await {
                    println!("error {}", e);
                }
            }
        });
    }
}
