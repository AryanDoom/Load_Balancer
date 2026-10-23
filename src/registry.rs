use crate::backend::Backend;

#[derive(Debug, Default)]
pub struct BackendRegistry {
    backend_list: Vec<Backend>,
}

impl BackendRegistry {
    pub fn new() -> Self {
        let backend_registry = Self {
            backend_list: Vec::new(),
        };

        backend_registry
    }

    pub fn add_backend(&mut self, backend: Backend) -> Result<(), String> {
        for existing_backend in &self.backend_list {
            if existing_backend.id == backend.id {
                let error_message = format!("backend id '{}' is already in use", backend.id);
                return Err(error_message);
            }
        }

        self.backend_list.push(backend);
        Ok(())
    }

    pub fn all_backends(&self) -> &[Backend] {
        &self.backend_list
    }

    pub fn set_backend_health(&mut self, backend_id: &str, is_healthy: bool) -> bool {
        for backend in &mut self.backend_list {
            if backend.id == backend_id {
                backend.is_healthy = is_healthy;
                return true;
            }
        }

        false
    }
}
