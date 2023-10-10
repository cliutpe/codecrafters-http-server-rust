use anyhow::Result;
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;

fn parse_request(request_str: &str) -> (&str, &str) {
    let parsed = request_str.split("\r\n\r\n").collect::<Vec<&str>>();
    (parsed[0], parsed[1])
}

fn parse_start_line(start_line_str: &str) -> (&str, &str, &str) {
    let parsed = start_line_str.split(" ").collect::<Vec<&str>>();
    (parsed[0], parsed[1], parsed[2])
}

fn parse_header(header_str: &str) -> HashMap<&str, &str> {
    let mut headers = HashMap::new();
    for line in header_str.split("\r\n") {
        let pair = line.split(": ").collect::<Vec<&str>>();
        headers.insert(pair[0], pair[1].trim());
    }
    headers
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    let request_size = stream.read(&mut buffer).unwrap();

    match str::from_utf8(&buffer[..request_size]) {
        Ok(request_str) => {
            let (start_line_str, header_str) = parse_request(request_str);
            let (_request_method, request_path, _http_version) = parse_start_line(start_line_str);
            let headers = parse_header(header_str);

            if request_path == r"/" {
                stream.write(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
            } else if request_path.starts_with("/echo/") {
                let random_string = request_path.split("/echo/").collect::<Vec<&str>>()[1];
                let buffer = format!(
                    "HTTP/1.1 200 OK\r\n\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    random_string.len(),
                    random_string
                );
                stream.write(buffer.as_bytes()).unwrap();
            } else if request_path == "/user-agent" {
                let user_agent = headers["User-Agent"];
                let buffer = format!(
                    "HTTP/1.1 200 OK\r\n\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    user_agent.len(),
                    user_agent
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
