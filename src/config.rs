use crate::backend::Backend;
use std::net::SocketAddr;
#[derive(Debug)]
pub struct Config {
    //config ios the struct that holds the config for the lb to run, it holds the listener address and the list of backends
    pub listener_address: SocketAddr,
    pub backends: Vec<Backend>,
}

//all the initialization and validation functions for the config struct
impl Config {
    pub fn new(listener_address: SocketAddr, backends: Vec<Backend>) -> Self {
        let config = Self {
            listener_address,
            backends,
        };

        config
    }
    //to check if the confid is valid or nah
    pub fn validate(&self) -> Result<(), String> {
        if self.backends.is_empty() {
            return Err("add at least one backend before starting the load balancer".to_owned());
        }

        for backend_index in 0..self.backends.len() {
            let current_backend = &self.backends[backend_index];

            for other_backend_index in (backend_index + 1)..self.backends.len() {
                let other_backend = &self.backends[other_backend_index];

                if current_backend.id == other_backend.id {
                    let error_message = format!(
                        "backend id '{}' is configured more than once",
                        current_backend.id
                    );
                    return Err(error_message);
                }
            }
        }

        Ok(())
    }
}
