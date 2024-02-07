use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::process;

mod config;
use config::AppConfig;

fn main() {
    // Load configuration
    let config = AppConfig::new("127.0.0.1:8080", vec!["127.0.0.1:8000", "127.0.0.1:8001"]);

    // Your code will go here
    println!("Hello, Load Balancer!");

    // Example: create a TCP listener using the bind address from the configuration
    let listener = match TcpListener::bind(&config.bind_address) {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("Error binding to address: {}", err);
            process::exit(1);
        }
    };
    
    // Example: handle incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Handle the connection (you'll replace this with your logic)
                handle_connection(stream);
            }
            Err(err) => {
                eprintln!("Error accepting connection: {}", err);
            }
        }
    }
}

// Example: handle individual connections
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    
    // Read data from the client
    match stream.read(&mut buffer) {
        Ok(_) => {
            // Process the request (you'll replace this with your logic)
            let response = process_request(&buffer);
            
            // Send the response back to the client
            match stream.write_all(response.as_bytes()) {
                Ok(_) => (),
                Err(err) => eprintln!("Error sending response: {}", err),
            }
        }
        Err(err) => eprintln!("Error reading from client: {}", err),
    }
}

// Example: process the client request
fn process_request(request: &[u8]) -> String {
    // Replace this with your request processing logic
    String::from("HTTP/1.1 200 OK\r\n\r\nHello, client!")
}
