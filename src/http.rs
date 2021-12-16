pub mod request {
    use super::method::Method;


    pub struct Request {
        path: String,
        query_string: Option<String>,
        method: Method,
    }
}

mod method {
    pub enum Method {
        GET,
        POST,
        PATCH,
        PUT,
        HEAD,
        CONNECT,
        OPTIONS,
        DELETE,
        TRACE,
    }
}