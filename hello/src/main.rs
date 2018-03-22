use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::thread;
use std::time::Duration;

fn main() {
    let port = 8080;
    let listener = connect(port);


    for request in listener.incoming() {
        let stream = request.unwrap();

        let _res = handle_connection(stream);
    }
}

fn connect(port: u16) -> TcpListener {
    match TcpListener::bind(format!("0.0.0.0:{}", port)) {
        Ok(listener) => {
            println!("Listening to: http://localhost:{}",  port);

            listener
        },
        Err(err) => {
            println!("Unable to use port {} due to {:?}", port, err);
            println!("Retrying next port {}", port + 1);

            connect(port + 1)
        },
    }
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 512];

    stream.read(&mut buf)?;

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (path, status) = if buf.starts_with(get) {
        ("index.html", "200 OK")
    } else if buf.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));

        ("sleep.html", "200 OK")
    } else {
        ("404.html", "404 NOT FOUND")
    };

    // responding
    let mut file = File::open(format!("pages/{}", path))?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let response = format!("HTTP/1.1 {}\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}", status, contents.len(), contents);

    stream.write(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}