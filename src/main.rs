fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

// holds the data for the server struct
struct Server {
    addr: String,
}

// Functionality for the struct
impl Server {
    // needs self to be able to access the data
    fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    fn run(self) {
        println!("Listening on {}", self.addr);
    }
}

struct Request {
    path: String,
    query_string: String,
    method: Method,
}

enum Method {
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
