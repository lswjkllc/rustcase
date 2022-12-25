use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write};
use std::fs;
use std::thread;
use std::time::Duration;

use simweb::ThreadPool;

fn main() {
    let addr = "127.0.0.1:8001";
    let listener = TcpListener::bind(addr).unwrap();

    let pool = ThreadPool::new(5);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    // let lines: Vec<_> = reader
    //     .lines()
    //     .map(|line| line.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    // println!("request lines: {:#?}", lines);
    let req_line = reader.lines().next().unwrap().unwrap();

    let (status_line, filepath) = match &req_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "./simweb/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "./simweb/hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "./simweb/404.html"),
    };
    
    let contents = fs::read_to_string(filepath).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    stream.write_all(response.as_bytes()).unwrap();
}
