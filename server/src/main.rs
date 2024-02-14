use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc,
    thread,
    time::Duration,
};

fn main() {
    let server_addr = "127.0.0.1:8888";
    let (tx, rx) = mpsc::channel::<String>();
    let mut clients: Vec<TcpStream> = Vec::new();

    let server = TcpListener::bind(server_addr).expect("server failed");
    server.set_nonblocking(true).expect("nonblocking failed");
    println!(
        "Server is running on {} and waiting for connections",
        server_addr
    );

    loop {
        // wait for messages from clients
        if let Ok((client, addr)) = server.accept() {
            println!("New client: {}", addr);
            clients.push(client.try_clone().expect("clone failed"));
            start_thread(client, tx.clone());
        }

        // wait for messages from threads
        if let Ok(msg) = rx.try_recv() {
            println!("send all: {}", msg.trim());
            clients = send_all(clients, &msg);
        }
        thread::sleep(Duration::from_millis(100));
    }
}

// start receiving messages for a client
fn start_thread(client: TcpStream, tx: mpsc::Sender<String>) {
    let mut reader = BufReader::new(client);
    thread::spawn(move || loop {
        let mut msg = String::new();
        if let Ok(n) = reader.read_line(&mut msg) {
            if n > 0 {
                tx.send(msg).unwrap();
            }
        }
        thread::sleep(Duration::from_micros(100));
    });
}

// send a message to all clients
fn send_all(clients: Vec<TcpStream>, s: &str) -> Vec<TcpStream> {
    let mut collector: Vec<TcpStream> = Vec::new();
    for mut socket in clients.into_iter() {
        let bytes = String::from(s).into_bytes();
        if let Err(e) = socket.write_all(&bytes) {
            println!("Failed to send message: {}", e);
            continue;
        }
        collector.push(socket);
    }
    collector
}
