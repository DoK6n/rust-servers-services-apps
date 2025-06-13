use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    // 스트림을 버퍼로 읽는다.
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    
    // 실제로 읽은 바이트만 처리
    let request = String::from_utf8_lossy(&buffer[0..bytes_read]);
    let request = request.trim(); // 공백, 줄바꿈 등 모두 제거
    
    println!("받은 데이터: '{}'", request); // 디버깅용


    // 메시지를 만들고 스트림을 쓴다.
    let message = format!("Hello, {}", request);
    stream.write(message.as_bytes())?;

    Ok(())

}
fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        match handle_client(stream?) {
            Ok(_) => (),
            Err(e) => eprintln!("클라이언트 처리 중 오류 발생: {}", e),
        }
    }
    Ok(())
}

