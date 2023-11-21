#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum ServerToClientMsg {
    Message(String),
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum ClientToServerMsg {
    Message(String),
}
