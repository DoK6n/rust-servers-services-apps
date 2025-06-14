use std::collections::HashMap;
use std::io::{Result, Write};

/*
  Rust에서 라이프타임(lifetime)은 참조(reference)가 유효한 범위를 명시적으로 지정해주는 기능입니다.
  HttpResponse<'a>는 "이 구조체가 참조하는 모든 데이터는 최소한 'a라는 라이프타임 동안 살아있어야 한다"는 의미입니다.
  수동으로 메모리 관리를 수행하는 C/C++과 같은 언어에서 흔하게 발생하는 댕글링 포인터(dangling pointer)나 해제 후 사용(use-after-free) 문제를 방지하기 위해 사용합니다.
    - 댕글링 포인터란? 메모리 해제 후 사용하는 포인터를 의미합니다.
    - 해제 후 사용이란? 메모리 해제 후 해당 메모리를 다시 사용하는 것을 의미합니다.
*/

/*
  #[derive()]: 유도(derivable) 트레이트(trait)를 사용하여 컴파일러에게 이런 트레이트의 구현을 유도할 것을 알립니다.
  - Debug: 디버깅(debug)을 위한 메서드를 제공합니다.
  - PartialEq: 부분적 동등성(partial equality)을 확인하는 메서드를 제공합니다.
  - Clone: 복제(clone)를 허용하는 메서드를 제공합니다.
*/
#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
  pub version: &'a str,
  pub status_code: &'a str,
  pub status_text: &'a str,
  pub headers: Option<HashMap<&'a str, &'a str>>,
  pub body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
  fn default() -> Self {
    Self {
      version: "HTTP/1.1".into(),
      status_code: "200".into(),
      status_text: "OK".into(),
      headers: None,
      body: None,
    }
  }
}

impl<'a> HttpResponse<'a> {
  pub fn new(status_code: &'a str, headers: Option<HashMap<&'a str, &'a str>>, body: Option<String>) -> Self {
    let mut response: HttpResponse<'a> = HttpResponse::default();
    if status_code != "200" {
      response.status_code = status_code.into();
    }
    
    response.headers = match &headers {
      Some(h) => Some(h.clone()),
      None => {
        let mut h = HashMap::new();
        h.insert("Content-Type", "text/html");
        Some(h)
      },
    };

    response.status_text = match response.status_code {
      "200" => "OK".into(),
      "400" => "Bad Request".into(),
      "404" => "Not Found".into(),
      "500" => "Internal Server Error".into(),
      _ => "Not Found".into(),
    };

    response.body = body;

    response
  }

  pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
    let res = self.clone();
    let response_string: String = String::from(res);
    let _ = write!(write_stream, "{}", response_string);
    Ok(())
  }

  fn version(&self) -> &str {
    self.version
  }

  fn status_code(&self) -> &str {
    self.status_code
  }

  fn status_text(&self) -> &str {
    self.status_text
  }

  fn headers(&self) -> String {
    let map = self.headers.clone().unwrap();
    let mut header_string: String = "".into();
    for (k, v) in map.iter() {
      header_string = format!("{}{}: {}\r\n", header_string, k, v);
    }
    header_string
  }

  fn body(&self) -> &str {
    match &self.body {
      Some(b) => b.as_str(),
      None => "",
    }
  }
  
}

impl<'a> From<HttpResponse<'a>> for String {
  fn from(res: HttpResponse) -> String {
    let res1 = res.clone();
    format!(
      "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
      &res1.version(),
      &res1.status_code(),
      &res1.status_text(),
      &res1.headers(),
      &res.body.unwrap_or_default().len(),
      &res1.body()
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_response_struct_creation_200() {
    // Given
    let status_code = "200";
    let headers = None;
    let body = Some("Item was shipped on 21st Dec 2020".into());
    
    let response_expected = HttpResponse {
      version: "HTTP/1.1",
      status_code: "200",
      status_text: "OK",
      headers: {
        let mut h = HashMap::new();
        h.insert("Content-Type", "text/html");
        Some(h)
      },
      body: Some("Item was shipped on 21st Dec 2020".into()),
    };

    // When
    let response_actual = HttpResponse::new(
      status_code,
      headers,
      body,
    );

    // Then
    assert_eq!(response_actual, response_expected);
  }

  #[test]
  fn test_response_struct_creation_404() {
    // Given
    let status_code = "404";
    let headers = None;
    let body = Some("Item was shipped on 21st Dec 2020".into());
    
    let response_expected = HttpResponse {
      version: "HTTP/1.1",
      status_code: "404",
      status_text: "Not Found",
      headers: {
        let mut h = HashMap::new();
        h.insert("Content-Type", "text/html");
        Some(h)
      },
      body: Some("Item was shipped on 21st Dec 2020".into()),
    };

    // When
    let response_actual = HttpResponse::new(
      status_code,
      headers,
      body,
    );

    // Then
    assert_eq!(response_actual, response_expected);
  }
}
