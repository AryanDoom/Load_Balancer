use std::net::SocketAddr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Backend {
    pub id: String,
    pub address: SocketAddr,
}

impl Backend {
    pub fn new(backend_id: impl Into<String>, backend_address: SocketAddr) -> Self {
        let backend_id = backend_id.into();

        let backend = Self {
            id: backend_id,
            address: backend_address,
        };

        backend
    }
}
