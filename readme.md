## Loquitor is a async chat server written in rust so as to refresh my understandingon the tokio library and async functions in general.

Certain notes:

1. A future that does not have a known value just yet, maybe later on.
2. #[Tokio::main] -> doesnt know how to run a future, so running this macro turns the async function into normal function with main .
3. .await allows us to gain access to future value of listener.
4. write_all doesnt send to every connection it just sends everything that was in the buffer.
5. Socket faces an issue as it is out of scope that can be handled by split method on the 
bytes_read can be used to check connection.

6. We use tokio if we are going to use multiple clients independently
tokio::spawn is used to create new tasks, which means you can have multiple users.

7. When do we use tokio::select! VS spawn -> use select when we have something operating on same state (i.e: finite) you can also share references, spawn can be unlimited

8. When do we use tokio::select! VS spawn -> use select when we have something operating on same 
spawn can be unlimited.Broadcast channel allows sender and reciver on all clients.

9. Broadcast channel allows sender and reciver on all clients.You cant just do rx.clone as each thing needs a new reciver.

10. You cant just do rx.clone as each thing needs a new reciver.



