use std::collections::HashMap;
use std::error;
use std::fmt;

#[derive(Default)]
pub struct Request {
    pub method: Method,
    pub url: String,
    pub http_version: String,
    pub raw_headers: Vec<String>,
    pub headers: HashMap<String, String>,
}

impl Request {
    pub fn parse<T: Iterator<Item = String>>(mut lines: T)
        -> Result<Request, RequestParseError> {
        // jissou ga bimyou

        let mut req = Request::default();
        
        let request_line = match lines.next() {
            Some(l) => l,
            None => return Err(RequestParseError {})
        };

        let tokens: Vec<_> 
            = request_line.split(' ').collect();
        if (tokens.len() < 3) { 
            return Err(RequestParseError {});
        }
        req.method = tokens[0].to_string();
        req.url = tokens[1].to_string();
        req.http_version = tokens[2].to_string();

        req.raw_headers = lines.collect();

        Ok(req)

        /* 
        (|| {
            let mut req = Request::default();

            let request_line = lines.next()?;
            let mut tokens = request_line.split(' ').map(|s| s.to_string());
            req.method = tokens.next()?;
            req.url = tokens.next()?;
            req.http_version = tokens.next()?;

            req.raw_headers = lines.collect();

            Ok(req)
        })().or_else(|e| Err(RequestParseError {}))
        */
    }
}

#[derive(Debug)]
pub struct RequestParseError {}

impl error::Error for RequestParseError {}

impl fmt::Display for RequestParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invarid request format")
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
