use async_std::fs::File;
use async_std::io::{self, prelude::*, SeekFrom};
use async_std::net::TcpListener;
use async_std::task;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Address for the server to listen on
    let addr = "0.0.0.0:8080";

    // Start the server
    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on {}", addr);

    // Accept incoming connections
    while let Ok((stream, _)) = listener.accept().await {
        // Spawn a new task to handle each connection
        task::spawn(handle_connection(stream));
    }

    Ok(())
}

async fn handle_connection(mut stream: async_std::net::TcpStream) {
    let mut buffer = [0; 1024];
    let mut file = File::create("E:/projects/testing/storage/destination.zip").await.unwrap();

    // Read from the stream in chunks and write to the file
    while let Ok(n) = stream.read(&mut buffer).await {
        if n == 0 {
            break; // End of stream
        }

        // Write the received chunk to the file
        file.write_all(&buffer[0..n]).await.unwrap();
    }

    // Seek to the beginning of the file for further processing if needed
    file.seek(SeekFrom::Start(0)).await.unwrap();
    println!("File successfully received and saved.");

    // Perform additional processing or respond to the client as needed
    // ...
}
