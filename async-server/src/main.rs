use std::{
    fs,
    time::Duration,
};
use async_std::{task, prelude::*};
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use futures::stream::StreamExt;


#[async_std::main]
async fn main() {
    // Listen for incoming TCP connections on localhost port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    listener.incoming().for_each_concurrent(/*limit*/ None, |tcpstream| async move {
        let tcpstream = tcpstream.unwrap();
        handle_connection(tcpstream).await;
    }).await;
}

async fn handle_connection(mut stream: TcpStream) {
    // Read the first 1024 bytes of data from the stream
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // Respond with greetings or a 404,
    // depending on the data in the request
    
    // println!("--- 100 ---");
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "./src/hello.html")
    } else if buffer.starts_with(sleep) {
        // println!("--- 150 ---");
        task::sleep(Duration::from_secs(10)).await;
        // println!("--- 175 ---");
        ("HTTP/1.1 200 OK\r\n\r\n", "./src/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "./src/404.html")
    };
    // println!("--- 200 ---");

    let contents = fs::read_to_string(filename).unwrap();

    // Write response back to the stream,
    // and flush the stream to ensure the response is sent back to the client
    let response = format!("{status_line}{contents}");
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
