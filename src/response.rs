use std::io;
use std::io::{Write, BufWriter};
use std::net::TcpStream;
use std::collections::HashMap;

pub struct Response {
    pub socket: TcpStream,
    pub http_version: String,
    pub status_code: i32,
    pub status_message: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>
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

    pub fn write(&mut self, chunk: &[u8]) {
        self.body.extend(chunk);
    }

    pub fn end(self) -> io::Result<()> {
        let mut bw = BufWriter::new(self.socket);

        write!(&mut bw, "{} {} {}\r\n", 
               self.http_version, self.status_code, self.status_message)?;
        write!(&mut bw, "{}: {}\r\n", "Content-Type", "text/html;charset=UTF-8")?;
        write!(&mut bw, "\r\n")?;
        bw.write(&self.body)?;

        bw.flush()?;

        Ok(())
    }
}
