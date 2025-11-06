use std::net::{TcpStream, TcpListener};

fn create_server() -> () {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:5000").expect("Problems in binding with port");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_stream(&stream);
            },
            Err(e) => {
                println!("{}",e);
            }
        }
    }
    ()
}

fn handle_stream(_stream: &TcpStream) -> () {
    println!("Stream connected");
    ()
}

fn main() {

    create_server();
}
