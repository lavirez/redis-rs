// Uncomment this block to pass the first stage
use std::{net::{TcpListener, TcpStream}, io::{Read, Write}};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                handle_stream(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_stream(mut stream: TcpStream) {
    let mut buf = [0; 512];
    match stream.read(&mut buf) {
        Ok(_) => {
            let recieved = String::from_utf8_lossy(&buf);
            if recieved == "PING" {
                let response = "+PONG\r\n";
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        }
        Err(e) => {
            println!("An error occured {}", e);
        }
    }
}
