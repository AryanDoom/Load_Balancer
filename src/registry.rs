use crate::backend::Backend;
#[derive(Debug, Default)]

//this is the struct that holds the backend list and does fucntions related to it
pub struct BackendRegistry {
    backend_list: Vec<Backend>,
}
//all the funcs related to backend registry
impl BackendRegistry {
    pub fn new() -> Self {
        let backend_registry = Self {
            backend_list: Vec::new(),
        };
        //final func call for the new backend registry
        backend_registry
    }
    // this func adds a backend to the backend list, it checks if the backend id is already in use and returns an error if it is
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
    //removes backend
    pub fn all_backends(&self) -> &[Backend] {
        &self.backend_list
    }
    //basically sets the health of the server based on the id and health check
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
