use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::env;
use rp::ThreadPool;

// Kharon
fn main() {
    connect_client();
}


/*
Connect the reverse proxy server to the client
*/
fn connect_client() {
    let listener: TcpListener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = ThreadPool::new(12).unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    handle_client_connection(stream); 
                });
            }
            Err(e) => { 
                println!("Err: {}", e.to_string()); 
            }
        }
    }
}

fn handle_client_connection(mut stream: TcpStream){

        let mut buffer = [0; 1024]; //TODO: look into buffer size
        stream.read(&mut buffer).unwrap();

        // Choose which server to talk to.
        let server_host = env::var("EXP_HOST").unwrap();
        let server_port = env::var("EXP_PORT").unwrap();
        let address = format!("{}:{}", server_host, server_port);
        let mut out_stream = TcpStream::connect(address).unwrap();
        
        // Send bytes to backend
        out_stream.write(&buffer[..]).unwrap();
        out_stream.flush().unwrap();
        out_stream.read(&mut buffer).unwrap();
            
        // Send backend response to client
        stream.write(&buffer[..]).unwrap();
        stream.flush().unwrap();
    
}