use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    thread,
    time::Duration,
};

fn main() {
    let server_addr = "127.0.0.1:8888";
    let mut socket = TcpStream::connect(server_addr).expect("Could not bind to address");
    println!("Server listening on {}", server_addr);

    socket
        .set_nonblocking(true)
        .expect("Could not set non-blocking");

    println!("server connected with {}", server_addr);
    start_thread(socket.try_clone().unwrap());

    let user = input("Enter your name: ");
    loop {
        let msg = input("Enter message: ");
        let msg = format!("{} > {}\n", user, msg);
        socket.write_all(msg.as_bytes()).unwrap();
    }
}

// start thread and recieve message from server
fn start_thread(socket: TcpStream) {
    let mut reader = BufReader::new(socket);
    thread::spawn(move || loop {
        // read message from server
        let mut buf = String::new();
        if let Ok(n) = reader.read_line(&mut buf) {
            if n > 0 {
                println!("Server: {}", buf.trim());
            }
        }
        thread::sleep(Duration::from_millis(100));
    });
}

// input
fn input(msg: &str) -> String {
    if msg != "" {
        println!("{}", msg);
    }
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    String::from(buf.trim())
}
