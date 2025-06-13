use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let connection_listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Server is running on port 3000");

    for stream in connection_listener.incoming() {
        let mut stream = stream.unwrap(); // 스트림을 뮤터블로 만들어 읽고 쓸 수 있게 한다.
        println!("New connection established!");

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap(); // 유입되는 스트림을 읽는다.
        stream.write(&mut buffer).unwrap(); // 받은 데이터를 같은 커넥션을 통해 클라이언트에게 다시 전송한다.
    }
}