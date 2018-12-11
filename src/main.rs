#![feature(pattern)]

extern crate mio;

mod server;
mod request;
mod response;
mod static_router;
mod mime;

use server::*;

use std::io;

fn main() -> io::Result<()> {
    let host = [127, 0, 0, 1];
    let port = 7777;

    // println!("start to listen on {}:{}", host, port);

    Server::new().listen(&(host, port).into())?;

    Ok(())
}

