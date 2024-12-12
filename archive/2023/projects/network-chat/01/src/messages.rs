#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum ServerToClientMsg {
    Hello(String),
    Pong,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum ClientToServerMsg {
    Ping,
}
