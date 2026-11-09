use std::io;

use crate::{
    listener::ClientListener, proxy::handle_client_connection, registry::BackendRegistry,
    scheduler::Scheduler,
};

//main loop for the lb, it accepts clients and sends them to the selected backend
pub async fn run_load_balancer<S: Scheduler>(
    client_listener: ClientListener,
    backend_registry: BackendRegistry,
    scheduler: S,
) -> io::Result<()> {
    loop {
        //wait for the next client to connect
        let accepted_connection = client_listener.accept_client().await;
        let (client_stream, client_address) = accepted_connection?;

        //the scheduler gives back the index of the backend to use
        let backend_list = backend_registry.all_backends();
        let selected_backend_index = scheduler.select(backend_list, client_address);

        let selected_backend_index = match selected_backend_index {
            Some(backend_index) => backend_index,
            None => {
                //no healthy backend to send this client to
                eprintln!("no healthy backend for client {client_address}");
                drop(client_stream);
                continue;
            }
        };

        let selected_backend = backend_list.get(selected_backend_index);

        let backend = match selected_backend {
            Some(backend) => backend,
            None => {
                eprintln!("scheduler returned an invalid backend index");
                drop(client_stream);
                continue;
            }
        };

        //grab the backend address and mark that a connection is starting
        let backend_address = backend.address;
        let active_connection_guard = backend.start_connection();

        //each client gets its own task so the listener can keep accepting more clients
        tokio::spawn(async move {
            //this gets dropped when the connection task ends, which lowers the active count
            let _active_connection_guard = active_connection_guard;
            let proxy_result = handle_client_connection(client_stream, backend_address).await;

            match proxy_result {
                Ok((bytes_to_backend, bytes_to_client)) => {
                    //show how much data went in each direction
                    println!(
                        "client {client_address} closed; sent {bytes_to_backend} bytes to backend and {bytes_to_client} bytes to client"
                    );
                }
                Err(proxy_error) => {
                    //one failed connection shouldnt stop the whole lb
                    eprintln!("connection from client {client_address} failed: {proxy_error}");
                }
            }
        });
    }
}
