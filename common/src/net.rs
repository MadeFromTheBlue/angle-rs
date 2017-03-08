use protobuf::*;

use std::io;
use std::net::{ToSocketAddrs};
use std::marker::PhantomData;

use tokio_core::io::{Framed, Io, EasyBuf, Codec};
use tokio_core::net::{TcpStream};
use tokio_core::reactor::Handle;

use futures::{Future};

use byteorder::{ByteOrder, NetworkEndian};

pub struct NetCodec<I, O> {
    i_type: PhantomData<I>,
    o_type: PhantomData<O>,
}

impl<I: MessageStatic, O: Message> NetCodec<I, O> {
    pub fn new() -> NetCodec<I, O> {
        NetCodec {
            i_type: PhantomData,
            o_type: PhantomData,
        }
    }
}

impl<I: MessageStatic, O: Message> Codec for NetCodec<I, O> {
    type In = I;
    type Out = O;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<I>> {
        let avalible = buf.len();

        if avalible < 4 { return Ok(None) }
        else {
            let len = NetworkEndian::read_u32(&buf.as_ref()[..4]) as usize;
            if avalible - 4 >= len {
                Ok(Some(parse_from_bytes(&buf.drain_to(len + 4).as_ref()[4..])?))
            }
            else { Ok(None) }
        }
    }

    fn encode(&mut self, msg: O, buf: &mut Vec<u8>) -> io::Result<()> {
        buf.extend(&[0u8; 4]);
        msg.write_to_vec(buf)?;
        NetworkEndian::write_u32(&mut buf[0..4], msg.get_cached_size());
        Ok(())
    }
}

pub struct Connection<S: Io, I, O> {
    stream: Framed<S, NetCodec<I, O>>,
}

impl<I: MessageStatic, O: Message> Connection<TcpStream, I, O> {
    pub fn connect<A: ToSocketAddrs>(addr: A, handle: &Handle) -> Result<Connection<TcpStream, I, O>, io::Error> {
        let addr = addr.to_socket_addrs()?.next().unwrap();
        Ok(Self::from_stream(TcpStream::connect(&addr, handle).wait()?))
    }
}

impl<S: Io, I: MessageStatic, O: Message> Connection<S, I, O> {
    pub fn from_stream(stream: S) -> Connection<S, I, O> {
        Connection {
            stream: stream.framed(NetCodec::new())
        }
    }

    pub fn get_stream(&self) -> &Framed<S, NetCodec<I, O>> {
        &self.stream
    }

    pub fn get_mut_stream(&mut self) -> &mut Framed<S, NetCodec<I, O>> {
        &mut self.stream
    }

    pub fn as_stream(self) -> Box<Framed<S, NetCodec<I, O>>> {
        box self.stream
    }
}