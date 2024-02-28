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
    let mut buf_reader = BufReader::new(&mut stream);
    let mut request_line = String::new(); // Define a mutable variable to store the request line
    buf_reader.read_line(&mut request_line).unwrap();
    request_line = request_line.trim().to_string();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        line if line.starts_with("GET /?POST=")  => ("HTTP/1.1 200 OK", "hello.html"),
        line if line.ends_with(".css HTTP/1.1") => {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let uri = parts.get(1).unwrap_or(&"").trim_start_matches('/');
            ("HTTP/1.1 200 OK", uri)
        },
        line if line.ends_with(".js HTTP/1.1") => {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let uri = parts.get(1).unwrap_or(&"").trim_start_matches('/');
            ("HTTP/1.1 200 OK", uri)
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    println!("Status: {}", status_line);
    println!("Request: {}", request_line); 

    if status_line == "HTTP/1.1 200 OK" {
        println!("Request 2: {:?}", request_line);
        if request_line.starts_with("GET /script.js") {
            let mut buffer = [0, 255];
            //stream.read(&mut buffer).unwrap();
            let query_string = str::from_utf8(&buffer).unwrap();
            let post_message = query_string.splitn(2, '=').nth(1).unwrap_or("");
            println!("Request: 3 {:?}", request_line);
            println!("POST message: {}", post_message);
            let post_contents = fs::read_to_string(filename).unwrap();
            let post_length = post_contents.len();
            let response =
                format!("{status_line}\r\nContent-Length: {post_length}\r\n\r\n{post_contents}");

            stream.write_all(response.as_bytes()).unwrap();
        }
    }

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
