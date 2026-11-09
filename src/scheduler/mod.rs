use std::net::SocketAddr;
use crate::backend::Backend;
mod algos;
pub use algos::{Algorithm, Algos};

//this "trait" is used to define the behavior of a scheduler which is responsible for selecting a backend from a list of available servers or backends typeshi
pub trait Scheduler: Send + Sync {
    fn select(&self, backend_list: &[Backend], client_address: SocketAddr) -> Option<usize>; //main func call for selecting a backend from the list of available backends
}
