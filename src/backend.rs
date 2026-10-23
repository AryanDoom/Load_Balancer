use std::net::SocketAddr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Backend {
    pub id: String,
    pub address: SocketAddr,
    pub is_healthy: bool,
}

impl Backend {
    pub fn new(backend_id: impl Into<String>, backend_address: SocketAddr) -> Self {
        let backend_id = backend_id.into();

        let backend = Self {
            id: backend_id,
            address: backend_address, //basically the ip address plus a port number of the server sum bs like 222.0.0.1:6969
            is_healthy: true,
        };
        backend // main func call for creating a new backend
    }
}
