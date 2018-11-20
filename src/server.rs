use std::io;
use std::net::{SocketAddr/*, TcpListener, TcpStream*/};
use std::io::prelude::*;
use mio::*;
use mio::net::{TcpListener, TcpStream};
use std::collections::HashMap;

use request::*;
use response::*;
use static_router;

#[derive(Clone, Copy, PartialEq)]
enum ConnectionState {
    Loading,
    Sending,
    Closed,
}

struct Connection {
    token: Token,
    state: ConnectionState,
    socket: TcpStream,
    addr: SocketAddr,
    buf: Vec<u8>,
    pos: usize
}

const INITIAL_BUF_SIZE: usize = 256;

impl Connection {
    fn new(token: Token, socket: TcpStream, addr: SocketAddr) -> Connection {
        let mut conn = Connection {
            state: ConnectionState::Loading,
            token,
            socket,
            addr,
            buf: Vec::with_capacity(INITIAL_BUF_SIZE),
            pos: 0
        };
        conn.buf.resize(INITIAL_BUF_SIZE, 0);
        conn
    }
}

pub struct Server {
    next_socket_index: usize,
    connections: HashMap<Token, Connection>
}

const LISTENER: Token = Token(0);

impl Server {
    pub fn new() -> Server {
        Server {
            next_socket_index: 1,
            connections: HashMap::new()
        }
    }

    pub fn listen(&mut self, addr: &SocketAddr) -> io::Result<()> {
        let listener = TcpListener::bind(addr)?;
        let poll = Poll::new()?;
        poll.register(&listener, LISTENER, Ready::readable(), PollOpt::edge())?;
        let mut events = Events::with_capacity(1024);

        loop {
            println!("==== polling...");
            let _size = poll.poll(&mut events, None)?;

            for event in &events {
                match event.token() {
                    LISTENER => {
                        self.accept(&listener, &poll)?;
                    }
                    token => {
                        let state = {
                            let conn = self.connections.get_mut(&token).unwrap();
                            match conn.state {
                                ConnectionState::Loading => Self::load(conn, &poll)?,
                                ConnectionState::Sending => Self::send(conn, &poll)?,
                                _ => {}
                            }
                            conn.state
                        };
                        if state == ConnectionState::Closed {
                            println!("==== closed: {}", token.0);
                            self.connections.remove(&token);  
                        }
                    }
                }
            }
        }
        unreachable!();
    }

    fn accept(&mut self, listener: &TcpListener, poll: &Poll) -> io::Result<()> {
        loop { // need loop?
            match listener.accept() {
                Ok((socket, addr)) => {
                    let token = Token(self.next_socket_index);
                    self.next_socket_index += 1;

                    println!("==== accept: {}", token.0);

                    poll.register(
                        &socket, token, Ready::readable(), PollOpt::edge())?;
                    
                    self.connections.insert(token, Connection::new(token, socket, addr));
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    return Ok(());
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }

    fn load(conn: &mut Connection, poll: &Poll) -> io::Result<()> {
        loop {
            if conn.buf.len() == conn.pos {
                let new_len = 2 * conn.buf.len();
                conn.buf.resize(new_len, 0);
            }
            match conn.socket.read(&mut conn.buf[conn.pos..]) {
                Ok(0) => {
                    conn.state = ConnectionState::Closed;
                    return Ok(());
                }
                Ok(n) => {
                    println!("==== chunk: {}", conn.token.0);
                    let prev_pos = conn.pos;
                    conn.pos += n;
                    io::stdout().write(&conn.buf[prev_pos .. conn.pos]);

                    if conn.buf[.. conn.pos].ends_with(b"\r\n\r\n") { //ends_with?
                        println!("==== end header");
                        let req_result = Request::parse(&conn.buf[.. conn.pos]);
                        match req_result {
                            Ok(req) => {
                                /*
                                println!("method: {}", req.method());
                                println!("url   : {}", req.url());
                                println!("http_version: {}", req.http_version());
                                println!("raw_headers: ");
                                for line in req.raw_headers() {
                                    println!("    {}", line);
                                }
                                */

                                conn.state = ConnectionState::Sending;
                                poll.reregister(
                                    &conn.socket, conn.token, Ready::writable(), PollOpt::edge());

                                // let res = Response::ok(s);
                                // static_router::serve(req, res);
                            }
                            Err(_) => {/*todo: bad request*/}
                        }
                    } 

                    println!();
                    println!("==== end of chunk");
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    return Ok(());
                }
                Err(e) => { return Err(e); }
            } 
        }
    }

    fn send(conn: &mut Connection, poll: &Poll) -> io::Result<()> {
        let mut res = Response::ok();
        let body = b"<h1>Hello world!</h1>";
        res.write(body);
        res.set_header("content-length", &body.len().to_string());
        res.end(&conn.socket);
        // static_router::serve();
        // clear buffer
        conn.pos = 0;
        conn.state = ConnectionState::Loading;
        poll.reregister(&conn.socket, conn.token, Ready::readable(), PollOpt::edge());

        println!("==== sent: {}", conn.token.0);
        Ok(())
    }
}
