use quinn::{ClientConfig, Endpoint, NewConnection};
use std::{
    fs,
    io::{self, BufReader},
};
use try_quinn::config::{ANY_ADDR_STR, CA_CERT, LISTEN_ADDR_STR, SERVER_NAME};

#[tokio::main]
async fn main() {
    let any_addr = ANY_ADDR_STR.parse().unwrap();
    let listen_addr = LISTEN_ADDR_STR.parse().unwrap();
    let mut certs = rustls::RootCertStore::empty();
    {
        let cert_file = fs::File::open(CA_CERT).unwrap();
        let mut cert_file_rdr = BufReader::new(cert_file);
        let cert_vec = rustls_pemfile::certs(&mut cert_file_rdr).unwrap();
        assert_eq!(cert_vec.len(), 1);
        for cert in cert_vec {
            certs.add(&rustls::Certificate(cert)).unwrap();
        }
    }
    let config = ClientConfig::with_root_certificates(certs);
    let mut client = Endpoint::client(any_addr).unwrap();
    client.set_default_client_config(config);
    let connecting = client.connect(listen_addr, SERVER_NAME).unwrap();
    let NewConnection { connection, .. } = connecting.await.unwrap();
    println!("connected");
    loop {
        let (mut send, _recv) = connection.open_bi().await.unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line = line.trim_end().to_string();

        send.write_all(line.as_bytes()).await.unwrap();
        send.finish().await.unwrap();
    }
}
