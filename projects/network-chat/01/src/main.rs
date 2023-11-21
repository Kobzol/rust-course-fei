use chat::messages::{ClientToServerMsg, ServerToClientMsg};
use chat::reader::MessageReader;
use chat::writer::MessageWriter;
use chat::ChatResult;
use clap::Parser;
use std::net::TcpStream;

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
    let mut reader = MessageReader::new(&socket);
    for msg in reader {
        let msg = msg?;
        match msg {
            ServerToClientMsg::Hello(msg) => {
                println!("Server sent hello: {msg}");
            }
            ServerToClientMsg::Pong => {
                println!("Server sent pong");
            }
        }
    }

    Ok(())
}

fn run_server(port: u16) -> ChatResult<()> {
    println!("Server listening on port {port}");

    let server = std::net::TcpListener::bind(("127.0.0.1", port))?;
    let (client, address) = server.accept()?;
    println!("Client connected from {address}");

    let mut reader = MessageReader::new(&client);
    let mut writer = MessageWriter::new(&client);

    writer.send(ServerToClientMsg::Hello("Welcome".to_string()))?;

    for msg in reader {
        let msg = match msg {
            Ok(msg) => msg,
            Err(error) => {
                println!("Bad client! Error: {error:?}");
                continue;
            }
        };
        match msg {
            ClientToServerMsg::Ping => {
                writer.send(ServerToClientMsg::Pong)?;
            }
        }
    }

    println!("Client disconnected from {address}");
    Ok(())
}
