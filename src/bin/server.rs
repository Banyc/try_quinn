use futures_util::StreamExt;
use quinn::{Endpoint, NewConnection, ServerConfig};
use std::{fs, io::BufReader};
use try_quinn::config::{LISTEN_ADDR_STR, SERVER_CERT, SERVER_KEY};

#[tokio::main]
async fn main() {
    let listen_addr = LISTEN_ADDR_STR.parse().unwrap();
    let certs: Vec<_> = {
        let cert_file = fs::File::open(SERVER_CERT).unwrap();
        let mut cert_file_rdr = BufReader::new(cert_file);
        let cert_vec = rustls_pemfile::certs(&mut cert_file_rdr).unwrap();
        cert_vec
            .into_iter()
            .map(|cert| rustls::Certificate(cert))
            .collect()
    };
    let key = {
        let key_file = fs::File::open(SERVER_KEY).unwrap();
        let mut key_file_rdr = BufReader::new(key_file);
        // // "BEGIN RSA PRIVATE KEY"
        // let mut key_vec = rustls_pemfile::rsa_private_keys(&mut key_file_rdr).unwrap();
        // "BEGIN PRIVATE KEY"
        let mut key_vec = rustls_pemfile::pkcs8_private_keys(&mut key_file_rdr).unwrap();
        assert_eq!(key_vec.len(), 1);
        rustls::PrivateKey(key_vec.remove(0))
    };
    let config = ServerConfig::with_single_cert(certs, key).unwrap();
    let (_server, mut incoming) = Endpoint::server(config, listen_addr).unwrap();
    while let Some(conn) = incoming.next().await {
        let mut conn: NewConnection = conn.await.unwrap();
        println!("connected");
        while let Some(Ok((mut _send, recv))) = conn.bi_streams.next().await {
            pub const READ_SIZE_LIMIT: usize = 50;
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
