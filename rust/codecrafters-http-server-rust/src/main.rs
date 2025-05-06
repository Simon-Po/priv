use std::io::{Read, Write};
#[allow(unused_imports)]
use std::net::TcpListener;

#[derive(Debug)]
struct StatusLine<'a> {
    http_version: &'a str,
    status_code: &'a str,
    reason: Option<&'a str>,
}

impl<'a> StatusLine<'a> {
    fn new(http_version: &'a str, status_code: &'a str, reason_or: &'a str) -> StatusLine<'a> {
        StatusLine {
            reason: match reason_or {
                "" => None,
                _ => Some(reason_or),
            },
            status_code,
            http_version,
        }
    }

    fn get_format(&self) -> String {
        match self.reason {
            Some(reason) => format!(
                "{} {} {}\r\n\r\n",
                self.http_version, self.status_code, reason
            ),
            None => format!("{} {}\r\n\r\n", self.http_version, self.status_code),
        }
    }
}
#[derive(Debug)]
struct Request {
    http_method: String,
    target: String,
    http_version: String,
    host: String,
    user_agent: String,
    accept: String,
}

impl Request {
    fn new(request_string: String) -> Request {
        let (request_line, header_block) = request_string
            .split_once("\r\n")
            .expect("Invalid request string: missing CRLF");

        let parts: Vec<&str> = request_line.split_whitespace().collect();
        let (http_method, target, http_version) = match &parts[..] {
            [method, target, version] => {
                (method.to_string(), target.to_string(), version.to_string())
            }
            _ => panic!("Invalid request line format"),
        };

        let mut host = String::new();
        let mut user_agent = String::new();
        let mut accept = String::new();

        for line in header_block.lines() {
            if let Some((key, value)) = line.split_once(": ") {
                match key {
                    "Host" => host = value.to_string(),
                    "User-Agent" => user_agent = value.to_string(),
                    "Accept" => accept = value.to_string(),
                    _ => {}
                }
            }
        }

        Request {
            http_method,
            target,
            http_version,
            host,
            user_agent,
            accept,
        }
    }
}

use std::fmt;

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "HTTP Method: {}", self.http_method)?;
        writeln!(f, "Target: {}", self.target)?;
        writeln!(f, "HTTP Version: {}", self.http_version)?;
        writeln!(f, "Host: {}", self.host)?;
        writeln!(f, "User-Agent: {}", self.user_agent)?;
        writeln!(f, "Accept: {}", self.accept)
    }
}
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut request: String = String::default();
                stream.read_to_string(&mut request).unwrap();
                println!("accepted new connection");
                let request = Request::new(request);

                println!("Request received: {}", request);
                match request.target.as_str() {
                    "/" => match stream.write_all(
                        StatusLine::new("HTTP/1.1", "200", "OK")
                            .get_format()
                            .as_bytes(),
                    ) {
                        Ok() => println!("Succesfully answered!"),
                        Err(e) => eprintln!("Error while sending response: {}", e),
                    },
                    _ => stream
                        .write_all(
                            StatusLine::new("HTTP/1.1", "404", "Not Found")
                                .get_format()
                                .as_bytes(),
                        )
                        .unwrap(),
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
