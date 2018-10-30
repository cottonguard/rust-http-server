extern crate log;
use log::*;
extern crate env_logger;

mod request;
use request::*;

use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::{Read, Write, BufReader, BufWriter, BufRead};

fn main() -> io::Result<()> {
    env_logger::init();

    let host = "127.0.0.1";
    let port = 7777;

    let listener = TcpListener::bind((host, port))?;

    for stream in listener.incoming() {
        let mut s = stream?;

        let mut lines = BufReader::new(s.try_clone()?).lines();

        let req = Request::parse(
            lines
            .map(|rl| rl.unwrap()) // 
            .take_while(|l| !l.is_empty())
        ).unwrap();

        println!("method: {}", req.method);
        println!("url   : {}", req.url);
        println!("http_version: {}", req.http_version);
        println!("raw_headers: ");
        for line in req.raw_headers {
            println!("    {}", line);
        }

        let mut bw = BufWriter::new(s);

        write!(&mut bw, "{} {} {}\r\n", "HTTP/1.1", 200, "OK")?;
        write!(&mut bw, "{}:{}\r\n", "Content-Type", "text/html;charset=UTF-8")?;
        write!(&mut bw, "\r\n")?;

        write!(&mut bw, "<h1>Hello world, Rust!</h1>")?;
        
        bw.flush()?;
    }

    Ok(())
}
