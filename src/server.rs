// holds the data for the server struct
pub struct Server {
    addr: String,
}

// Functionality for the struct
impl Server {
    // needs self to be able to access the data
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
    }
}