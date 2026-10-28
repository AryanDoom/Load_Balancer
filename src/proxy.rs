use std::{io, net::SocketAddr};
use tokio::{io::copy_bidirectional, net::TcpStream};

//this is the func that handles the client connection and forwards the data to the backend server
pub async fn handle_client_connection(
    mut client_stream: TcpStream,
    backend_address: SocketAddr,
) -> io::Result<(u64, u64)> {
    //opens a connection to the backend server and forwards the data btw client and server
    let mut backend_stream = TcpStream::connect(backend_address).await?;
    let copied_byte_counts = copy_bidirectional(&mut client_stream, &mut backend_stream).await?; //main data transfer
     //number of bytes transferred from cli to server
    let bytes_from_client_to_backend = copied_byte_counts.0;
    let bytes_from_backend_to_client = copied_byte_counts.1;

    Ok((bytes_from_client_to_backend, bytes_from_backend_to_client))
}
