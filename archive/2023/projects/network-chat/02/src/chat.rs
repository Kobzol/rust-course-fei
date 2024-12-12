use crate::messages::ServerToClientMsg;
use crate::writer::MessageWriter;
use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};
use std::sync::Arc;

pub struct Client {
    socket: Arc<TcpStream>,
}

#[derive(Default)]
pub struct Chat {
    clients: HashMap<SocketAddr, Client>,
}

impl Chat {
    pub fn add_client(&mut self, address: SocketAddr, socket: Arc<TcpStream>) {
        log::info!("Connected client {address}");
        self.broadcast(address, format!("[{address} connected]"));
        assert!(self.clients.insert(address, Client { socket }).is_none());
    }
    pub fn remove_client(&mut self, address: SocketAddr) {
        log::info!("Disconnected client {address}");
        self.broadcast(address, format!("[{address} disconnected]"));
        assert!(self.clients.remove(&address).is_some());
    }
    pub fn broadcast(&self, source: SocketAddr, message: String) {
        self.clients
            .iter()
            .filter(|(address, _)| **address != source)
            .for_each(|(address, socket)| {
                let content = format!("{source}: {message}");
                let mut writer = MessageWriter::new(&socket.socket);
                log::info!("Sending {content} to {address}");
                writer.send(ServerToClientMsg::Message(content)).unwrap();
            })
    }
}
