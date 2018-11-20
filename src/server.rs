use std::io;
use std::net::{SocketAddr/*, TcpListener, TcpStream*/};
use std::io::prelude::*;
use mio::*;
use mio::net::{TcpListener, TcpStream};
use std::collections::HashMap;

use request::*;
use response::*;
use static_router;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ConnectionState {
    Loading,
    Sending,
    Closed,
}

#[derive(Debug)]
pub struct Connection {
    token: Token,
    state: ConnectionState,
    socket: TcpStream,
    addr: SocketAddr,
    buf: Vec<u8>,
    pos: usize,
    request: Option<Request>
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
            pos: 0,
            request: None
        };
        conn.buf.resize(INITIAL_BUF_SIZE, 0);
        conn
    }
    
    fn load(&mut self, poll: &Poll) -> io::Result<()> {
        loop {
            if self.buf.len() == self.pos {
                let new_len = 2 * self.buf.len();
                self.buf.resize(new_len, 0);
            }
            match self.socket.read(&mut self.buf[self.pos..]) {
                Ok(0) => {
                    self.state = ConnectionState::Closed;
                    return Ok(());
                }
                Ok(n) => {
                    println!("==== chunk: {}", self.token.0);
                    let prev_pos = self.pos;
                    self.pos += n;
                    io::stdout().write(&self.buf[prev_pos .. self.pos]);

                    if self.buf[.. self.pos].ends_with(b"\r\n\r\n") { //ends_with?
                        println!("==== end header");
                        let req_result = Request::parse(&self.buf[.. self.pos]);
                        match req_result {
                            Ok(req) => {
                                self.state = ConnectionState::Sending;
                                poll.reregister(
                                    &self.socket, self.token, Ready::writable(), PollOpt::edge())?;
                                self.request = Some(req);
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

    fn send(&mut self, poll: &Poll) -> io::Result<()> {
        let mut res = Response::ok();
        static_router::serve(self.request.as_ref().unwrap(), &mut res);
        // let body = b"<h1>Hello world!</h1>";
        // res.write(body);
        // res.set_header("content-length", &body.len().to_string());
        res.end(&self.socket);

        // clear buffer
        self.pos = 0;

        self.state = ConnectionState::Loading;
        poll.reregister(&self.socket, self.token, Ready::readable(), PollOpt::edge())?;

        println!("==== sent: {}", self.token.0);
        Ok(())
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
                                ConnectionState::Loading => conn.load(&poll)?,
                                ConnectionState::Sending => conn.send(&poll)?,
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
    }

    fn accept(&mut self, listener: &TcpListener, poll: &Poll) -> io::Result<()> {
        loop { 
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
}
