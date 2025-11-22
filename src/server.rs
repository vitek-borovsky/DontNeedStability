use std::net::UdpSocket;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

type F = dyn FnMut(&[u8], SocketAddr) + Send + 'static;
pub struct Server {
    socket: UdpSocket,
    thread_handle: Option<JoinHandle<()>>,
    callback: Option<Box<F>>,
    running: Arc<AtomicBool>,
}

impl Server {
    /// Creates a new `Server` instance, binding to the specified socket address.
    ///
    /// # Arguments
    ///
    /// * `socket_addr` - The `SocketAddr` to bind the server to.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::net::{Ipv4Addr, SocketAddr};
    /// use dont_need_stability::server::Server;
    ///
    /// let socket_addr = SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 8080);
    /// let server = Server::new(socket_addr);
    /// // Server is created but not yet running.
    /// ```
    pub fn new(socket_addr: SocketAddr) -> Self {
        Server {
            socket: UdpSocket::bind(socket_addr).unwrap(),
            thread_handle: None,
            callback: None,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Registers a callback function to be executed when a UDP packet is received.
    ///
    /// The callback function takes the received data as a byte slice and the source `SocketAddr`.
    /// This method can only be called once before `run()`.
    ///
    /// # Arguments
    ///
    /// * `f` - A `Box` containing the callback function.
    ///
    /// # Panics
    ///
    /// Panics if `run()` has already been called.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::net::{Ipv4Addr, SocketAddr};
    /// use dont_need_stability::server::Server;
    ///
    /// let socket_addr = SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 8081);
    /// let mut server = Server::new(socket_addr);
    ///
    /// server.register_callback(Box::new(|data, src| {
    ///     println!("Received {:?} bytes from {}", data.len(), src);
    /// }));
    /// // Callback is registered, server can now be run.
    /// ```
    pub fn register_callback(&mut self, f: Box<F>) {
        self.callback = Some(f);
    }

    /// Starts the server, listening for incoming UDP packets and executing the registered callback.
    ///
    /// This method spawns a new thread to handle packet reception. It will panic if no callback
    /// has been registered or if `run()` is called multiple times.
    ///
    /// # Panics
    ///
    /// * If no callback has been registered using `register_callback()`.
    /// * If `run()` is called more than once on the same server instance.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
    /// use std::thread;
    /// use std::time::Duration;
    /// use dont_need_stability::server::Server;
    ///
    /// let server_addr = SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 8082);
    /// let client_addr = SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 8083);
    ///
    /// let mut server = Server::new(server_addr);
    /// server.register_callback(Box::new(|data, src| {
    ///     assert_eq!(data, b"hello");
    ///     println!("Server received: {:?} from {}", String::from_utf8_lossy(data), src);
    /// }));
    /// server.run();
    ///
    /// // Simulate a client sending a packet
    /// let client_socket = UdpSocket::bind(client_addr).unwrap();
    /// client_socket.send_to(b"hello", server_addr).unwrap();
    ///
    /// thread::sleep(Duration::from_millis(100)); // Give server time to process
    /// server.stop();
    /// ```
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

    /// Stops the server, joining the background thread.
    ///
    /// This method sets an internal flag to signal the background thread to stop,
    /// then waits for the thread to finish its execution.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::net::{Ipv4Addr, SocketAddr};
    /// use dont_need_stability::server::Server;
    ///
    /// let socket_addr = SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 8084);
    /// let mut server = Server::new(socket_addr);
    /// server.register_callback(Box::new(|_, _| { /* do nothing */ }));
    /// server.run();
    /// // ... server is running ...
    /// server.stop();
    /// // Server is now stopped.
    /// ```
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.thread_handle.take() {
            handle.join().unwrap();
        }
    }
}
