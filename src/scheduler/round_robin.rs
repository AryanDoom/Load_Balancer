use std::sync::atomic::{AtomicUsize, Ordering};
use super::Scheduler;
#[derive(Debug, Default)] 
pub struct RoundRobin {
    next_backend_index: AtomicUsize,//this is teh atomic var that keeps track of the next server to be selected from the list of available servers or backends
}
impl Scheduler for RoundRobin {
    fn select(&self, backend_count: usize) -> Option<usize> {
        if backend_count == 0 {
            return None;
        }

        let current_backend_index = self.next_backend_index.fetch_add(1, Ordering::Relaxed); //increment

        let selected_backend_index = current_backend_index % backend_count; //curr index

        Some(selected_backend_index) //returns the index of the selected backend from the list of avail backends
    }
}
