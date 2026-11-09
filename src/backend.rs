use std::{
    net::SocketAddr,
    sync::{atomic::{AtomicUsize, Ordering}, Arc},
};

#[derive(Debug)]
pub struct Backend { //all the defining characs for a backend server
    pub id: String,
    pub address: SocketAddr,
    pub is_healthy: bool,
    pub weight: usize,
    active_connections: Arc<AtomicUsize>,
}

impl Backend {
    pub fn new(backend_id: impl Into<String>, backend_address: SocketAddr) -> Self {
        let backend_id = backend_id.into();

        let backend = Self {
            id: backend_id,
            address: backend_address, //basically the ip address plus a port number of the server sum bs like 222.0.0.1:6969
            is_healthy: true,
            weight: 1,
            active_connections: Arc::new(AtomicUsize::new(0)),
        };
        backend // main func call for creating a new backend
    }
//this is a new function for creating a backend with a weight value (for wrr)
    pub fn new_with_weight(backend_id: impl Into<String>,backend_address: SocketAddr,backend_weight: usize,) -> Result<Self, String> {
        if backend_weight == 0 {
            return Err("backend weight shld be atlaest 1".to_owned());
        }
        let backend_id = backend_id.into();
        let backend = Self {
            id: backend_id,
            address: backend_address,
            is_healthy: true,
            weight: backend_weight,
            active_connections: Arc::new(AtomicUsize::new(0)),
        };

        Ok(backend)
    }

    pub fn active_connection_count(&self) -> usize {
        self.active_connections.load(Ordering::Relaxed)
    }

    pub(crate) fn start_connection(&self) -> ActiveConnectionGuard {
        self.active_connections.fetch_add(1, Ordering::Relaxed);

        let connection_guard = ActiveConnectionGuard {
            active_connections: Arc::clone(&self.active_connections),
        };

        connection_guard
    }
}

pub(crate) struct ActiveConnectionGuard {
    active_connections: Arc<AtomicUsize>,
}

impl Drop for ActiveConnectionGuard {
    fn drop(&mut self) {
        self.active_connections.fetch_sub(1, Ordering::Relaxed);
    }
}
