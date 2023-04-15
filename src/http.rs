use std::io::{BufRead, BufReader, Read, Result, Write};
use std::net::TcpStream;

pub enum HttpMethod {
    Get,
    Post,
}

pub fn make_request(host: &str, port: u16, method: HttpMethod, endpoint: &str) -> Result<String> {
    let request = match method {
        HttpMethod::Get => format!("GET /{} HTTP/1.1\r\nHost: {}\r\n\r\n", endpoint, host),
        HttpMethod::Post => format!("POST /{} HTTP/1.1\r\nHost: {}\r\n\r\n", endpoint, host),
    };
    let mut stream = TcpStream::connect(format!("{}:{}", host, port))?;
    stream.write_all(request.as_bytes())?;

    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader.read_to_string(&mut response)?;
    Ok(response)
}
