#[allow(unused_imports)]
use std::net::{TcpListener,TcpStream};
use std::io::Write;
use std::io::{BufRead,BufReader};
fn main() {
    println!("Logs from your program will appear here!");

     let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
     for stream in listener.incoming() {
         let mut stream=stream.unwrap();
         let mut reader=BufReader::new(&stream);
         let request=reader.lines().next().unwrap().unwrap();
         
         stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
         //
     }
     
}
