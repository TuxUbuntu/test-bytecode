
#[derive(Debug)]
pub struct Error(String);

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error(s.to_owned())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error(s)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self)
    }
}

#[cfg(feature = "actix-web")]
impl actix_web::ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let res = actix_web::HttpResponse::new(self.status_code());
        res.set_body(actix_web::body::BoxBody::new(self.0.clone()))
    }
}

