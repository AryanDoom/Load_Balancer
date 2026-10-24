use std::{io, net::SocketAddr};

use tokio::net::{TcpListener, TcpStream}; //japan kawaiiii :3

pub struct ClientListener {
    //it is a struct that holds the listener for the lb to listen for incoming clnt connection, it supposed to return the accepted connection to the lb to be processed and sent to the selected backend
    listener: TcpListener,
}

//all the funcs of the listener
impl ClientListener {
    //it opens a tcp listening socket at the given address adn teh socket is wrapped in the clientlistener
    pub async fn bind(listener_address: SocketAddr) -> io::Result<Self> {
        let tcp_listener = TcpListener::bind(listener_address).await?;

        let client_listener = Self {
            listener: tcp_listener,
        };

        Ok(client_listener)
    }

    //to get the local address of the listener
    pub fn local_address(&self) -> io::Result<SocketAddr> {
        let listener_address = self.listener.local_addr()?;

        Ok(listener_address)
    }

    //to accept the inc connec frfom the clint and return teh accepted connec to the lb which ic then sent to the bakcend
    pub async fn accept_client(&self) -> io::Result<(TcpStream, SocketAddr)> {
        let accepted_connection = self.listener.accept().await?;

        let (client_stream, client_address) = accepted_connection;

        Ok((client_stream, client_address))
    }
}
