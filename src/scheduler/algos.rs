use std::{
    hash::{Hash, Hasher},
    net::SocketAddr,
    sync::atomic::{AtomicUsize, Ordering},
};

//all the algorithms that will be in the scheduler are here and will be called from here

use rand::Rng;

use super::Scheduler;
use crate::backend::Backend;

#[derive(Clone, Copy, Debug)]
pub enum Algorithm { //we have 5 as of now might add a few more like least reponse time and resource based schdeuling
    RoundRobin,
    WeightedRoundRobin,
    LeastConnections,
    IpHash,
    Random,
}

#[derive(Debug)]
pub struct Algos { //this is to hold the next algo and the next index for say the rr wr and least connection 
    selected_algorithm: Algorithm,
    next_round_robin_index: AtomicUsize,
    next_weight_position: AtomicUsize,
    next_least_connections_tie: AtomicUsize,
}


//main implementaion for the scheduler 
impl Algos {
    pub fn new(selected_algorithm: Algorithm) -> Self { //alogs is the main struct with the info about the next alorithm adn teh values we shld have with it 
        let algos = Self {
            selected_algorithm,
            next_round_robin_index: AtomicUsize::new(0),
            next_weight_position: AtomicUsize::new(0),
            next_least_connections_tie: AtomicUsize::new(0),
        };

        algos
    }


//now all taht would happen if u were to select a backend and one of these algorithms
    fn select_round_robin(&self, backend_list: &[Backend]) -> Option<usize> {
        let mut healthy_backend_indexes = Vec::new(); //healthy backend list declaration
    
        for (backend_index, backend) in backend_list.iter().enumerate() { //going throught teh list of teh available backends and selecting on the healthy ones
            if backend.is_healthy {
                healthy_backend_indexes.push(backend_index);
            }
        }

        if healthy_backend_indexes.is_empty() { //gone  every backend got corona
            return None;
        }

        let current_backend_index = self.next_round_robin_index.fetch_add(1, Ordering::Relaxed); //increase the index for next server

        let selected_healthy_position = current_backend_index % healthy_backend_indexes.len();
        let selected_backend_index = healthy_backend_indexes.get(selected_healthy_position);

        selected_backend_index.copied() //returns the index of the selected backend from the list of avail backends
    }

    fn select_weighted_round_robin(&self, backend_list: &[Backend]) -> Option<usize> {
        let mut total_healthy_weight = 0;

        for backend in backend_list {
            if backend.is_healthy {
                total_healthy_weight += backend.weight;
            }
        }

        if total_healthy_weight == 0 {
            return None;
        }

        let current_weight_position =
            self.next_weight_position.fetch_add(1, Ordering::Relaxed) % total_healthy_weight;

        let mut remaining_weight_position = current_weight_position;

        for (backend_index, backend) in backend_list.iter().enumerate() {
            if !backend.is_healthy {
                continue;
            }

            if remaining_weight_position < backend.weight {
                return Some(backend_index);
            }

            remaining_weight_position -= backend.weight;
        }

        None
    }

    fn select_least_connections(&self, backend_list: &[Backend]) -> Option<usize> {
        let mut least_active_connections = usize::MAX;
        let mut least_busy_backend_indexes = Vec::new();

        for (backend_index, backend) in backend_list.iter().enumerate() {
            if !backend.is_healthy {
                continue;
            }

            let active_connection_count = backend.active_connection_count();
            

            //the comparison logic
            if active_connection_count < least_active_connections {
                least_active_connections = active_connection_count;
                least_busy_backend_indexes.clear();
                least_busy_backend_indexes.push(backend_index);
            } else if active_connection_count == least_active_connections {
                least_busy_backend_indexes.push(backend_index);
            }
        }

        if least_busy_backend_indexes.is_empty() {
            return None;
        }

        //tie breaker if many servers wuth same number of connections
        let tie_number = self
            .next_least_connections_tie
            .fetch_add(1, Ordering::Relaxed);
        let selected_tie_position = tie_number % least_busy_backend_indexes.len();
        //return the index of the selected backend from the list of least busy backends
        least_busy_backend_indexes
            .get(selected_tie_position)
            .copied()
    }


    //ip hash basically same ip goes to teh same server unless its down and shi
    fn select_ip_hash(&self,backend_list: &[Backend],client_address: SocketAddr,) -> Option<usize> {
        let mut healthy_backend_indexes = Vec::new();

        for (backend_index, backend) in backend_list.iter().enumerate() {
            if backend.is_healthy {
                healthy_backend_indexes.push(backend_index);
            }
        }

        if healthy_backend_indexes.is_empty() {
            return None;
        }

        let client_ip_address = client_address.ip();
        let mut address_hasher = std::collections::hash_map::DefaultHasher::new();
        client_ip_address.hash(&mut address_hasher);

        let hash_value = address_hasher.finish();
        let selected_healthy_position = hash_value as usize % healthy_backend_indexes.len();

        healthy_backend_indexes
            .get(selected_healthy_position)
            .copied()
    }
    //tis is the basicest thing ever
    fn select_random(&self, backend_list: &[Backend]) -> Option<usize> {
        let mut healthy_backend_indexes = Vec::new();

        for (backend_index, backend) in backend_list.iter().enumerate() {
            if backend.is_healthy {
                healthy_backend_indexes.push(backend_index);
            }
        }

        if healthy_backend_indexes.is_empty() {
            return None;
        }

        let mut random_generator = rand::thread_rng();
        let selected_healthy_position =
            random_generator.gen_range(0..healthy_backend_indexes.len());

        healthy_backend_indexes
            .get(selected_healthy_position)
            .copied()
    }
}
//all the function calls 
impl Scheduler for Algos {
    fn select(&self, backend_list: &[Backend], client_address: SocketAddr) -> Option<usize> {
        match self.selected_algorithm {
            Algorithm::RoundRobin => self.select_round_robin(backend_list),
            Algorithm::WeightedRoundRobin => self.select_weighted_round_robin(backend_list),
            Algorithm::LeastConnections => self.select_least_connections(backend_list),
            Algorithm::IpHash => self.select_ip_hash(backend_list, client_address),
            Algorithm::Random => self.select_random(backend_list),
        }
    }
}
