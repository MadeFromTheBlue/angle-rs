use protobuf::*;
use protobuf::core::parse_length_delimited_from_reader;

use crossbeam::sync::SegQueue;

use std::sync::Arc;
use std::thread::{JoinHandle, spawn};
use std::io;
#[allow(unused)]
use std::net::{Shutdown, TcpStream, ToSocketAddrs};

pub struct Conection<P, I, O> {
	pub in_queue: Arc<SegQueue<I>>,
	pub out_queue: Arc<SegQueue<O>>,
	pub partner: Option<P>,
	thread: JoinHandle<()>,
}

impl<I: MessageStatic, O: Message, P> Conection<P, I, O> {
	pub fn open<A: ToSocketAddrs>(addr: A) -> Result<Conection<P, I, O>, io::Error> {
		Ok(Self::from_stream(TcpStream::connect(addr)?))
	}

	pub fn from_stream(stream: TcpStream) -> Conection<P, I, O> {
		let in_queue = Arc::new(SegQueue::new());
		let in_queue_handle = in_queue.clone();
		let out_queue = Arc::new(SegQueue::new());

		Conection {
			in_queue: in_queue,
			out_queue: out_queue,
			partner: None,
			thread: spawn(move || {
				let mut stream = stream;
				loop {
					in_queue_handle.push(parse_length_delimited_from_reader::<I>(&mut stream).unwrap())
				}
			}),
		}
	}
}