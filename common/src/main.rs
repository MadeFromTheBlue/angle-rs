#![feature(box_syntax, box_patterns)]

extern crate protobuf;
extern crate crossbeam;
extern crate tokio_core;
extern crate byteorder;
extern crate futures;

pub mod net;
pub mod netdata;

pub fn main() {
    use tokio_core::net::TcpListener;
    use tokio_core::reactor::Core;
    use futures::{future, Stream, Future};
    use net::*;
    use netdata;

    let mut core = Core::new().unwrap();
    let handle1 = core.handle();
    let handle2 = core.handle();

    let addr = "127.0.0.1:12345".parse().unwrap();
    let sock = TcpListener::bind(&addr, &handle2).unwrap();

    println!("Starting server...");
    let server = sock.incoming().for_each(move |(sock, _)| {
        println!("Client connected");
        let conn = Connection::<netdata::FromClient, netdata::FromServer>::run_on(sock, &handle1);
        while conn.is_running() {
            if let Some(pak) = conn.receive() { println!("{:?}", pak); }
        }
        println!("{:?}", conn.get_state());
        future::ok(())
    }).or_else(|_| {
        future::err(())
    });


    handle2.spawn(server);

    println!("Starting client...");
    let client = Connection::<netdata::FromServer, netdata::FromClient>::run(&addr, &handle2);

    let mut pak = netdata::FromClient::new();
    let mut chat = netdata::CChat::new();
    chat.set_msg("hello, world!".to_owned());
    pak.set_chat(chat);

    client.send(pak);

    loop {}
}