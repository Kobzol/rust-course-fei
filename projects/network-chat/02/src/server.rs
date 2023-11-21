use crate::chat::Chat;
use crate::messages::{ClientToServerMsg, ServerToClientMsg};
use crate::reader::MessageReader;
use crate::writer::MessageWriter;
use crate::ChatResult;
use std::arch::x86_64::_mm_sha1msg1_epu32;
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};

pub fn handle_client(
    address: SocketAddr,
    client: Arc<TcpStream>,
    chat: Arc<Mutex<Chat>>,
) -> ChatResult<()> {
    let mut reader = MessageReader::new(&client);

    for msg in reader {
        let msg = match msg {
            Ok(msg) => msg,
            Err(error) => {
                log::error!("Bad client! Error: {error:?}");
                continue;
            }
        };
        match msg {
            ClientToServerMsg::Message(msg) => {
                chat.lock().unwrap().broadcast(address, msg);
            }
        }
    }
    Ok(())
}
