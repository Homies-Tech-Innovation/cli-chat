use std::net::{TcpStream, Shutdown};
use std::io::{stdin, stdout, Write, Read};
use std::{thread, time};

fn connect_server() -> () {

    loop {
        let connection = TcpStream::connect("127.0.0.1:5000");
        match connection {
            Ok(mut stream) => {
                println!("Connected to the server");
                let input: String = handle_user_input();
                let bytes = input.as_bytes();
                stream.write_all(&bytes).expect("Error in writing to the stream");
                stream.shutdown(Shutdown::Write).expect("Failed in shutting down writing part");
                let mut message: String = String::new();
                stream.read_to_string(&mut message).expect("Error in reading broadcast message");
                println!("{}",message);

            },
            
            _ => {}
        }
    }
    ()
    
}

fn handle_user_input() -> String {
    let mut s = String::new();
    print!("Enter text: ");
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a valid string");
    s
}


fn main() {
    connect_server();
}
