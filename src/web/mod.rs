use actix_web::{body::BoxBody, http::{header::{HeaderName, HeaderValue}, StatusCode}, HttpRequest, HttpResponse, Responder};

pub mod task;

pub struct ServerResponse {
    status: StatusCode,
    body: Option<serde_json::Value>,
}

impl ServerResponse {
    pub fn new(status: StatusCode, body: Option<serde_json::Value>) -> Self {
        return Self { status, body };
    }
}

impl Responder for ServerResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let mut response = HttpResponse::new(self.status);
        response.headers_mut().insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("application/json"),
        );

        if let Some(body) = &self.body {
            let body = serde_json::to_string(body).unwrap();

            return response.set_body(BoxBody::new(body));
        } else {
            return response;
        }
    }
}

