use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::thread;
use std::io::Read;

fn create_server() -> () {
    let listener = TcpListener::bind("127.0.0.1:5000").expect("Failure to bind to port: ");
    for stream in listener.incoming() {
        let stream = stream.expect("Error in unwrapping the result");
        thread::spawn(|| {
        handle_client(stream);
        });
    }
    ()
}

fn handle_client(mut socket: TcpStream) -> () {
    println!("Connection done");
    let mut s: String = String::new();

    let _ = socket.read_to_string(&mut s).expect("Error in reading to string");
    println!("Received message");
    broadcast_message(s, &mut socket);

    ()
}

fn broadcast_message(message: String, mut socket: &TcpStream) -> (){
    socket.write_all(&message.as_bytes()).expect("Error in broadcasting message");
    ()
}


fn main() {
    create_server();
} 
