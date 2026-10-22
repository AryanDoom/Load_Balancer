mod round_robin;

pub use round_robin::RoundRobin;

//this "trait" is used to define the behavior of a scheduler which is responsible for selecting a backend from a list of available servers or backends
pub trait Scheduler: Send + Sync {
    fn select(&self, backend_count: usize) -> Option<usize>; //main func call for selecting a backend from the list of available backends
}
