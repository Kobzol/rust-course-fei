//! TODO: implement a simple chat server using non-blocking I/O
//!
//! The chat server should behave identically as the one from last week, with one new feature.
//! However, it should be implemented using non-blocking I/O and run on a single thread.
//! It should still support concurrency and allow the connection of multiple clients at once.
//!
//! Ideally, reuse your implementation from last week, but change blocking I/O to non-blocking I/O.
//! Your code should not "spin", i.e. use 100% of CPU all the time. It should also not sleep for
//! arbitrary duration of time, it should always sleep until the next I/O event or the next timeout
//! event. Use `epoll` (or a similar mechanism on your OS) to achieve this. You can use `epoll`
//! on Linux or the `mio` crate on macOS/Windows (or anything else that you want).
//!
//! **Do not use blocking I/O. Do not use `async/await` or any external crates that deal with
//! networking, except for `epoll` and similar crates, for this assignment.**
//!
//! Note: it is enough to deal with non-blocking reads, you can simply assume that writes will be
//! non-blocking. As a **bonus**, you can try to sketch a solution for also dealing with
//! non-blocking writes.
//!
//! TODO(question): try to examine the message protocol from the last week. What issue would there be
//! if we used non-blocking I/O for reading with the previous implementation? Could we just use the
//! previous `MessageReader` as it was? Try to replace the current reader with it and describe what
//! is the issue.

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

/// TODO: implement the following function called `run_server`
/// It should start a chat server on a TCP/IP port assigned to it by the operating system and
/// return a structure called `RunningServer`. This struct should have a method called `port`,
/// which returns the port on which the server is running.
///
/// You should create a single thread that will run the whole server, but you should not create
/// any additional threads for handling clients. The thread is only used to let the tests know
/// on which port is the server running.
///
/// The server should implement the messages described in `messages.rs`, see the message comments
/// for more details. The details are the same as last week, with one exception described below.
///
/// # Client connection
/// When a client connects to the server, it should send a `Join` message.
/// - **(NEW)** If the client does not send a `Join` message within two seconds, the server should
/// send an error "Timed out waiting for Join" and disconnect the client immediately.
/// If it sends anything else, the server should respond with an error "Unexpected message received"
/// and disconnect the client immediately.
/// If the user sends a Join message (with a unique username), the server should respond with
/// the `Welcome` message.
/// Then it should start receiving requests from the client.
/// If the client ever sends the `Join` message again, the server should respond with an error
/// "Unexpected message received" and disconnect the client immediately.
///
/// # Maximum number of clients
/// When a client connects and there are already `opts.max_clients` other clients connected, the
/// server should respond with an error "Server is full" and disconnect the client immediately.
/// Note that if the server is full, the client should be disconnected even before it sends the
/// `Join` message.
///
/// # Graceful shutdown
/// When `RunningServer` is dropped, it should:
/// 1) Stop receiving new TCP/IP connections
/// 2) Correctly disconnect all connected users
/// 3) Wait until all threads that it has created has completed executing
///
/// Graceful shutdown, even with non-blocking I/O and `epoll`, can be challenging.
/// Think about how you can get around this - can you find some way to "wake" the `epoll`?
/// You can use the same mechanism as last week, or try something else.
///
/// See tests for more details.
fn run_server(opts: ServerOpts) -> anyhow::Result<RunningServer> { todo!() }


#[cfg(test)]
mod tests {
    use crate::messages::{ClientToServerMsg, ServerToClientMsg};
    use crate::reader::MessageReader;
    use crate::writer::MessageWriter;
    use crate::{run_server, RunningServer, ServerOpts};
    use std::io::{Read, Write};
    use std::net::{Shutdown, TcpStream};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Barrier, Mutex};
    use std::thread::spawn;
    use std::time::Duration;

    // If you're struggling with this test, comment it and implement the rest of the
    // functionality first.
    #[test]
    fn empty_server_shuts_down() {
        run_test(opts(2), |_| Ok(()));
    }

    #[test]
    fn max_clients() {
        run_test(opts(2), |server| {
            let _client = server.client();
            let _client2 = server.client();

            let mut client3 = server.client();
            client3.expect_error("Server is full");
            client3.check_closed();

            Ok(())
        });
    }

    #[test]
    fn max_clients_after_client_leaves() {
        run_test(opts(2), |server| {
            let _client = server.client();
            let client2 = server.client();
            client2.close();

            sleep(1000);

            let mut client3 = server.client();
            client3.join("Foo");

            Ok(())
        });
    }

    #[test]
    fn max_clients_herd() {
        let max_clients = 5;
        run_test(opts(max_clients), |server| {
            let thread_count = 50;

            let server = Arc::new(server);
            let barrier = Arc::new(Barrier::new(thread_count));

            let errors = Arc::new(AtomicUsize::new(0));
            let successes = Arc::new(AtomicUsize::new(0));

            let joined_clients = Arc::new(Mutex::new(vec![]));
            std::thread::scope(|s| {
                for thread_id in 0..thread_count {
                    let barrier = barrier.clone();
                    let server = server.clone();
                    let errors = errors.clone();
                    let successes = successes.clone();
                    let joined_clients = joined_clients.clone();
                    s.spawn(move || {
                        barrier.wait();
                        let mut client = server.client();
                        let _ = client.try_send(ClientToServerMsg::Join {
                            name: format!("Thread {thread_id}"),
                        });
                        match client.recv() {
                            ServerToClientMsg::Error(_) => {
                                errors.fetch_add(1, Ordering::SeqCst);
                            }
                            ServerToClientMsg::Welcome => {
                                successes.fetch_add(1, Ordering::SeqCst);
                                // Make sure that the client doesn't disconnect
                                joined_clients.lock().unwrap().push(client);
                            }
                            msg => {
                                panic!("Unexpected message {msg:?}");
                            }
                        }
                    });
                }
            });
            assert_eq!(errors.load(Ordering::SeqCst), thread_count - max_clients);
            assert_eq!(successes.load(Ordering::SeqCst), max_clients);

            drop(joined_clients);

            Ok(())
        });
    }

    #[test]
    fn list_users_before_join() {
        run_test(opts(2), |server| {
            let mut client = server.client();
            client.send(ClientToServerMsg::ListUsers);
            client.expect_error("Unexpected message received");

            Ok(())
        });
    }

    #[test]
    fn join_after_half_sec() {
        run_test(opts(2), |server| {
            let mut client = server.client();
            sleep(500);
            client.join("Foo");
            assert_eq!(client.list_users(), vec!["Foo".to_string()]);

            Ok(())
        });
    }

    #[test]
    fn join_timeout() {
        run_test(opts(2), |server| {
            let mut client = server.client();
            sleep(3000);
            match client.try_send(ClientToServerMsg::Join {
                name: "Bilbo".to_string(),
            }) {
                Ok(_) => {
                    client.expect_error("Timed out waiting for Join");
                }
                Err(_) => {}
            }

            Ok(())
        });
    }

    #[test]
    fn duplicated_join() {
        run_test(opts(2), |server| {
            let mut client = server.client();
            client.join("Foo");
            client.send(ClientToServerMsg::Join {
                name: "Bar".to_string(),
            });
            client.expect_error("Unexpected message received");

            Ok(())
        });
    }

    #[test]
    fn error_then_disconnect() {
        run_test(opts(2), |server| {
            let mut client = server.client();
            client.join("Foo");
            client.send(ClientToServerMsg::Join {
                name: "Bar".to_string(),
            });
            client.close();

            let mut client2 = server.client();
            client2.join("Bar");

            Ok(())
        });
    }

    #[test]
    fn duplicated_username() {
        run_test(opts(2), |server| {
            let mut client = server.client();
            client.join("Foo");

            let mut client2 = server.client();
            client2.send(ClientToServerMsg::Join {
                name: "Foo".to_string(),
            });
            client2.expect_error("Username already taken");

            Ok(())
        });
    }

    #[test]
    fn ping() {
        run_test(opts(2), |server| {
            let mut luca = server.client();
            luca.join("Luca");
            luca.ping();

            Ok(())
        });
    }

    #[test]
    fn ping_before_join() {
        run_test(opts(2), |server| {
            let mut client = server.client();
            client.send(ClientToServerMsg::Ping);
            client.expect_error("Unexpected message received");

            Ok(())
        });
    }

    #[test]
    fn list_users_reconnect() {
        run_test(opts(2), |server| {
            let mut client = server.client();
            client.join("Foo");
            client.close();

            let mut client = server.client();
            client.join("Foo");
            assert_eq!(client.list_users(), vec!["Foo".to_string()]);

            Ok(())
        });
    }

    #[test]
    fn list_users_self() {
        run_test(opts(2), |server| {
            let mut client = server.client();
            client.join("Martin");
            assert_eq!(client.list_users(), vec!["Martin".to_string()]);

            Ok(())
        });
    }

    #[test]
    fn list_users_ignore_not_joined_users() {
        run_test(opts(2), |server| {
            let _client = server.client();
            let mut client2 = server.client();
            client2.join("Joe");
            assert_eq!(client2.list_users(), vec!["Joe".to_string()]);

            Ok(())
        });
    }

    #[test]
    fn list_users_after_error() {
        run_test(opts(2), |server| {
            let mut client = server.client();
            client.join("Terrence");

            let mut client2 = server.client();
            client2.join("Joe");

            client.send(ClientToServerMsg::Join {
                name: "Barbara".to_string(),
            });

            sleep(1000);

            assert_eq!(client2.list_users(), vec!["Joe".to_string()]);

            Ok(())
        });
    }

    #[test]
    fn list_users() {
        run_test(opts(2), |server| {
            let mut client = server.client();
            client.join("Terrence");

            let mut client2 = server.client();
            client2.join("Joe");
            assert_eq!(
                client2.list_users(),
                vec!["Joe".to_string(), "Terrence".to_string()]
            );
            client2.close();

            sleep(1000);

            assert_eq!(client.list_users(), vec!["Terrence".to_string()]);

            Ok(())
        });
    }

    #[test]
    fn dm_nonexistent_user() {
        run_test(opts(2), |server| {
            let mut client = server.client();
            client.join("Mark");
            client.dm("Fiona", "Hi");
            client.expect_error("User Fiona does not exist");

            Ok(())
        });
    }

    #[test]
    fn dm_self() {
        run_test(opts(2), |server| {
            let mut client = server.client();
            client.join("Xal'atath");
            client.dm("Xal'atath", "I'm so lonely :(");
            client.expect_error("Cannot send a DM to yourself");

            Ok(())
        });
    }

    #[test]
    fn dm_other() {
        run_test(opts(2), |server| {
            let mut terrence = server.client();
            terrence.join("Terrence");

            let mut joe = server.client();
            joe.join("Joe");

            terrence.dm("Joe", "How you doin'");
            joe.expect_message("Terrence", "How you doin'");

            Ok(())
        });
    }

    #[test]
    fn dm_spam() {
        run_test(opts(2), |server| {
            let mut diana = server.client();
            diana.join("Diana");

            let mut francesca = server.client();
            francesca.join("Francesca");

            let barrier = Arc::new(Barrier::new(2));
            let barrier2 = barrier.clone();

            let count = 10000;

            // Let's say that someone is spamming you...
            let t1 = spawn(move || {
                barrier.wait();

                for _ in 0..count {
                    diana.dm("Francesca", "Can I borrow your brush? Pleeeeeease :(((");
                }
            });

            // ...so you get angry, and start spamming them back.
            // But you make a critical *error*, because you're sending the message
            // to the wrong account.
            // Can your chat server handle that?
            let t2 = spawn(move || {
                // Sync the threads a little bit
                barrier2.wait();

                for _ in 0..count {
                    francesca.dm("Daina", "NO! Get your own!");
                    match francesca.recv() {
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
                    match francesca.recv() {
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
            });
            t1.join().unwrap();
            t2.join().unwrap();

            Ok(())
        });
    }

    #[test]
    fn dm_spam_2() {
        // Meanwhile, in a parallel universe...
        run_test(opts(2), |server| {
            let mut diana = server.client();
            diana.join("Diana");

            let mut francesca = server.client();
            francesca.join("Francesca");

            let barrier = Arc::new(Barrier::new(2));
            let barrier2 = barrier.clone();

            let count = 10000;

            // Let's say that someone is spamming you...
            let t1 = spawn(move || {
                barrier.wait();

                for _ in 0..count {
                    diana.dm("Francesca", "Can I borrow your brush? Pleeeeeease :(((");
                }
            });

            // ...so you get angry, and start spamming them back.
            // But you make a critical *error*, because you push the wrong button and start
            // sending pings to the server instead.
            // Can your chat server handle that?
            let t2 = spawn(move || {
                // Sync the threads a little bit
                barrier2.wait();

                for _ in 0..count {
                    francesca.send(ClientToServerMsg::Ping);
                    match francesca.recv() {
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
                    match francesca.recv() {
                        ServerToClientMsg::Message { from, message } => {
                            assert_eq!(from, "Diana");
                            assert_eq!(message, "Can I borrow your brush? Pleeeeeease :(((");
                        }
                        ServerToClientMsg::Pong => {}
                        msg => panic!("Unexpected message {msg:?}"),
                    }
                }
            });
            t2.join().unwrap();
            t1.join().unwrap();

            Ok(())
        });
    }

    #[test]
    fn broadcast_empty() {
        run_test(opts(2), |server| {
            let mut ji = server.client();
            ji.join("Ji");
            ji.send(ClientToServerMsg::Broadcast {
                message: "Haaaaaai!".to_string(),
            });
            ji.ping();

            Ok(())
        });
    }

    #[test]
    fn broadcast() {
        run_test(opts(10), |server| {
            let mut niko = server.client();
            niko.join("Niko");

            let users: Vec<Client> = (0..5)
                .map(|i| {
                    let mut client = server.client();
                    client.join(&format!("NPC {i}"));
                    client
                })
                .collect();

            niko.send(ClientToServerMsg::Broadcast {
                message: "Borrow this!".to_string(),
            });
            niko.ping();

            for mut user in users {
                user.expect_message("Niko", "Borrow this!");
            }

            Ok(())
        });
    }

    // The server should correctly close client socket when it shuts down,
    // to avoid a situation where the clients would be stuck waiting for a message
    // for some indeterminate amount of time.
    #[test]
    fn drop_clients_on_shutdown() {
        let server = run_server(opts(2)).expect("creating server failed");

        let mut client = server.client();
        client.join("Bar");
        let mut client2 = server.client();
        client2.join("Foo");

        drop(server);

        assert!(client.reader.recv().is_none());
        assert!(client2.reader.recv().is_none());
    }

    fn run_test<F: FnOnce(RunningServer) -> anyhow::Result<()>>(opts: ServerOpts, func: F) {
        let server = run_server(opts).expect("creating server failed");
        let port = server.port;
        func(server).expect("test failed");

        TcpStream::connect(("127.0.0.1", port)).expect_err("server is still alive");
    }

    struct Client {
        writer: MessageWriter<ClientToServerMsg, SocketWrapper>,
        reader: MessageReader<ServerToClientMsg, SocketWrapper>,
    }

    impl Client {
        #[track_caller]
        fn join(&mut self, name: &str) {
            self.send(ClientToServerMsg::Join {
                name: name.to_string(),
            });
            let msg = self.recv();
            assert!(matches!(msg, ServerToClientMsg::Welcome));
        }

        #[track_caller]
        fn ping(&mut self) {
            self.send(ClientToServerMsg::Ping);
            let msg = self.recv();
            assert!(matches!(msg, ServerToClientMsg::Pong));
        }

        #[track_caller]
        fn list_users(&mut self) -> Vec<String> {
            self.send(ClientToServerMsg::ListUsers);
            let msg = self.recv();
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

        #[track_caller]
        fn dm(&mut self, to: &str, message: &str) {
            self.send(ClientToServerMsg::SendDM {
                to: to.to_string(),
                message: message.to_string(),
            });
        }

        #[track_caller]
        fn expect_message(&mut self, expected_from: &str, expected_message: &str) {
            let msg = self.recv();
            match msg {
                ServerToClientMsg::Message { from, message } => {
                    assert_eq!(from, expected_from);
                    assert_eq!(message, expected_message);
                }
                msg => panic!("Unexpected message {msg:?}"),
            }
        }

        #[track_caller]
        fn send(&mut self, msg: ClientToServerMsg) {
            self.writer.send(msg).expect("cannot send message");
        }

        #[track_caller]
        fn try_send(&mut self, msg: ClientToServerMsg) -> anyhow::Result<()> {
            self.writer.send(msg)
        }

        #[track_caller]
        fn expect_error(&mut self, expected_error: &str) {
            let msg = self.recv();
            match msg {
                ServerToClientMsg::Error(error) => {
                    assert_eq!(error, expected_error);
                }
                msg => {
                    panic!("Unexpected response {msg:?}");
                }
            }
        }

        fn recv(&mut self) -> ServerToClientMsg {
            self.reader
                .recv()
                .expect("connection was closed")
                .expect("did not receive welcome message")
        }

        #[track_caller]
        fn close(self) {
            self.writer.inner().0.shutdown(Shutdown::Both).unwrap();
        }

        #[track_caller]
        fn check_closed(mut self) {
            assert!(matches!(self.reader.recv(), None | Some(Err(_))));
        }
    }

    #[derive(Clone)]
    struct SocketWrapper(Arc<TcpStream>);

    impl Read for SocketWrapper {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            self.0.as_ref().read(buf)
        }
    }

    impl Write for SocketWrapper {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.as_ref().write(buf)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.0.as_ref().flush()
        }
    }

    impl RunningServer {
        fn client(&self) -> Client {
            let client =
                TcpStream::connect(("127.0.0.1", self.port())).expect("cannot connect to server");
            let client = SocketWrapper(Arc::new(client));

            let writer = MessageWriter::<ClientToServerMsg, SocketWrapper>::new(client.clone());
            let reader = MessageReader::<ServerToClientMsg, SocketWrapper>::new(client);
            Client { reader, writer }
        }
    }

    fn sleep(duration_ms: u64) {
        std::thread::sleep(Duration::from_millis(duration_ms));
    }

    fn opts(max_clients: usize) -> ServerOpts {
        ServerOpts { max_clients }
    }
}
