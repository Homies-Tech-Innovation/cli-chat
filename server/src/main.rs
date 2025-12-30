use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{self, Receiver, Sender};

enum Message {
    Text(String),
    Channel(Sender<String>),
}
pub struct Broadcaster{
    receiver: Receiver<Message>,
    client_handlers: Vec<Sender<String>>
}
impl Broadcaster {
    fn new(rx1: Receiver<Message>) -> Broadcaster {
        Broadcaster {
            receiver: rx1,
            client_handlers: Vec::new()
        }
    }
    async fn broadcast_message(&mut self) -> () {

        while let Some(s) = self.receiver.recv().await {
            match s {
                Message::Text(s) => {
                    let mut failed= Vec::new();

                    for (i,sender) in self.client_handlers.iter().enumerate() {
                        if sender.send(s.clone()).await.is_err() {
                            failed.push(i);
                        }
                        else {
                            print!("{}",s);
                        }
                    }
                    for i in failed.into_iter().rev() {
                        self.client_handlers.swap_remove(i);
                    }
                },
                Message::Channel(s) => {
                    self.client_handlers.push(s);
                }
            }
        }

        ()
    }
    fn _add_handler(&mut self, tx: Sender<String>) -> () {
        self.client_handlers.push(tx);
    }
}


async fn create_server() {

    let  listener: TcpListener = TcpListener::bind("127.0.0.1:5000").await.expect("Failure to bind to por~t: ");
    
    //For message passing from CLIENT HANDLER TO BROADCASTER
    let (tx1, rx1) = mpsc::channel(100);

    //Creating broadcasting server for later use
    let mut broadcaster: Broadcaster = Broadcaster::new(rx1);

    //Broadcast task
    tokio::spawn(async move {
        broadcaster.broadcast_message().await;
        
    });

    //Client Handler Loop
    loop {
        let (socket, _) = listener.accept().await.expect("Error in connection");
        //BROADCASTER to CLIENT HANDLER
        let (tx2 , rx2) = mpsc::channel(1000);

        let txx1:Sender<Message> = tx1.clone();
        //add client channel to broadcaster
        tx1.send(Message::Channel(tx2)).await.unwrap();

        //Client Handler Task
        tokio::spawn(async move {

            handle_client(socket, txx1, rx2).await;
        });

    }
}

async fn handle_client(socket: TcpStream, tx1: Sender<Message>, mut rx2: Receiver<String>) -> () {
    println!("Connected");

    let (mut reader, mut writer) = socket.into_split();

    //Read task
    tokio::spawn(async move {
        loop {
            //let mut message:Message  = Message::Text(String::new());

            
            let mut message: String = String::new();
            match reader.read_to_string(&mut message).await {
                Ok(_) => {
                    if !message.trim().is_empty() {
                        tx1.send(Message::Text(message.clone())).await.expect("Error in sending message through channel");

                    }
                }
                Err(_) => {
                    drop(reader);
                    drop(tx1);
                    break;
                }
            }
        }
    });

    //Write task
    tokio::spawn (async move {
        loop {
            while let Some(i) = rx2.recv().await {
                match writer.write_all(i.as_bytes()).await{
                    Ok(_) =>{
                        writer.flush().await.ok();
                        println!("Message sent successfully");
                    }
                    Err(_) => {
                        println!("Error in writing to client");
                        break;
                    }
                };
            }
            break;
        }
        drop(writer);
    });
    
    ()
}

fn main() {
    let rt: Runtime = Runtime::new().unwrap();
    rt.block_on(async {
        create_server().await;
    });
    println!("Done");
} 
