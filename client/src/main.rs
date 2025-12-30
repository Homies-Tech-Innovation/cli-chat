use std::io::{Write, stdin, stdout};
use tokio_util::sync::CancellationToken;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpStream};
use tokio::runtime::Runtime;

async fn connect_server() -> () {
    let sd = CancellationToken::new();
    let rs = sd.child_token();
    let ws = sd.child_token();

        let connection = TcpStream::connect("127.0.0.1:5000").await;
        match connection {
            Ok(stream) => {
                let (reader, writer) = stream.into_split();
                println!("Connected to the server");
                let read_handle = tokio::spawn(async move {
                    read_messages(reader, rs).await;
                });
                //Input taking task
                let _write_handle = tokio::spawn(async move {
                    write_message(writer, ws).await;
                });
                read_handle.await.unwrap();
            },
            _ => {
                println!("Errors");
            }
        }
    ()
}

async fn read_messages(mut reader: OwnedReadHalf, rs:CancellationToken) {
    let mut buffer = [0u8; 1024];
    loop {
        tokio::select! {
            _ = rs.cancelled() => {
                println!("Reader shut down");
                break;
            }

            res = reader.read(&mut buffer) => match res {
                Ok(n) => {
                    let message = String::from_utf8_lossy(&buffer[..n]);
                    println!("The message is: {}", message);
                },
                Err(_) => break,
            }
        }
    }
    drop(reader);

}


async fn write_message (mut stream: OwnedWriteHalf, _ws:CancellationToken) -> () {
    let string: String = read_user_input();
    let input = string.as_bytes();
    stream.write_all(&input).await.expect("Error in sending message");
    drop(stream);
}

fn read_user_input() -> String {
    let mut buf: String = String::new();
    print!("Enter message: ");
    let _ = stdout().flush();
    stdin().read_line(&mut buf).expect("Error in reading input");
    buf
}



fn main() {
    let rt: Runtime = Runtime::new().unwrap();
    rt.block_on(async {
        connect_server().await;
    });
}
