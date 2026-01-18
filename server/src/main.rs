use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufReader, BufRead};
use std::sync::{Arc, Mutex};
use std::thread;

type ClientList = Arc<Mutex<Vec<TcpStream>>>;


fn create_server() -> () {
    let listener = TcpListener::bind("127.0.0.1:5000").expect("Failed to bind to port: ");
    
    let clients: ClientList = Arc::new(Mutex::new(Vec::new()));

    println!("Server listening on port 5000\n");

    for stream in listener.incoming() {
        match stream{
            Ok(stream) => {
                let clients = Arc::clone(&clients);
                thread::spawn(move || {
                    handle_client(stream, clients);
                });
            }
            Err(e) =>{
                eprint!("Error Accepting connection: {}", e);
            }
        }
    }
    ()
}

fn handle_client(socket: TcpStream, clients: ClientList) -> () {
    let addr = socket.peer_addr().ok();
    println!("New connection: {:?}", addr);

    //We clone the stream for broacasting
    let socket_clone = socket.try_clone().expect("Failed to colne socket");

    //Then we add client to the client list
    {
        let mut client_list = clients.lock().unwrap();
        client_list.push(socket_clone);
        println!("Total clients: {}\n", client_list.len());
    }
    let mut reader = BufReader::new(&socket);
    let mut line = String::new();

    loop{
        line.clear();
        match reader.read_line(&mut line){
            Ok(0) => {
                println!("Client disconnected: {:?}", addr);
                break;
            }
            Ok(_) =>{
                let message = line.trim();
                if !message.is_empty(){
                    println!("Received message from {:?}: {}", addr, message);
                    broadcast_message(message.to_string(), &clients, &socket);
                }
            }
            Err(e) =>{
                eprintln!("Error reading from client {:?}: {}", addr, e);
                break;
            }
        }
    }
    
}

fn broadcast_message(message: String, clients: &ClientList,sender: &TcpStream) -> (){
    let sender_addr = sender.peer_addr().ok();
    let broadcast = format!("Broadcast: {}\n", message);

    let mut client_list = clients.lock().unwrap();
    let mut disconnected = Vec::new();

    for (i, client) in client_list.iter_mut().enumerate()
{
    if let Ok(addr)= client.peer_addr()
{
    if Some(addr) != sender_addr{
        if let Err(e) = client.write_all(broadcast.as_bytes()){
            eprintln!("Error sending message to {:?}: {}", addr, e);
            disconnected.push(i);
        }
    }
}}    for &i in disconnected.iter().rev(){
    client_list.remove(i);
}
println!("Active clients: {}", client_list.len());
}


fn main() {
    create_server();
} 
