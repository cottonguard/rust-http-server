use std::io;
use std::io::{BufReader, BufRead};
use std::net::TcpStream;
use std::collections::HashMap;
use std::{fmt};

#[derive(Default, Debug)]
pub struct Request {
    method: Method,
    url: String,
    http_version: String,
    raw_headers: Vec<String>,
    headers: HashMap<String, String>,
}

impl Request {
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn http_version(&self) -> &str {
        &self.http_version
    }

    pub fn raw_headers(&self) -> &[String] {
        &self.raw_headers
    } 

    /*
    pub fn receive(stream: &TcpStream) -> io::Result<Result<Request, Error>> {
        let lines = BufReader::new(stream.try_clone()?).lines();

        Ok(Request::parse(
            lines
            .map(|rl| rl.unwrap())
            .take_while(|l| !l.is_empty())
        ))
    }
    */

    pub fn parse(data: &[u8])
        -> Result<Request, Error> {

        let mut lines = data.lines().map(|rl| rl.unwrap()).take_while(|l| !l.is_empty());

        let mut req = Request::default();
        
        let request_line = match lines.next() {
            Some(l) => l,
            None => return Err(Error::new(ErrorKind::InvalidRequest))
        };

        Self::parse_request_line(&mut req, &request_line)?;

        Self::parse_headers(&mut req, lines.collect())?;

        Ok(req)
    }

    fn parse_request_line(req: &mut Request, line: &str) -> Result<(), Error> {
        let tokens: Vec<_> = line.split(' ').collect();
        if tokens.len() < 3 { 
            return Err(Error::new(ErrorKind::InvalidRequest));
        }

        req.method = tokens[0].to_string();
        req.url = tokens[1].to_string();
        req.http_version = tokens[2].to_string();
        
        Ok(())
    }

    fn parse_headers(req: &mut Request, lines: Vec<String>/* <-(?_?) */)
    -> Result<(), Error> {
        for line in &lines {
            let mut tokens = line.split(":");
            let name = tokens.next().map(|s| s.trim().to_string());
            let value = tokens.next().map(|s| s.trim().to_string());
            
            match (name, value) { 
                (Some(n), Some(v)) => req.headers.insert(n, v),
                _ => return Err(Error::new(ErrorKind::InvalidRequest))
            };
        }
        req.raw_headers = lines;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorKind {
    InvalidRequest
}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error { kind }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}


type Method = String;
/*
pub enum Method {
    Get,
    Post,
    Head
}

impl Default for Method {
    fn default() -> Method {
        Method::Get
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        let msg = [
            b"GET /dir/file.html HTTP/1.1",
            b"Host: example.com:7777",
        ].join(b"\r\n");

        let req = Request::parse(msg).unwrap();

        assert_eq!(req.method(), "GET");
        assert_eq!(req.url(), "/dir/file.html");
        assert_eq!(req.http_version(), "HTTP/1.1");
    }

    #[test]
    fn invalid_request_line() {
        let msg = [
            ":)",
            "host: example.com"
        ].iter().map(|s| s.to_string());

        let req = Request::parse(msg);

        assert_eq!(req.unwrap_err().kind(), ErrorKind::InvalidRequest);
    }

    #[test]
    fn invalid_header() {
        let msg = [
            "GET / HTTP/1.1",
            "X-Valid-Header: ok",
            "no colon"
        ].iter().map(|s| s.to_string());

        let req = Request::parse(msg);
        
        assert_eq!(req.unwrap_err().kind(), ErrorKind::InvalidRequest);
    }
}
