
use std::net::TcpListener;

use std::net::TcpStream;

use std::io::prelude::*;

use std::fs;


fn main() {

    //Initialized server

    let address = "127.0.0.1:8000";
    let listener = TcpListener::bind(&address).unwrap();

    println!("Servidor iniciado  en {}",&address);

    //Listening for conexions

    for stream in listener.incoming() {

        let stream = stream.unwrap();

        println!("Escucha activa");


        handle_conexion(stream);

    }

}

fn handle_conexion(mut stream: TcpStream) {
    let mut buffer=[0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Stream recibido");
    println!("{}",String::from_utf8_lossy(&buffer[..]));


    let get = b"GET /index HTTP/1.1";

    if buffer.starts_with(get) {
        send_index(stream);
    } else {
        send_not_found(stream);
    }
    }

fn build_response(content: String) -> String {
    format!(
        
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content,

    )


}


fn send_index(mut stream: TcpStream){

    let contents = fs::read_to_string("index.html").unwrap();

    let response = format!(
        
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents,

    );

    stream.write(build_response(contents).as_bytes()).unwrap();

    stream.flush().unwrap();

}

fn send_not_found(mut stream: TcpStream){

    let contents = fs::read_to_string("404.html").unwrap();

    let response = format!(
        
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents,

    );

    stream.write(build_response(contents).as_bytes()).unwrap();

    stream.flush().unwrap();


}

