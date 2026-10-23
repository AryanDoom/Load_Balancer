use std::sync::atomic::{AtomicUsize, Ordering};

use super::Scheduler;
use crate::backend::Backend;

#[derive(Debug, Default)]
pub struct RoundRobin {
    next_backend_index: AtomicUsize, //this is teh atomic var that keeps track of the next server to be selected from the list of available servers or backends
}

impl Scheduler for RoundRobin {
    fn select(&self, backend_list: &[Backend]) -> Option<usize> {
        let mut healthy_backend_indexes = Vec::new();

        for (backend_index, backend) in backend_list.iter().enumerate() {
            if backend.is_healthy {
                healthy_backend_indexes.push(backend_index);
            }
        }

        let healthy_backend_count = healthy_backend_indexes.len();

        if healthy_backend_count == 0 {
            return None;
        }

        let current_backend_index = self.next_backend_index.fetch_add(1, Ordering::Relaxed); //increment

        let selected_healthy_position = current_backend_index % healthy_backend_count;
        let selected_backend_index = healthy_backend_indexes.get(selected_healthy_position);

        selected_backend_index.copied() //returns the index of the selected backend from the list of avail backends
    }
}
