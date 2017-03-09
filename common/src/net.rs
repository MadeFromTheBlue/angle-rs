use protobuf::*;

use crossbeam::sync::{ArcCell, SegQueue};

use std::io;
use std::net::{SocketAddr};
use std::marker::PhantomData;
use std::sync::{Arc};

use futures::{future, Sink, Future, Stream};
use futures::sync::mpsc;
use tokio_core::net::TcpStream;
use tokio_core::io::{Io, Codec, EasyBuf};
use tokio_core::reactor::Handle;

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

pub enum ConnectionState {
    Starting,
    NotConnected(io::Error),
    Running,
    Error(io::Error),
    Shutdown,
}

impl ConnectionState {
    pub fn is_running(&self) -> bool {
        use self::ConnectionState::*;
        match *self {
            Running => true,
            _ => false,
        }
    }
}

pub struct Connection<I, O> {
    out_queue: mpsc::UnboundedSender<O>,
    in_queue: Arc<SegQueue<I>>,
    state: Arc<ArcCell<ConnectionState>>,
}

impl<I: MessageStatic, O: Message> Connection<I, O> {
    pub fn send(&self, msg: O) {
        match (&self.out_queue).send(msg) {
            Ok(_) => (),
            Err(_) => (),
        }
    }

    pub fn receive(&self) -> Option<I> {
        self.in_queue.try_pop()
    }

    pub fn is_running(&self) -> bool {
        self.state.get().is_running()
    }

    pub fn get_state(&self) -> Arc<ConnectionState> {
        self.state.get()
    }

    pub fn run(addr: &SocketAddr, run: &Handle) -> Connection<I, O> {
        use self::ConnectionState::*;

        let iqr = Arc::new(SegQueue::new());
        let iqs = iqr.clone();
        let (oqs, oqr) = mpsc::unbounded();

        let state = Arc::new(ArcCell::new(Arc::new(Starting)));
        let cstate = state.clone();
        let jstate = state.clone();
        let estate = state.clone();

        let remote = run.remote();

        let stream = TcpStream::connect(addr, run)
            .and_then(move |stream| {
                let (to, from) = stream.framed(NetCodec::<I, O>::new()).split();

                let sender = to
                    .send_all(oqr.map_err(|_| io::ErrorKind::Other));

                let receiver = from
                    .for_each(move |v| {
                        iqs.push(v);
                        future::ok::<(), io::Error>(())
                    });

                let joined = sender.join(receiver)
                    .and_then(|_| {
                        jstate.set(Arc::new(Shutdown));
                        future::ok(())
                    }).or_else(|e| {
                        estate.set(Arc::new(Error(e)));
                        future::err(())
                    });

                // TODO join

                future::ok(())
            }).or_else(move |e| {
                cstate.set(Arc::new(NotConnected(e)));
                future::err(())
            });

        run.spawn(stream);

        Connection {
            out_queue: oqs,
            in_queue: iqr,
            state: state,
        }
    }
}