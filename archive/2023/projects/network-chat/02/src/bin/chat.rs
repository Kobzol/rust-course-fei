use chat::chat::Chat;
use chat::client::client_loop;
use chat::messages::{ClientToServerMsg, ServerToClientMsg};
use chat::reader::MessageReader;
use chat::server::handle_client;
use chat::writer::MessageWriter;
use chat::ChatResult;
use clap::Parser;
use log::LevelFilter;
use std::mem::size_of_val;
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

#[derive(Parser)]
enum Args {
    /// Start a chat server.
    Server(ServerArgs),
    /// Start a chat client.
    Client(ClientArgs),
}

#[derive(Parser)]
struct ServerArgs {
    /// Port on which will the server listen.
    #[arg(long, default_value_t = 5555)]
    port: u16,
}

#[derive(Parser)]
struct ClientArgs {
    /// Port on which to connect.
    port: u16,
}

fn main() -> ChatResult<()> {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();

    let args = Args::parse();
    match args {
        Args::Server(args) => {
            run_server(args.port)?;
        }
        Args::Client(args) => {
            run_client(args.port)?;
        }
    }

    Ok(())
}

fn run_client(port: u16) -> ChatResult<()> {
    let socket = TcpStream::connect(("127.0.0.1", port))?;
    client_loop(socket)
}

const MAX_CLIENTS: usize = 3;

fn run_server(port: u16) -> ChatResult<()> {
    log::info!("Server listening on port {port}");

    let server = std::net::TcpListener::bind(("127.0.0.1", port))?;
    let mut connected_clients: Vec<JoinHandle<()>> = vec![];

    let mut chat = Arc::new(Mutex::new(Chat::default()));

    loop {
        let (client, address) = server.accept()?;
        connected_clients = connected_clients
            .into_iter()
            .filter_map(|handle| {
                if handle.is_finished() {
                    handle.join().unwrap();
                    None
                } else {
                    Some(handle)
                }
            })
            .collect();

        if connected_clients.len() >= MAX_CLIENTS {
            log::warn!("Too many clients, bye");
            continue;
        }

        let client = Arc::new(client);
        chat.lock().unwrap().add_client(address, client.clone());

        let chat2 = chat.clone();
        let thread = std::thread::spawn(move || {
            log::info!("Client connected from {address}");
            if let Err(error) = handle_client(address, client, chat2.clone()) {
                log::error!("Client error: {error}");
            }
            log::info!("Client disconnected from {address}");
            chat2.lock().unwrap().remove_client(address);
        });
        connected_clients.push(thread);
    }
}
