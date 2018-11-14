extern crate log;
use log::*;

mod request;
mod response;
mod static_router;

use request::*;
use response::*;
use static_router::*;

use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::{Read, Write, BufReader, BufWriter, BufRead};

fn main() -> io::Result<()> {
    env_logger::init();

    let host = "127.0.0.1";
    let port = 7777;

    let listener = TcpListener::bind((host, port))?;

    println!("listening on {}:{}", host, port);

    for stream in listener.incoming() {
        let mut s = stream?;

        let mut lines = BufReader::new(s.try_clone()?).lines();

        let mut req = Request::parse(
            lines
            .map(|rl| rl.unwrap()) // 
            .take_while(|l| !l.is_empty())
        ).unwrap();

        println!("method: {}", req.method);
        println!("url   : {}", req.url);
        println!("http_version: {}", req.http_version);
        println!("raw_headers: ");
        for line in &req.raw_headers {
            println!("    {}", line);
        }

        let mut res = Response::ok(s);
        // res.write(b"<h1>Hello world, Rust!</h1>");
        static_router::serve(req, res);

        
    }

    Ok(())
}
