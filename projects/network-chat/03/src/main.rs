use crate::reader::MessageReader;
use crate::writer::MessageWriter;
use epoll::ControlOptions::{EPOLL_CTL_ADD, EPOLL_CTL_DEL};
use epoll::{Event, Events};
use std::io::{ErrorKind, Read};
use std::net::{SocketAddr, TcpListener};
use std::ops::Deref;
use std::os::fd::{AsRawFd, RawFd};
use std::time::{Duration, Instant};

mod reader;
mod writer;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
enum Message {
    Hello,
    Authenticate,
    ChatMessage,
}

const TIMEOUT: Duration = Duration::from_secs(30);

struct Client {
    reader: MessageReader<Message>,
    addr: SocketAddr,
    connected: bool,
    last_activity: Instant,
    state: ClientState,
}

impl Client {
    fn until_timeout(&self) -> Duration {
        TIMEOUT.saturating_sub(self.last_activity.elapsed())
    }
}

enum ClientState {
    Connected,
    ReceivedHello { received_hello: Instant },
    ReceivedAuthenticate,
}

trait Future {
    fn progress(
        &mut self,
        epoll: RawFd,
        new_futures: &mut Vec<Box<dyn Future>>,
        fds: &mut Vec<RawFd>,
    );
}

struct ServerFuture {
    listener: TcpListener,
    state: ServerFutureState,
}

enum ServerFutureState {
    Start,
    Accepting,
}

impl Future for ServerFuture {
    fn progress(
        &mut self,
        epoll: RawFd,
        new_futures: &mut Vec<Box<dyn Future>>,
        fds: &mut Vec<RawFd>,
    ) -> Option<()> {
        match self.state {
            ServerFutureState::Start => {
                let event = Event::new(Events::EPOLLIN, self.listener.as_raw_fd() as u64);
                epoll::ctl(epoll, EPOLL_CTL_ADD, self.listener.as_raw_fd(), event).unwrap();
                self.state = ServerFutureState::Accepting;
                fds.push(self.listener.as_raw_fd());
            }
            ServerFutureState::Accepting => {
                let (client, addr) = match self.listener.accept() {
                    Ok(ret) => ret,
                    Err(error) if error.kind() == ErrorKind::WouldBlock => {
                        return;
                    }
                    Err(error) => Err(error).unwrap(),
                };
                println!("Client connected: {addr}");
                client.set_nonblocking(true).unwrap();

                let client_future = Box::new(ClientFuture {
                    reader: MessageReader::new(client),
                    addr,
                    connected: true,
                    state: ClientFutureState::Start,
                });
                new_futures.push(client_future);
            }
        }
    }
}

struct ClientFuture {
    reader: MessageReader<Message>,
    addr: SocketAddr,
    connected: bool,
    state: ClientFutureState,
}

enum ClientFutureState {
    Start,
}

impl Future for ClientFuture {
    fn progress(
        &mut self,
        epoll: RawFd,
        new_futures: &mut Vec<Box<dyn Future>>,
        fds: &mut Vec<RawFd>,
    ) {
    }
}

fn create_server_future() -> ServerFuture {
    let listener = TcpListener::bind("127.0.0.1:5555").unwrap();
    listener.set_nonblocking(true).unwrap();
    ServerFuture {
        listener,
        state: ServerFutureState::Start,
    }
}

struct Task {
    future: Box<dyn Future>,
    fds: Vec<RawFd>,
}

#[derive(Default)]
struct EventLoop {
    tasks: Vec<Task>,
}

impl EventLoop {
    fn add<F: Future + 'static>(&mut self, f: F) {
        self.tasks.push(Task {
            future: Box::new(f),
            fds: vec![],
        });
    }
    fn run(mut self) -> std::io::Result<()> {
        let epoll = epoll::create(false)?;

        let mut new_futures = vec![];
        for task in &mut self.tasks {
            task.future.progress(epoll, &mut new_futures, &mut task.fds);
        }
        assert!(new_futures.is_empty());

        loop {
            let mut events = [Event::new(Events::empty(), 0); 1024];
            let event_count = epoll::wait(epoll, -1, &mut events)?;
            let mut new_futures = vec![];

            for event in &events[..event_count] {
                let fd = event.data;
                for task in &mut self.tasks {
                    if task.fds.contains(&(fd as RawFd)) {
                        task.future.progress(epoll, &mut new_futures, &mut task.fds);
                    }
                }
            }
            for mut future in new_futures {
                let mut fds = vec![];
                future.progress(epoll, &mut Vec::new(), &mut fds);
                self.tasks.push(Task { future, fds })
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut eventloop = EventLoop::default();
    eventloop.add(create_server_future());
    eventloop.run()?;

    Ok(())
}

fn client_logic(client: &mut Client) {
    match client.state {
        ClientState::Connected => {
            let msg = match read_message(client) {
                Some(ret) => ret,
                None => return,
            };
            assert!(matches!(msg, Message::Hello));
            let received = Instant::now();
            client.state = ClientState::ReceivedHello {
                received_hello: received,
            };
        }
        ClientState::ReceivedHello { received_hello } => {
            let msg = match read_message(client) {
                Some(ret) => ret,
                None => return,
            };
            assert!(matches!(msg, Message::Authenticate));
            client.state = ClientState::ReceivedAuthenticate;
        }
        ClientState::ReceivedAuthenticate => {
            let msg = match read_message(client) {
                Some(ret) => ret,
                None => return,
            };
            assert!(matches!(msg, Message::ChatMessage));
            println!("{msg:?}");
        }
    }
}

fn read_message(client: &mut Client) -> Option<Message> {
    match client.reader.read() {
        Some(Ok(msg)) => {
            println!("{msg:?}");
            client.last_activity = Instant::now();
            Some(msg)
        }
        Some(Err(error)) if error.kind() == ErrorKind::WouldBlock => {
            return None;
        }
        Some(Err(error)) => {
            println!("Client {} error: {error:?}", client.addr);
            client.connected = false;
            return None;
        }
        None => {
            // client disconnected
            client.connected = false;
            return None;
        }
    }
}
