use std::io::BufRead;
use std::io::Write;
use std::io::{self};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

fn main() {
    println!("Would you like to connect to a chat?");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    println!("{}", format!("Buffer: {buffer}"));

    if buffer.starts_with("y") {
        buffer.clear();
        println!("Which port would you like to connect to?");
        io::stdin().read_line(&mut buffer).unwrap();

        println!("Port value: {}", buffer);
        println!("{}", format!("127.0.0.1:{buffer}"));

        let stream = TcpStream::connect(format!("127.0.0.1:{buffer}").trim()).unwrap();
        handle_connection(stream);
    } else {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    handle_connection(stream);
                }
                Err(e) => {
                    println!("Connection failed. Error: {}", e);
                }
            }
        }
    }
}

fn handle_receiving_thread(mut stream: TcpStream, rx: Receiver<String>) {
    let mut buf = vec![];
    let mut reader = io::BufReader::new(stream.try_clone().expect("Clone failed"));
    loop {
	let mut length = 0;
        match reader.fill_buf() {
            Ok(a) => {
                println!(
                    "{}",
                    String::from_utf8(buf.to_owned()).expect("Failed to parse to String")
                );
		length = a.len();
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // println!("Waiting...");
            }
            Err(e) => panic!("encountered IO error: {e}"),
        }
			reader.consume(length);
        match rx.try_recv() {
            Ok(val) => {
                println!("Received!");
                stream
                    .set_nonblocking(false)
                    .expect("set_nonblocking call failed");
                let test = stream.write(val.as_bytes()).expect("Write failed");
		stream.flush().expect("Flush failed: sent");
		println!("{} bytes sent", test);
                stream
                    .set_nonblocking(false)
                    .expect("set_nonblocking call failed");
                println!("Sent!");
            }
            Err(_) => {}
        }
    }
}

fn handle_sending_thread(tx: Sender<String>) {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    tx.send(buffer).unwrap();
}

fn handle_connection(stream: TcpStream) {
    stream
        .set_nonblocking(true)
        .expect("set_nonblocking call failed");

    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    let receiving = thread::spawn(move || {
        handle_receiving_thread(stream, rx);
    });

    let sending = thread::spawn(move || {
        handle_sending_thread(tx);
    });

    receiving.join().expect("The receiver thread has panicked");
    sending.join().expect("The sending thread has panicked");
}
