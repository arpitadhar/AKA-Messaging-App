use http_server::ThreadPool;
use std::{
    str,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},

};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(12);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            println!("Connection established :P!");
            handle_connection(stream);
        });
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    println!("{:?}", request_line);
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        line if line.starts_with("GET /?POST=")  => ("HTTP/1.1 200 OK", "hello.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    if status_line == "HTTP/1.1 200 OK" {
        println!("{:?}", request_line);
        if request_line.starts_with("GET /?POST=") {
            let mut buffer = [0, 255];
            stream.read(&mut buffer).unwrap();
            let query_string = str::from_utf8(&buffer).unwrap();
            let post_message = query_string.splitn(2, '=').nth(1).unwrap_or("");
            println!("{:?}", request_line);
            println!("POST message: {}", post_message);
        }
    }
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
