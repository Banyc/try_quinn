use futures_util::StreamExt;
use quinn::{Endpoint, IdleTimeout, Incoming, NewConnection, ServerConfig, TransportConfig};
use std::{error::Error, fs, io::BufReader, sync::Arc};
use try_quinn::config::{
    IDLE_TIMEOUT, KEEP_ALIVE_INTERVAL, LISTEN_ADDR_STR, SERVER_CERT, SERVER_KEY, READ_SIZE_LIMIT,
};

#[tokio::main]
async fn main() {
    let mut listener = Listener::new().await.unwrap();
    while let Some(mut new_connection) = listener.accept().await.unwrap() {
        println!("connected");
        while let Some(Ok((mut _send, recv))) = new_connection.bi_streams.next().await {
            let bytes = match recv.read_to_end(READ_SIZE_LIMIT).await {
                Ok(x) => x,
                Err(e) => match e {
                    quinn::ReadToEndError::Read(e) => match e {
                        quinn::ReadError::Reset(_) => panic!(),
                        quinn::ReadError::ConnectionLost(e) => {
                            println!("{}", e);
                            panic!();
                        }
                        quinn::ReadError::UnknownStream => panic!(),
                        quinn::ReadError::IllegalOrderedRead => panic!(),
                        quinn::ReadError::ZeroRttRejected => panic!(),
                    },
                    quinn::ReadToEndError::TooLong => panic!(),
                },
            };
            let line = String::from_utf8_lossy(&bytes);
            println!("{}", line);
        }
    }
}

pub struct Listener {
    local_endpoint: Endpoint,
    incoming: Incoming,
}

impl Listener {
    pub async fn new() -> Result<Listener, Box<dyn Error>> {
        let listen_addr = LISTEN_ADDR_STR.parse()?;
        let certs: Vec<_> = {
            let cert_file = fs::File::open(SERVER_CERT)?;
            let mut cert_file_rdr = BufReader::new(cert_file);
            let cert_vec = rustls_pemfile::certs(&mut cert_file_rdr)?;
            cert_vec
                .into_iter()
                .map(|cert| rustls::Certificate(cert))
                .collect()
        };
        let key = {
            let key_file = fs::File::open(SERVER_KEY)?;
            let mut key_file_rdr = BufReader::new(key_file);
            // // "BEGIN RSA PRIVATE KEY"
            // let mut key_vec = rustls_pemfile::rsa_private_keys(&mut key_file_rdr)?;
            // "BEGIN PRIVATE KEY"
            let mut key_vec = rustls_pemfile::pkcs8_private_keys(&mut key_file_rdr)?;
            assert_eq!(key_vec.len(), 1);
            rustls::PrivateKey(key_vec.remove(0))
        };
        let config = {
            let mut config = ServerConfig::with_single_cert(certs, key)?;
            let mut transport_config = TransportConfig::default();
            let idle_timeout = IdleTimeout::try_from(IDLE_TIMEOUT)?;
            transport_config.max_idle_timeout(Some(idle_timeout));
            transport_config.keep_alive_interval(Some(KEEP_ALIVE_INTERVAL));
            config.transport = Arc::new(transport_config);
            config
        };
        let (endpoint, incoming) = Endpoint::server(config, listen_addr)?;
        let this = Listener {
            local_endpoint: endpoint,
            incoming,
        };
        Ok(this)
    }

    pub async fn accept(&mut self) -> Result<Option<NewConnection>, Box<dyn Error>> {
        match self.incoming.next().await {
            Some(connecting) => {
                let new_connection = connecting.await?;
                Ok(Some(new_connection))
            }
            None => Ok(None),
        }
    }

    #[must_use]
    pub fn local_endpoint(&self) -> &Endpoint {
        &self.local_endpoint
    }
}
