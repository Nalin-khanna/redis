#![allow(unused_imports)]
use std::{io::{Read, Write}, net::TcpListener};

fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                let mut buffer = [0;1024];
                let n = _stream.read(& mut buffer);
                match n {
                    Ok(n) => {
                        _stream.write_all(b"+PONG\r\n").unwrap();
                    }
                    Err(err) => {
                        print!("error : {} " , err);
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
