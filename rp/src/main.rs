// use std::io::prelude::*;
// use std::net::TcpListener;
// use std::net::TcpStream;
// use rp::ThreadPool;
use std::env;

// Kharon
// fn main() {
//     connect_client();
// }
// /*
// Connect the reverse proxy server to the client
// */
// fn connect_client() {
//     let listener: TcpListener = TcpListener::bind("0.0.0.0:7878").unwrap();
//     let pool = ThreadPool::new(12).unwrap();
    
//     for stream in listener.incoming() {
//         match stream {
//             Ok(stream) => {
//                 pool.execute(|| {
//                     handle_client_connection(stream); 
//                 });
//             }
//             Err(e) => { 
//                 println!("Err: {}", e.to_string()); 
//             }
//         }
//     }
// }

// fn handle_client_connection(mut stream: TcpStream){

//     let mut buffer = [0; 1024]; //TODO: look into buffer size
//     stream.read(&mut buffer).unwrap();

//     // Choose which server to talk to.
//     let server_host: String = env::var("EXP_HOST").unwrap();
//     let server_port: String = env::var("EXP_PORT").unwrap();
//     let address = format!("{}:{}", server_host, server_port);
//     let mut out_stream = TcpStream::connect(address).unwrap();
        
//     // Send bytes to backend~
//     out_stream.write(&buffer[..]).unwrap();
//     out_stream.flush().unwrap();
//     out_stream.read(&mut buffer).unwrap();
            
//     // Send backend response to client
//     stream.write(&buffer[..]).unwrap();
//     stream.flush().unwrap();
    
// }

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let listener = TcpListener::bind("0.0.0.0:7878").await?;
    
    loop {
        let (mut socket, _) = match listener.accept().await {
            Ok((socket, addr)) => (socket, addr),
            Err(e) => {
                eprintln!("failed to connect to addres; err = {:?}", e);
                panic!("panic")
            }
        };
        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket, write data to destination, write response to socket
            loop {
                let _: usize = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => {
                        
                        let server_host: String = env::var("EXP_HOST").unwrap();
                        let server_port: String = env::var("EXP_PORT").unwrap();
                        let address = format!("{}:{}", server_host, server_port);
                        let mut server: TcpStream = match TcpStream::connect(address).await {
                            Ok(stream) => stream,
                            Err(e) => {
                                eprintln!("failed to connect to addres; err = {:?}", e);
                                return;
                            }
                        };
                        // Send bytes to server
                        server.write(&buf[0..n]).await.unwrap();
                        server.flush().await.unwrap();
                        server.read(&mut buf).await.unwrap();
                        // Send backend response to socket
                        socket.write(&buf[0..n]).await.unwrap();
                        socket.flush().await.unwrap();
                        n
                    }
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                // if let Err(e) = socket.write_all(&buf[0..n]).await {
                //     eprintln!("failed to write to socket; err = {:?}", e);
                //     return;
                // }
            }
        });
    }
}