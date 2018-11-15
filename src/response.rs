use std::io;
use std::io::{Write, BufWriter};
use std::net::TcpStream;
use std::collections::HashMap;

pub struct Response {
    socket: TcpStream,
    http_version: String,
    status_code: i32,
    status_message: String,
    headers: HashMap<String, String>,
    body: Vec<u8>
}

impl Response {
    pub fn ok(socket: TcpStream) -> Response {
        Response {
            socket,
            http_version: String::from("HTTP/1.1"),
            status_code: 200,
            status_message: String::from("OK"),
            headers: HashMap::new(),
            body: Vec::with_capacity(256)
        }
    }

    pub fn set_status_code(&mut self, code: i32) {
        self.status_code = code;
    }

    pub fn set_status_message(&mut self, msg: &str) {
        self.status_message = String::from(msg);
    }

    pub fn set_header(&mut self, name: &str, value: &str) {
        self.headers.insert(String::from(name), String::from(value));
    }

    pub fn write(&mut self, chunk: &[u8]) {
        self.body.extend(chunk);
    }

    pub fn end(self) -> io::Result<()> {
        let mut bw = BufWriter::new(self.socket);

        write!(&mut bw, "{} {} {}\r\n", 
               self.http_version, self.status_code, self.status_message)?;
        for (name, value) in self.headers {
            write!(&mut bw, "{}: {}\r\n", name, value);
        }
        write!(&mut bw, "\r\n")?;
        bw.write(&self.body)?;

        bw.flush()?;

        Ok(())
    }
}
