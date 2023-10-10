use anyhow::Result;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;

fn parse_request(request_str: &str) -> (&str, &str) {
    let parsed = request_str.splitn(2, "\r\n").collect::<Vec<&str>>();
    (parsed[0], parsed[1])
}

fn parse_start_line(start_line_str: &str) -> (&str, &str, &str) {
    let parsed = start_line_str.split(" ").collect::<Vec<&str>>();
    (parsed[0], parsed[1], parsed[2])
}

fn parse_header(header_str: &str) -> HashMap<&str, &str> {
    let mut headers = HashMap::new();
    for header_line in header_str.split("\r\n") {
        let pair = header_line.split(": ").collect::<Vec<&str>>();
        if pair.len() == 2 {
            headers.insert(pair[0], pair[1].trim());
        }
    }
    headers
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];

    let request_size = stream.read(&mut buffer).unwrap();

    match str::from_utf8(&buffer[..request_size]) {
        Ok(request_str) => {
            println!("request string:\n\n{:?}", request_str);
            let (start_line_str, header_str) = parse_request(request_str);
            let (_request_method, request_path, _http_version) = parse_start_line(start_line_str);
            let headers = parse_header(header_str);

            if request_path == r"/" {
                stream.write(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
            } else if request_path.starts_with("/echo/") {
                let random_string = request_path.split("/echo/").collect::<Vec<&str>>()[1];
                let buffer = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    random_string.len(),
                    random_string
                );
                stream.write(buffer.as_bytes()).unwrap();
            } else if request_path == "/user-agent" {
                println!("{:?}", headers);
                let user_agent = headers["User-Agent"];
                let buffer = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    user_agent.len(),
                    user_agent
                );
                stream.write(buffer.as_bytes()).unwrap();
            } else if request_path.starts_with("/files/") {
                let dir_arg = env::args().next().unwrap();
                println!("{:?}", dir_arg);
                let dir = dir_arg.split_once("=").unwrap().1;

                let file_name = request_path.split("/files/").collect::<Vec<&str>>()[1];
                let file_path = format!("{}/{}", dir, file_name);
                println!("{:?}", file_path);
                match fs::read_to_string(file_path) {
                    Ok(file_content) => {
                        let buffer = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
                            file_content.len(),
                            file_content
                        );
                        println!("{:?}", buffer);
                        stream.write(buffer.as_bytes()).unwrap();
                    }
                    Err(_e) => {
                        stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n").unwrap();
                    }
                }
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
                thread::spawn(|| handle_connection(_stream));
            }
            Err(_e) => {}
        }
    }
    Ok(())
}
