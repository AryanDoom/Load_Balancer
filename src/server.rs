use std::io;

use crate::{
    listener::ClientListener, proxy::handle_client_connection, registry::BackendRegistry,
    scheduler::Scheduler,
};

pub async fn run_load_balancer<S: Scheduler>(
    client_listener: ClientListener,
    backend_registry: BackendRegistry,
    scheduler: S,
) -> io::Result<()> {
    loop {
        let accepted_connection = client_listener.accept_client().await;
        let (client_stream, client_address) = accepted_connection?;

        let backend_list = backend_registry.all_backends();
        let selected_backend_index = scheduler.select(backend_list);

        let selected_backend_index = match selected_backend_index {
            Some(backend_index) => backend_index,
            None => {
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

        let backend_address = backend.address;

        tokio::spawn(async move {
            let proxy_result = handle_client_connection(client_stream, backend_address).await;

            match proxy_result {
                Ok((bytes_to_backend, bytes_to_client)) => {
                    println!(
                        "client {client_address} closed; sent {bytes_to_backend} bytes to backend and {bytes_to_client} bytes to client"
                    );
                }
                Err(proxy_error) => {
                    eprintln!("connection from client {client_address} failed: {proxy_error}");
                }
            }
        });
    }
}
