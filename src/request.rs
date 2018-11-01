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

        Self::parse_request_line(&mut req, &request_line)?;

        Self::parse_headers(&mut req, lines.collect())?;

        Ok(req)
    }

    fn parse_request_line(req: &mut Request, line: &str) -> Result<(), RequestParseError> {
        let tokens: Vec<_> = line.split(' ').collect();
        if tokens.len() < 3 { 
            return Err(RequestParseError {});
        }

        req.method = tokens[0].to_string();
        req.url = tokens[1].to_string();
        req.http_version = tokens[2].to_string();
        
        Ok(())
    }

    fn parse_headers(req: &mut Request, lines: Vec<String>/* <-(?_?) */)
    -> Result<(), RequestParseError> {
        req.raw_headers = lines;
        Ok(())
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
