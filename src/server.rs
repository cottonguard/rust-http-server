use std::io;
use std::net::{ToSocketAddrs, TcpListener, TcpStream};
use std::io::{BufReader, BufRead};

use request::*;
use response::*;
use static_router;

pub struct Server {

}

impl Server {
    pub fn new() -> Server {
        Server {}
    }

    pub fn listen(&self, addr: impl ToSocketAddrs) -> io::Result<()> {
        let listener = TcpListener::bind(addr)?;

        for stream in listener.incoming() {
            match self.process(stream) {
                _ => {}
            }
        }

        unreachable!();
    }

    fn process(&self, stream: io::Result<TcpStream>) -> io::Result<()> {
        let s = stream?;

        let lines = BufReader::new(s.try_clone()?).lines();

        let req = Request::parse(
            lines
            .map(|rl| rl.unwrap())
            .take_while(|l| !l.is_empty())
        ).unwrap();

        println!("method: {}", req.method());
        println!("url   : {}", req.url());
        println!("http_version: {}", req.http_version());
        println!("raw_headers: ");
        for line in req.raw_headers() {
            println!("    {}", line);
        }

        let res = Response::ok(s);
        static_router::serve(req, res);

        Ok(())
    }
}
