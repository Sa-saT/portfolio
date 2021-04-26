extern crate server_rust;
//use actix_files as fs;
//use actix_web::{web, App, HttpServer};
use server_rust::ThreadPool;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

// fn creat() {
//     HttpServer::new(|| App::new().service(web::scope("imges")))
// }

fn main() {
    let url = "18.221.114.239:80";
    let listener = TcpListener::bind(url).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        //println!("Connection established!");

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    // 閉じます
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let get2 = b"GET /main.html HTTP/1.1\r\n";
    // let pictu = b"GET / *.gif HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let sleep2 = b"GET /main.html sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else if buffer.starts_with(get2) {
        ("HTTP/1.1 200 OK\r\n\r\n", "main.html")
    // } else if buffer.starts_with(pictu) {
    //     ("HTTP/1.1 200 OK\r\n\r\n", "imgs/ *.gif")
    } else if buffer.starts_with(sleep2) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "main.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
