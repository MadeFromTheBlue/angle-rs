use protobuf::*;
use protobuf::core::parse_length_delimited_from_reader;
use std::io;
#[allow(unused)]
use std::net::{Shutdown, TcpStream};

pub struct Conection<P> {
	stream: TcpStream,
	partner: P,
}

impl<I: Message, O: Message, P> Conection<P> {
	pub fn open<A: ToSocketAddrs> 
}