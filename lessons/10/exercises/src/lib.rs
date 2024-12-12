#![warn(clippy::await_holding_refcell_ref)]

//! TODO: implement a simple chat server using async/await and tokio (still using non-blocking I/O)
//!
//! The chat server should behave identically as the one from last week, with one new feature.
//! However, it should be implemented using async/await (with non-blocking I/O) and run on a
//! single thread. It should still support concurrency and allow the connection of multiple clients
//! at once.
//!
//! Ideally, reuse your implementation from **week 08** (from two weeks ago), but remove threads and
//! add `await` and `tokio`. Remember not to block inside `async` functions and blocks.
//!
//! Your code will run inside [`tokio::task::LocalSet`], so you can use [`tokio::task::spawn_local`]
//! to spawn new asynchronous tasks.

/// The following modules were prepared for you. You should not need to modify them.
///
/// Take a look at this file to see how should the individual messages be handled
mod messages;
/// Message reading
mod reader;
/// Message writing
mod writer;

#[derive(Copy, Clone)]
struct ServerOpts {
    /// Maximum number of clients that can be connected to the server at once.
    max_clients: usize,
}

/// Representation of a running server
struct RunningServer {
    /// Port on which the server is running
    port: u16,
    /// Main future of the server
    future: Pin<Box<dyn Future<Output = anyhow::Result<()>>>>,
    /// Channel that can be used to tell the server to stop
    tx: tokio::sync::oneshot::Sender<()>,
}

/// TODO: implement the following asynchronous function called `run_server`
/// It should start a chat server on a TCP/IP port assigned to it by the operating system and
/// return a [`RunningServer`]. Note that the function is asynchronous, but it should create a
/// separate future that will run the server loop, and return that future inside the returned
/// [`RunningServer`].
///
/// You should not create any threads anywhere in this assignment. Everything should run on a single
/// thread. You should not need `Arc<Mutex<...>>` anywhere, although `Rc<RefCell<...>>` could be
/// useful. Just be careful not to hold any `RefCell` borrows across `await` points if some other
/// async task might also access that same `RefCell` and break the alias xor mut rule.
///
/// The server should implement the messages described in `messages.rs`, see the message comments
/// for more details. The details are the same as last week, with one exception described below.
///
/// # Client connection
/// When a client connects to the server, it should send a `Join` message.
/// - If the client does not send a `Join` message within two seconds, the server should
/// send an error "Timed out waiting for Join" and disconnect the client immediately.
/// - If it sends anything else, the server should respond with an error "Unexpected message received"
/// and disconnect the client immediately.
/// - If the user sends a Join message (with a unique username), the server should respond with
/// the `Welcome` message.
///
/// Then it should start receiving requests from the client.
/// - If the client ever sends the `Join` message again, the server should respond with an error
/// "Unexpected message received" and disconnect the client immediately.
/// - **(NEW)** If the client does not send any message in three seconds AND it does not receive
/// any message (through a DM or a broadcast) within that duration, the server should respond with
/// an error "Timeouted" and disconnect the client immediately. This three second timer is refreshed
/// everytime the client sends something or receives a DM/broadcast.
///
/// # Maximum number of clients
/// When a client connects and there are already `opts.max_clients` other clients connected, the
/// server should respond with an error "Server is full" and disconnect the client immediately.
/// Note that if the server is full, the client should be disconnected even before it sends the
/// `Join` message.
///
/// # Graceful shutdown
/// Your server should react to a message sent through the oneshot channel that you should create
/// in `RunningServer`. When a message is received on this channel, the server should:
/// 1) Stop receiving new TCP/IP connections
/// 2) Correctly disconnect all connected users (bonus, see [`tests::drop_clients_on_shutdown`])
/// 3) Wait until all async tasks that it has created has completed executing (bonus)
/// The rest is handled by the test infrastructure.
///
/// See tests for more details.
async fn run_server(opts: ServerOpts) -> anyhow::Result<RunningServer> { todo!() }


#[cfg(test)]
mod tests {
    use crate::messages::{ClientToServerMsg, ServerToClientMsg};
    use crate::reader::MessageReader;
    use crate::writer::MessageWriter;
    use crate::{run_server, ServerOpts};
    use std::cell::{Cell, RefCell};
    use std::future::Future;
    use std::rc::Rc;
    use std::time::Duration;
    use tokio::io::AsyncWriteExt;
    use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
    use tokio::net::TcpStream;
    use tokio::task::LocalSet;

    // If you're struggling with this test, comment it and implement the rest of the
    // functionality first.
    #[tokio::test]
    async fn empty_server_shuts_down() {
        run_test(opts(2), |_| async move { Ok(()) }).await;
    }

    #[tokio::test]
    async fn max_clients() {
        run_test(opts(2), |server| async move {
            let _client = server.client().await;
            let _client2 = server.client().await;

            let mut client3 = server.client().await;
            client3.expect_error("Server is full").await;
            client3.check_closed().await;

            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn max_clients_after_client_leaves() {
        run_test(opts(2), |spawner| async move {
            let _client = spawner.client().await;
            let client2 = spawner.client().await;
            client2.close().await;

            sleep(500).await;

            let mut client3 = spawner.client().await;
            client3.join("Foo").await;

            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn max_clients_herd() {
        let max_clients = 5;
        run_test(opts(max_clients), |spawner| async move {
            let client_count = 50;

            let errors = Rc::new(Cell::new(0));
            let successes = Rc::new(Cell::new(0));

            let joined_clients = Rc::new(RefCell::new(vec![]));

            let futs = (0..client_count).map(|client_id| {
                let errors = errors.clone();
                let successes = successes.clone();
                let joined_clients = joined_clients.clone();

                async move {
                    let mut client = spawner.client().await;
                    let _ = client
                        .try_send(ClientToServerMsg::Join {
                            name: format!("Client {client_id}"),
                        })
                        .await;
                    match client.recv().await {
                        ServerToClientMsg::Error(_) => {
                            errors.set(errors.get() + 1);
                        }
                        ServerToClientMsg::Welcome => {
                            successes.set(successes.get() + 1);
                            // Make sure that the client doesn't disconnect
                            joined_clients.borrow_mut().push(client);
                        }
                        msg => {
                            panic!("Unexpected message {msg:?}");
                        }
                    }
                }
            });
            futures_util::future::join_all(futs).await;

            assert_eq!(errors.get(), client_count - max_clients);
            assert_eq!(successes.get(), max_clients);

            drop(joined_clients);

            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn list_users_before_join() {
        run_test(opts(2), |spawner| async move {
            let mut client = spawner.client().await;
            client.send(ClientToServerMsg::ListUsers).await;
            client.expect_error("Unexpected message received").await;

            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn join_after_half_sec() {
        run_test(opts(2), |spawner| async move {
            let mut client = spawner.client().await;
            sleep(500).await;
            client.join("Foo").await;
            assert_eq!(client.list_users().await, vec!["Foo".to_string()]);

            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn join_timeout() {
        run_test(opts(2), |spawner| async move {
            let mut client = spawner.client().await;
            sleep(3000).await;
            match client
                .try_send(ClientToServerMsg::Join {
                    name: "Bilbo".to_string(),
                })
                .await
            {
                Ok(_) => {
                    client.expect_error("Timed out waiting for Join").await;
                }
                Err(_) => {}
            }

            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn duplicated_join() {
        run_test(opts(2), |spawner| async move {
            let mut client = spawner.client().await;
            client.join("Foo").await;
            client
                .send(ClientToServerMsg::Join {
                    name: "Bar".to_string(),
                })
                .await;
            client.expect_error("Unexpected message received").await;

            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn error_then_disconnect() {
        run_test(opts(2), |spawner| async move {
            let mut client = spawner.client().await;
            client.join("Foo").await;
            client
                .send(ClientToServerMsg::Join {
                    name: "Bar".to_string(),
                })
                .await;
            client.close().await;

            let mut client2 = spawner.client().await;
            client2.join("Bar").await;

            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn duplicated_username() {
        run_test(opts(2), |spawner| async move {
            let mut client = spawner.client().await;
            client.join("Foo").await;

            let mut client2 = spawner.client().await;
            client2
                .send(ClientToServerMsg::Join {
                    name: "Foo".to_string(),
                })
                .await;
            client2.expect_error("Username already taken").await;

            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn ping() {
        run_test(opts(2), |spawner| async move {
            let mut luca = spawner.client().await;
            luca.join("Luca").await;
            luca.ping().await;

            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn ping_before_join() {
        run_test(opts(2), |spawner| async move {
            let mut client = spawner.client().await;
            client.send(ClientToServerMsg::Ping).await;
            client.expect_error("Unexpected message received").await;

            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn list_users_reconnect() {
        run_test(opts(2), |spawner| async move {
            let mut client = spawner.client().await;
            client.join("Foo").await;
            client.close().await;

            let mut client = spawner.client().await;
            client.join("Foo").await;
            assert_eq!(client.list_users().await, vec!["Foo".to_string()]);

            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn list_users_self() {
        run_test(opts(2), |spawner| async move {
            let mut client = spawner.client().await;
            client.join("Martin").await;
            assert_eq!(client.list_users().await, vec!["Martin".to_string()]);

            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn list_users_ignore_not_joined_users() {
        run_test(opts(2), |spawner| async move {
            let _client = spawner.client().await;
            let mut client2 = spawner.client().await;
            client2.join("Joe").await;
            assert_eq!(client2.list_users().await, vec!["Joe".to_string()]);

            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn list_users_after_error() {
        run_test(opts(2), |spawner| async move {
            let mut client = spawner.client().await;
            client.join("Terrence").await;

            let mut client2 = spawner.client().await;
            client2.join("Joe").await;

            client
                .send(ClientToServerMsg::Join {
                    name: "Barbara".to_string(),
                })
                .await;

            sleep(1000).await;

            assert_eq!(client2.list_users().await, vec!["Joe".to_string()]);

            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn list_users() {
        run_test(opts(2), |spawner| async move {
            let mut client = spawner.client().await;
            client.join("Terrence").await;

            let mut client2 = spawner.client().await;
            client2.join("Joe").await;
            assert_eq!(
                client2.list_users().await,
                vec!["Joe".to_string(), "Terrence".to_string()]
            );
            client2.close().await;

            sleep(1000).await;

            assert_eq!(client.list_users().await, vec!["Terrence".to_string()]);

            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn dm_nonexistent_user() {
        run_test(opts(2), |spawner| async move {
            let mut client = spawner.client().await;
            client.join("Mark").await;
            client.dm("Fiona", "Hi").await;
            client.expect_error("User Fiona does not exist").await;

            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn dm_self() {
        run_test(opts(2), |spawner| async move {
            let mut client = spawner.client().await;
            client.join("Xal'atath").await;
            client.dm("Xal'atath", "I'm so lonely :(").await;
            client.expect_error("Cannot send a DM to yourself").await;

            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn dm_other() {
        run_test(opts(2), |spawner| async move {
            let mut terrence = spawner.client().await;
            terrence.join("Terrence").await;

            let mut joe = spawner.client().await;
            joe.join("Joe").await;

            terrence.dm("Joe", "How you doin'").await;
            joe.expect_message("Terrence", "How you doin'").await;

            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn dm_spam() {
        run_test(opts(2), |spawner| async move {
            let mut diana = spawner.client().await;
            diana.join("Diana").await;

            let mut francesca = spawner.client().await;
            francesca.join("Francesca").await;

            let count = 10000;

            // Let's say that someone is spamming you...
            let t1 = async move {
                for _ in 0..count {
                    diana
                        .dm("Francesca", "Can I borrow your brush? Pleeeeeease :(((")
                        .await;
                }
            };

            // ...so you get angry, and start spamming them back.
            // But you make a critical *error*, because you're sending the message
            // to the wrong account.
            // Can your chat server handle that?
            let t2 = async move {
                for _ in 0..count {
                    francesca.dm("Daina", "NO! Get your own!").await;
                    match francesca.recv().await {
                        ServerToClientMsg::Message { from, message } => {
                            assert_eq!(from, "Diana");
                            assert_eq!(message, "Can I borrow your brush? Pleeeeeease :(((");
                        }
                        ServerToClientMsg::Error(error) => {
                            assert_eq!(error, "User Daina does not exist");
                        }
                        msg => panic!("Unexpected message {msg:?}"),
                    }
                }
                // Francesca should receive count * 2 messages, `count` from Diana and `count`
                // error messages
                for _ in 0..count {
                    match francesca.recv().await {
                        ServerToClientMsg::Message { from, message } => {
                            assert_eq!(from, "Diana");
                            assert_eq!(message, "Can I borrow your brush? Pleeeeeease :(((");
                        }
                        ServerToClientMsg::Error(error) => {
                            assert_eq!(error, "User Daina does not exist");
                        }
                        msg => panic!("Unexpected message {msg:?}"),
                    }
                }
            };

            // Wait until both processes complete
            let t1 = tokio::task::spawn_local(t1);
            let t2 = tokio::task::spawn_local(t2);
            let (ret1, ret2) = tokio::join!(t1, t2);
            Ok(ret1.and(ret2)?)
        })
        .await;
    }

    #[tokio::test]
    async fn dm_spam_2() {
        // Meanwhile, in a parallel universe...
        run_test(opts(2), |spawner| async move {
            let mut diana = spawner.client().await;
            diana.join("Diana").await;

            let mut francesca = spawner.client().await;
            francesca.join("Francesca").await;

            let count = 10000;

            // Let's say that someone is spamming you...
            let t1 = async move {
                for _ in 0..count {
                    diana
                        .dm("Francesca", "Can I borrow your brush? Pleeeeeease :(((")
                        .await;
                }
            };

            // ...so you get angry, and start spamming them back.
            // But you make a critical *error*, because you push the wrong button and start
            // sending pings to the server instead.
            // Can your chat server handle that?
            let t2 = async move {
                for _ in 0..count {
                    francesca.send(ClientToServerMsg::Ping).await;
                    match francesca.recv().await {
                        ServerToClientMsg::Message { from, message } => {
                            assert_eq!(from, "Diana");
                            assert_eq!(message, "Can I borrow your brush? Pleeeeeease :(((");
                        }
                        ServerToClientMsg::Pong => {}
                        msg => panic!("Unexpected message {msg:?}"),
                    }
                }
                // Francesca should receive count * 2 messages, `count` from Diana and `count`
                // pong messages
                for _ in 0..count {
                    match francesca.recv().await {
                        ServerToClientMsg::Message { from, message } => {
                            assert_eq!(from, "Diana");
                            assert_eq!(message, "Can I borrow your brush? Pleeeeeease :(((");
                        }
                        ServerToClientMsg::Pong => {}
                        msg => panic!("Unexpected message {msg:?}"),
                    }
                }
            };

            let t1 = tokio::task::spawn_local(t1);
            let t2 = tokio::task::spawn_local(t2);
            let (ret1, ret2) = tokio::join!(t1, t2);
            Ok(ret1.and(ret2)?)
        })
        .await;
    }

    #[tokio::test]
    async fn broadcast_empty() {
        run_test(opts(2), |spawner| async move {
            let mut ji = spawner.client().await;
            ji.join("Ji").await;
            ji.send(ClientToServerMsg::Broadcast {
                message: "Haaaaaai!".to_string(),
            })
            .await;
            ji.ping().await;

            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn broadcast() {
        run_test(opts(10), |spawner| async move {
            let mut niko = spawner.client().await;
            niko.join("Niko").await;

            let users: Vec<_> = (0..5)
                .map(|i| async move {
                    let mut client = spawner.client().await;
                    client.join(&format!("NPC {i}")).await;
                    client
                })
                .collect();
            let users: Vec<Client> = futures_util::future::join_all(users).await;

            niko.send(ClientToServerMsg::Broadcast {
                message: "Borrow this!".to_string(),
            })
            .await;
            niko.ping().await;

            for mut user in users {
                user.expect_message("Niko", "Borrow this!").await;
            }

            Ok(())
        })
        .await;
    }

    #[tokio::test]
    async fn message_timeout() {
        run_test(opts(2), |spawner| async move {
            let mut niko = spawner.client().await;
            niko.join("Niko").await;

            sleep(1000).await;
            niko.ping().await;
            sleep(1000).await;
            niko.list_users().await;
            sleep(4000).await;

            niko.expect_error("Timeouted").await;
            niko.check_closed().await;

            Ok(())
        })
        .await;
    }

    // This test runs for ~10s
    #[tokio::test]
    async fn message_timeout_receiving_dms() {
        run_test(opts(10), |spawner| async move {
            // Do not timeout user if he is receiving DMs
            let mut niko = spawner.client().await;
            niko.join("Niko").await;

            let mut kobzol = spawner.client().await;
            kobzol.join("Kobzol").await;

            sleep(1500).await;
            kobzol.dm("Niko", "Hi there!").await;
            niko.recv().await;
            sleep(1500).await;
            kobzol.dm("Niko", "So, what you're up to?").await;
            niko.recv().await;
            sleep(1500).await;
            kobzol.dm("Niko", "See you at RustWeek?").await;
            niko.recv().await;
            sleep(1500).await;
            kobzol
                .send(ClientToServerMsg::Broadcast {
                    message: "Rust is really cool, you know".to_string(),
                })
                .await;
            niko.recv().await;
            sleep(2000).await;
            kobzol
                .send(ClientToServerMsg::Broadcast {
                    message: "...anyone here?".to_string(),
                })
                .await;
            niko.recv().await;
            sleep(2000).await;

            niko.ping().await;

            Ok(())
        })
        .await;
    }

    // TODO(bonus): The server should correctly close client sockets when it shuts down,
    // to avoid a situation where the clients would be stuck waiting for a message
    // for some indeterminate amount of time.
    /*
    #[tokio::test]
    async fn drop_clients_on_shutdown() {
        let (mut client, mut client2) = run_test(opts(10), |spawner| async move {
            let mut client = spawner.client().await;
            client.join("Bar").await;
            let mut client2 = spawner.client().await;
            client2.join("Foo").await;
            Ok((client, client2))
        })
        .await;

        assert!(client.reader.recv().await.is_none());
        assert!(client2.reader.recv().await.is_none());
    }
    */
    async fn run_test<C, F, R>(opts: ServerOpts, func: C) -> R
    where
        C: FnOnce(ClientSpawner) -> F,
        F: Future<Output = anyhow::Result<R>>,
    {
        let localset = LocalSet::new();
        let (port, ret) = localset
            .run_until(async {
                // Start the server
                let server = run_server(opts).await.expect("creating server failed");
                let port = server.port;

                let spawner = ClientSpawner { port };

                // Spawn the server future
                let server_fut = tokio::task::spawn_local(server.future);

                // Run the test
                let ret = func(spawner).await.expect("test failed");

                // Tell the server to shut down
                server.tx.send(()).unwrap();

                // Wait until it shuts down
                server_fut.await.unwrap().unwrap();

                (port, ret)
            })
            .await;

        TcpStream::connect(("127.0.0.1", port))
            .await
            .expect_err("server is still alive");
        ret
    }

    struct Client {
        writer: MessageWriter<ClientToServerMsg, OwnedWriteHalf>,
        reader: MessageReader<ServerToClientMsg, OwnedReadHalf>,
    }

    impl Client {
        async fn join(&mut self, name: &str) {
            self.send(ClientToServerMsg::Join {
                name: name.to_string(),
            })
            .await;
            let msg = self.recv().await;
            assert!(matches!(msg, ServerToClientMsg::Welcome));
        }

        async fn ping(&mut self) {
            self.send(ClientToServerMsg::Ping).await;
            let msg = self.recv().await;
            assert!(matches!(msg, ServerToClientMsg::Pong));
        }

        async fn list_users(&mut self) -> Vec<String> {
            self.send(ClientToServerMsg::ListUsers).await;
            let msg = self.recv().await;
            match msg {
                ServerToClientMsg::UserList { mut users } => {
                    users.sort();
                    users
                }
                msg => {
                    panic!("Unexpected response {msg:?}");
                }
            }
        }

        async fn dm(&mut self, to: &str, message: &str) {
            self.send(ClientToServerMsg::SendDM {
                to: to.to_string(),
                message: message.to_string(),
            })
            .await;
        }

        async fn expect_message(&mut self, expected_from: &str, expected_message: &str) {
            let msg = self.recv().await;
            match msg {
                ServerToClientMsg::Message { from, message } => {
                    assert_eq!(from, expected_from);
                    assert_eq!(message, expected_message);
                }
                msg => panic!("Unexpected message {msg:?}"),
            }
        }

        async fn send(&mut self, msg: ClientToServerMsg) {
            self.writer.send(msg).await.expect("cannot send message");
        }

        async fn try_send(&mut self, msg: ClientToServerMsg) -> anyhow::Result<()> {
            self.writer.send(msg).await
        }

        async fn expect_error(&mut self, expected_error: &str) {
            let msg = self.recv().await;
            match msg {
                ServerToClientMsg::Error(error) => {
                    assert_eq!(error, expected_error);
                }
                msg => {
                    panic!("Unexpected response {msg:?}");
                }
            }
        }

        async fn recv(&mut self) -> ServerToClientMsg {
            self.reader
                .recv()
                .await
                .expect("connection was closed")
                .expect("did not receive welcome message")
        }

        async fn close(self) {
            self.writer.into_inner().shutdown().await.unwrap();
        }

        async fn check_closed(mut self) {
            assert!(matches!(self.reader.recv().await, None | Some(Err(_))));
        }
    }

    #[derive(Copy, Clone)]
    struct ClientSpawner {
        port: u16,
    }

    impl ClientSpawner {
        async fn client(&self) -> Client {
            let client = TcpStream::connect(("127.0.0.1", self.port))
                .await
                .expect("cannot connect to server");

            let (rx, tx) = client.into_split();

            let reader = MessageReader::<ServerToClientMsg, _>::new(rx);
            let writer = MessageWriter::<ClientToServerMsg, _>::new(tx);
            Client { reader, writer }
        }
    }

    async fn sleep(duration_ms: u64) {
        tokio::time::sleep(Duration::from_millis(duration_ms)).await;
    }

    fn opts(max_clients: usize) -> ServerOpts {
        ServerOpts { max_clients }
    }
}
