use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (tx, _rx) = broadcast::channel(100);

    loop {
        // Allows unlimited number of users to access it
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result=reader.read_line(&mut line) =>{
                        if result.unwrap()==0{
                            break;
                        }
                         tx.send((line.clone(),addr)).unwrap();
                         line.clear();
                    }
                    result=rx.recv() =>
                    {
                        let (msg, other)=result.unwrap();
                        if addr != other{
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}

// A future that does not have a known value just yet, maybe later on
// #[Tokio::main] -> doesnt know how to run a future, so running this macro turns the async function into normal function with main
// .await allows us to gain access to future value of listener
// write_all doesnt send to every connection it just sends everything that was in the buffer
//
//  let mut buffer = [0u8; 1024];

//         let bytes_read = socket.read(&mut buffer).await.unwrap();
// Is replaced by the above function

// socket faces an issue as it is out of scope that can be handled by split method on the socket write and read
// bytes_read can be used to check connection

// We use tokio if we are going to use multiple clients independently

// tokio::spawn is used to create new tasks, which means you can have multiple users

// Broadcast channel allows sender and reciver on all clients

// You cant just do rx.clone as each thing needs a new reciver
// tokio::select! allows you to run asynchronous function concurrently

//  let bytes_read = reader.read_line(&mut line).await.unwrap();
//                 if bytes_read == 0 {
//                     break;
//                 }
// Got replaced

// Do note that we are doing a very simple way to prevent the double write

// A turbofish tells the compiler what generic type we want from the program

// When do we use tokio::select! VS spawn -> use select when we have something operating on same state (i.e: finite) you can also shar, spawn can be unlimited
