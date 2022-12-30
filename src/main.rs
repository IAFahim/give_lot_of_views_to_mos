use std::io::{self, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let mut stream = TcpStream::connect("camo.githubusercontent.com:80").unwrap();
    loop {
        thread::sleep(Duration::from_millis(150));
        run(&mut stream);
    }
}

fn run(stream: &mut TcpStream) {
    stream.write_all(b"GET /0b0f3d7d2ca2fc0fa4fe34243ab56d73c33873ac8c0b8e592a926078ce9de80a/68747470733a2f2f6b6f6d617265762e636f6d2f67687076632f3f757365726e616d653d6d6f73726f6f726d6f66697a61726d616e26636f6c6f723d627269676874677265656e267374796c653d666f722d7468652d6261646765266c6162656c3d56495349544f5253 HTTP/1.1\r\nHost: camo.githubusercontent.com\r\n\r\n").unwrap();
}