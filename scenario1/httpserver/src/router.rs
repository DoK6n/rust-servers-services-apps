use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{http_request, http_request::HttpRequest, http_response::HttpResponse};
use std::io::prelude::*;

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {

            http_request::Method::Get => match &req.resource {
                http_request::Resource::Path(s) => {
                    // parse URI
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => {  // path가 /api로 시작하면 WebServiceHandler를 사용한다.
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        _ => {  // 그 외의 경우는 StaticPageHandler를 사용한다.
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
    
}
