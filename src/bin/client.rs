use quinn::{ClientConfig, Endpoint, IdleTimeout, NewConnection, TransportConfig};
use std::{
    error::Error,
    fs,
    io::{self, BufReader},
    sync::Arc,
};
use try_quinn::config::{
    ANY_ADDR_STR, CA_CERT, IDLE_TIMEOUT, KEEP_ALIVE_INTERVAL, LISTEN_ADDR_STR, SERVER_NAME,
};

#[tokio::main]
async fn main() {
    let NewConnection { connection, .. } = connect().await.unwrap();
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

async fn connect() -> Result<NewConnection, Box<dyn Error>> {
    let any_addr = ANY_ADDR_STR.parse()?;
    let listen_addr = LISTEN_ADDR_STR.parse()?;
    let mut certs = rustls::RootCertStore::empty();
    {
        let cert_file = fs::File::open(CA_CERT)?;
        let mut cert_file_rdr = BufReader::new(cert_file);
        let cert_vec = rustls_pemfile::certs(&mut cert_file_rdr)?;
        assert_eq!(cert_vec.len(), 1);
        for cert in cert_vec {
            certs.add(&rustls::Certificate(cert))?;
        }
    }
    let config = {
        let mut config = ClientConfig::with_root_certificates(certs);
        let mut transport_config = TransportConfig::default();
        let idle_timeout = IdleTimeout::try_from(IDLE_TIMEOUT)?;
        transport_config.max_idle_timeout(Some(idle_timeout));
        transport_config.keep_alive_interval(Some(KEEP_ALIVE_INTERVAL));
        config.transport = Arc::new(transport_config);
        config
    };
    let mut client = Endpoint::client(any_addr)?;
    client.set_default_client_config(config);
    let connecting = client.connect(listen_addr, SERVER_NAME)?;
    let new_connection = connecting.await?;
    Ok(new_connection)
}
