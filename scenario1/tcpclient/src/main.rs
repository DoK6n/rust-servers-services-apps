use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

/**
    실제 프로덕션 코드에서는 unwrap가 아닌 match나 ? (try) 연산자를 사용해 적절한 에러 처리하거나 
    [unwrap_or], [unwrap_or_else], 또는 [unwrap_or_default] 호출 해야 한다.
    wnwrap은 성공한 경우 값을 반환하고 실패한 경우 패닉을 발생시킨다.
    러스트에서 Result는 성공 또는 실패 가능성이 있는 연산의 결과를 나타내는 타입이다.
    패닉은 프로그램이 예상치 못한 상황에서 종료되는 것을 의미한다.
*/
fn main() {

    let mut stream = TcpStream::connect("localhost:3000").unwrap();

    stream.write("Hello".as_bytes()).unwrap(); // 'Hello' 메시지를 TCP 서버 커넥션에 쓴다.

    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap(); // 서버로부터 수신된 바이트를 읽는다.

    println!("Got response from server:{:?}", str::from_utf8(&buffer).unwrap()); // 서버로부터 받은 내용을 출력한다. 서버는 가공되지 않은 바이트를 보내므로 UTF-8 문자열로 변환하여 터미널에 출력해야 한다.
}
