#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum ClientToServerMsg {
    /// This is the first message in the communication, which should be sent by the client.
    /// When some other client with the same name already exists, the server should respond
    /// with an error "Username already taken" and disconnect the new client.
    Join { name: String },
    /// This message checks that the connection is OK.
    /// The server should respond with [ServerToClientMsg::Pong].
    Ping,
    /// Send a request to list the usernames of users currently connected to the server.
    /// The order of the usernames is not important.
    ListUsers,
    /// Sends a direct message to the user with the given name (`to`).
    /// If the user does not exist, the server responds with an error "User <to> does not exist".
    /// If the client tries to send a message to themselves, the server responds with an error
    /// "Cannot send a DM to yourself".
    SendDM { to: String, message: String },
    /// Sends a message to all currently connected users (except for the sender of the broadcast).
    Broadcast { message: String },
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum ServerToClientMsg {
    /// Response to [ClientToServerMsg::Join].
    Welcome,
    /// Response to [ClientToServerMsg::Ping].
    Pong,
    /// Response to [ClientToServerMsg::ListUsers].
    UserList { users: Vec<String> },
    /// This message is sent by the server to a client that should receive a message
    /// (that was sent either by [ClientToServerMsg::SendDM] or [ClientToServerMsg::Broadcast]).
    Message { from: String, message: String },
    /// This message is returned by the server when an error occurs.
    Error(String),
}
