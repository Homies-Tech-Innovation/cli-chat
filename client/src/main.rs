use std::net::{TcpStream};
use std::io::{BufReader, Read, Write, stdin, stdout};
use std::thread;
use std::time::Duration;

fn connect_server() -> () {

    loop {
        match TcpStream::connect("127.0.0.1:5000"){
            Ok(stream) =>{
                println!("Connected to the server");

                let read_stream = stream.try_clone().expect("Failed to clone stream");

                thread::spawn(move ||{
                    let mut reader = BufReader::new(&read_stream);
                    let mut line = String::new();

                    loop{
                        line.clear();
                        match reader.read_to_string(&mut line){
                            Ok(0) => {
                                println!("\nServer closed connection.");
                                break;
                            }
                            Ok(_) =>{
                               print!("\r{}", line);
                             //  print!("Enter text: ");
                               let _ = stdout().flush();
                            }
                            Err(e) =>{
                                eprintln!("\nError reading from server: {}", e);
                                break;
                            }
                        }
                    }
                });
                handle_user_input(stream);
                break;
            }
            Err(e) =>{
                eprintln!("Failed to connect to server: {}. Retrying in 5 seconds...", e);
                thread::sleep(Duration::from_secs(5));
            }
        }
    
    }
}

fn handle_user_input(mut stream: TcpStream){
    let stdin = stdin();

    loop{
        print!("Enter text: ");
        let _ = stdout().flush();

        let mut input = String::new();
        match stdin.read_line(&mut input){
            Ok(_) =>{
                if input.trim().eq_ignore_ascii_case("quit"){
                    println!("Disconnecting....");
                    break;
                }

                if let Err(e) = stream.write_all(input.as_bytes()){
                    eprintln!("Failed to send message: {}", e);
                    break;
                }
            }
            Err(e) =>{
                eprintln!("Error reading input: {}", e);
                break;
        }
    }
}
}


fn main() {
    connect_server();
}
