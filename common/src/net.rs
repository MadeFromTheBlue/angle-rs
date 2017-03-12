use protobuf::*;

use crossbeam::sync::{ArcCell, MsQueue};

use std::io::{self, Read, Write};
use std::net::{TcpStream, Shutdown};
use std::sync::{Arc};
use std::thread::{spawn, JoinHandle};

use byteorder::{ByteOrder, NetworkEndian};

use protobuf::parse_from_bytes;

#[derive(Debug)]
pub enum ConnectionState {
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
    stream: TcpStream,
    out_queue: Arc<MsQueue<O>>,
    out_thread: JoinHandle<()>,
    in_queue: Arc<MsQueue<I>>,
    in_thread: JoinHandle<()>,
    state: Arc<ArcCell<ConnectionState>>,
}

fn read_packet<I: MessageStatic>(stream: &mut TcpStream) -> Result<I, io::Error> {
    let mut size_bytes = [0u8; 4];
    stream.read_exact(&mut size_bytes)?;
    let size = NetworkEndian::read_u32(&size_bytes);

    let mut data: Vec<u8> = (0..size).map(|_| 0u8).collect();
    stream.read_exact(&mut data)?;

    Ok(parse_from_bytes(&mut data[..])?)
}

fn write_packet<O: Message>(stream: &mut TcpStream, pak: O) -> Result<(), io::Error> {
    let data = pak.write_to_bytes()?;

    let mut size_bytes = [0u8; 4];
    NetworkEndian::write_u32(&mut size_bytes, pak.get_cached_size());

    stream.write_all(&size_bytes)?;
    stream.write_all(&data[..])?;

    Ok(())
}

impl<I: MessageStatic, O: Message> Connection<I, O> {
    pub fn send(&self, msg: O) {
        self.out_queue.push(msg);
    }

    pub fn receive(&self) -> Option<I> {
        self.in_queue.try_pop()
    }

    pub fn receive_some(&self) -> I {
        self.in_queue.pop()
    }

    pub fn is_running(&self) -> bool {
        self.state.get().is_running()
    }

    pub fn get_state(&self) -> Arc<ConnectionState> {
        self.state.get()
    }

    pub fn close(self) {
        self.stream.shutdown(Shutdown::Both).unwrap();
        self.out_thread.join().unwrap();
        self.in_thread.join().unwrap();
    }

    pub fn run_on(stream: TcpStream) -> Result<Connection<I, O>, io::Error> {
        use self::ConnectionState::*;

        stream.set_nodelay(true).unwrap();

        let iq = Arc::new(MsQueue::new());
        let oq: Arc<MsQueue<O>> = Arc::new(MsQueue::new());

        let state = Arc::new(ArcCell::new(Arc::new(Running)));

        let iqc = iq.clone();
        let is = stream.try_clone()?;
        let istate = state.clone();
        let it = spawn(move || {
            let mut is = is;
            while istate.get().is_running() {
                iqc.push(match read_packet(&mut is) {
                    Ok(i) => i,
                    Err(e) => {
                        istate.set(Arc::new(Error(io::Error::from(e))));
                        return;
                    }
                });
            }
        });

        let oqc = oq.clone();
        let os = stream.try_clone()?;
        let ostate = state.clone();
        let ot = spawn(move || {
            let mut os = os;
            while ostate.get().is_running() {
                match write_packet(&mut os, oqc.pop()) {
                    Err(e) => {
                        ostate.set(Arc::new(Error(io::Error::from(e))));
                        return;
                    },
                    _ => (),
                }
                os.flush().unwrap();
            }
        });

        Ok(Connection {
            stream: stream,
            out_queue: oq,
            out_thread: ot,
            in_queue: iq,
            in_thread: it,
            state: state,
        })
    }
}