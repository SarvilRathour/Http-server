#[allow(unused_imports)]
use std::net::{TcpListener,TcpStream};
use std::io::prelude::*;
use std::io::{self, BufRead as _, BufReader, Write};
fn handle_client(mut stream:&TcpStream)->io::Result<()>{
     let mut reader=BufReader::new(stream);
     let mut url_path=String::new();
     reader.read_line(&mut url_path)?;
     println!("{}",url_path);
     url_path = url_path.trim().to_string();
     match url_path.as_str(){
          "GET /abcdefg HTTP/1.1"=>stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n"),
          "GET / HTTP/1.1"=>stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n"),
          _=>stream.write_all(b"HTTP/1.1 400 BAD REQUEST\r\n\r\n"),
     };
     Ok(())
}
fn main() {
     let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
     for stream in listener.incoming() {
         let mut stream=stream.unwrap();
         handle_client(&stream);
         // stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
     }
     
}
