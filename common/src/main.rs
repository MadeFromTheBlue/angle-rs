#![feature(box_syntax, box_patterns)]

extern crate protobuf;
extern crate crossbeam;
extern crate tokio_core;
extern crate byteorder;
extern crate futures;

pub mod net;
pub mod netdata;

pub fn main() {
    use std::thread::spawn;
    use std::net::{SocketAddr, TcpStream, TcpListener};
    use net::*;
    use netdata;

    let addr: SocketAddr = "127.0.0.1:12345".parse().unwrap();
    let sock = TcpListener::bind(&addr).unwrap();

    spawn(move || {
        println!("Starting server...");
        for conn in sock.incoming() {
            println!("Client connected");
            let conn = Connection::<netdata::FromClient, netdata::FromServer>::run_on(conn.unwrap()).unwrap();
            println!("{:?}", conn.receive_some());
            println!("{:?}", conn.receive_some());
            conn.close();
        }
    });

    println!("Starting client...");
    let client = TcpStream::connect(&addr).unwrap();
    let client = Connection::<netdata::FromServer, netdata::FromClient>::run_on(client).unwrap();

    let mut pak = netdata::FromClient::new();
    let mut chat = netdata::CChat::new();
    chat.set_msg("hello, world!".to_owned());
    pak.set_chat(chat);

    client.send(pak);

    let mut pak = netdata::FromClient::new();
    let mut chat = netdata::CChat::new();
    chat.set_msg("one more".to_owned());
    pak.set_chat(chat);

    client.send(pak);

    loop {}
}