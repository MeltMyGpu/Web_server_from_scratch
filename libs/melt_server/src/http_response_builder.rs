pub enum HttpStatusCode {
    StatusOk = 200,
    StatusNotFound = 404,
}

pub struct HttpResponse {
    response_code: String,
}