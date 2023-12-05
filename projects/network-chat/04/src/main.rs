use std::cell::RefCell;
use std::net::SocketAddr;
use std::rc::Rc;
use std::time::Duration;

use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

struct Chat {
    connected_clients: u64,
}

async fn client_loop(
    mut client: TcpStream,
    addr: SocketAddr,
    chat: Rc<RefCell<Chat>>,
) -> std::io::Result<()> {
    println!("Client has connected: {addr}");
    chat.borrow_mut().connected_clients += 1;

    let mut buffer = [0; 1024];
    loop {
        // Wait until the first future completes
        tokio::select! {
            read = client.read(&mut buffer) => {
                if read? == 0 {
                    break;
                }
            }
            _ = tokio::time::sleep(Duration::from_secs(3)) => {
                println!("Client {addr} timeouted");
                break;
            }
        }
        println!("{}", String::from_utf8_lossy(&buffer));
    }
    println!("Client {addr} disconnected");
    chat.borrow_mut().connected_clients -= 1;
    println!("Current clients: {}", chat.borrow().connected_clients);

    Ok(())
}

async fn server_loop() -> std::io::Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:5555").await?;

    let chat = Rc::new(RefCell::new(Chat {
        connected_clients: 0,
    }));
    loop {
        let (client, addr) = listener.accept().await?;
        let client_fut = client_loop(client, addr, chat.clone());
        tokio::task::spawn_local(client_fut);
    }
}

fn main() -> std::io::Result<()> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let localset = tokio::task::LocalSet::new();
        localset.run_until(server_loop()).await
    })
}
