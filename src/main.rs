use anyhow::Result;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    let request_size = stream.read(&mut buffer).unwrap();

    match str::from_utf8(&buffer[..request_size]) {
        Ok(request_str) => {
            let lines = request_str.split("\r\n").collect::<Vec<&str>>();
            println!("{:?}", lines);
            let start_line = lines[0].split(" ").collect::<Vec<&str>>();
            println!("{:?}", start_line);
            if start_line[1] == r"/" {
                stream.write(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
            } else if start_line[1].starts_with("/echo/") {
                let random_string = start_line[1].split("/echo/").collect::<Vec<&str>>()[1];
                println!("{:?}", random_string);
                let buffer = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    random_string.len(),
                    random_string
                );
                println!("{:?}", buffer);
                stream.write(buffer.as_bytes()).unwrap();
            } else {
                stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n").unwrap();
            }
        }
        Err(_e) => {}
    }
}
fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                handle_connection(_stream);
            }
            Err(_e) => {}
        }
    }
    Ok(())
}
