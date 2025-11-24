use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;
use std::sync::Arc;
use std::sync::mpsc;

fn create_server() -> () {
    let  listener = TcpListener::bind("127.0.0.1:5000").expect("Failure to bind to port: ");
    let mut senders: Vec<mpsc::Sender::<String>> = Vec::new();

    for stream in listener.incoming() {
        let (tx1, rx1) = mpsc::channel(); //ser to cli
        let (tx2, rx2) = mpsc::channel(); //cli to ser
        senders.push(tx1);
        let stream = stream.expect("Error in unwrapping the result");
        thread::spawn(move || {
            handle_client(stream, rx1, tx2);
        });
        let msg = rx2.recv().unwrap();
        broadcast_message(msg, &senders);
    }
    ()
}

fn handle_client(mut socket: TcpStream, rx: mpsc::Receiver::<String>, tx: mpsc::Sender::<String>) -> () {
    println!("Connection done");
    let mut s: String = String::new();
    socket.read_to_string(&mut s).expect("Error in reading to string");
    tx.send(s).unwrap();
    () } fn broadcast_message(message: String, _senders: &Vec<mpsc::Sender<String>>) -> () { println!("{}",message);
    ()
}


fn main() {
    create_server();
} 
