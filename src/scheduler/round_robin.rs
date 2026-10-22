use std::sync::atomic::{AtomicUsize, Ordering};

use super::Scheduler;
use crate::backend::Backend;

#[derive(Debug, Default)]
pub struct RoundRobin {
    next_backend_index: AtomicUsize,
}

impl Scheduler for RoundRobin {
    fn select<'a>(&self, backend_list: &'a [Backend]) -> Option<&'a Backend> {
        let backend_count = backend_list.len();

        if backend_count == 0 {
            return None;
        }

        let current_backend_index = self.next_backend_index.fetch_add(1, Ordering::Relaxed);

        let selected_backend_index = current_backend_index % backend_count;
        let selected_backend = backend_list.get(selected_backend_index);

        selected_backend
    }
}
