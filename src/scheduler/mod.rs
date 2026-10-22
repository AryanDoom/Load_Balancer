mod round_robin;

use crate::backend::Backend;

pub use round_robin::RoundRobin;

pub trait Scheduler: Send + Sync {
    fn select<'a>(&self, backend_list: &'a [Backend]) -> Option<&'a Backend>;
}
