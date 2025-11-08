use std::net::UdpSocket;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub struct Server<F>
where
    F: FnMut(&[u8], SocketAddr) + Send + 'static,
{
    socket: UdpSocket,
    thread_handle: Option<JoinHandle<()>>,
    callback: Option<F>,
    running: Arc<AtomicBool>,
}

impl<F> Server<F>
where
    F: FnMut(&[u8], SocketAddr) + Send + 'static,
{
    pub fn new(socket_addr: SocketAddr, callback: F) -> Self {
        Server {
            socket: UdpSocket::bind(socket_addr).unwrap(),
            thread_handle: None,
            callback: Some(callback),
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn run(&mut self) {
        if self.running.load(Ordering::SeqCst) {
            return;
        }
        if self.callback.is_none() {
            panic!("run can only be called once");
        }

        self.socket
            .set_read_timeout(Some(Duration::from_millis(100)))
            .unwrap();
        self.running.store(true, Ordering::SeqCst);
        let running = self.running.clone();
        let socket = self.socket.try_clone().unwrap();
        let mut callback = self.callback.take().unwrap();

        let thread_handle = thread::spawn(move || {
            let mut buf = [0; 1024];
            while running.load(Ordering::SeqCst) {
                match socket.recv_from(&mut buf) {
                    Ok((amt, src)) => {
                        callback(&buf[..amt], src);
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        eprintln!("Error receiving data: {}", e);
                        break;
                    }
                }
            }
        });

        self.thread_handle = Some(thread_handle);
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.thread_handle.take() {
            handle.join().unwrap();
        }
    }
}
