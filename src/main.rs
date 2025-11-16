use std::io::prelude::*;
use std::io::{self, BufRead as _, BufReader, Write};
#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
//This is what i got in http_request
// Request: [
//     "GET /test/path HTTP/1.1",
//     "Host: 127.0.0.1:4221",
//     "User-Agent: curl/8.13.0",
//     "Accept: */*",
// ]
fn handle_client(mut stream: TcpStream)->io::Result<()>{
    let mut buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let mut response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes());
    let specific_path:Vec<_>=http_request[0].split_whitespace().collect();
    if specific_path[1].contains("/user-agent"){
        let got_header=&http_request[3];
        let returned_values:Vec<&str>=got_header.split(":").collect();
        let returned_value=returned_values[1].trim();
        let passing_return=format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",returned_value.len(),returned_value);
        stream.write_all(passing_return.as_bytes());
        return Ok(());
    }
    if specific_path[1].contains("/echo"){
         let parts_path:Vec<_>=specific_path[1].split("/").collect();
         let lenght_returned=parts_path[2].len();
         let returned_value=format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",lenght_returned,parts_path[2]);
         stream.write_all(returned_value.as_bytes());
         return Ok(());
    }
    match http_request[0].trim() {
        // "GET /abcdefg HTTP/1.1" => stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n"),
        "GET / HTTP/1.1" => stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n"),
        _ => stream.write_all(b"HTTP/1.1 400 BAD REQUEST\r\n\r\n"),
    };
    println!("{:?}",specific_path);
    Ok(())
    // buf_reader.read_line(&mut url_path);
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        handle_client(stream);
        // stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
    }
}
