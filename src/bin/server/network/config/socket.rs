use aeronet_websocket::server::{Identity, ServerConfig};
use std::{env::var, net::{Ipv4Addr, SocketAddrV4}};
use super::DEFAULT_PORT;

#[cfg(not(feature = "custom-certificate"))]
pub fn identity() -> Identity {
    Identity::self_signed(
        [
            "localhost", 
            "127.0.0.1", 
            "::1"
        ]
    ).expect("all given SANs should be valid DNS names")
} 

#[cfg(feature = "custom-certificate")]
pub fn identity() -> Identity {
    use std::fs::read;
    use aeronet_websocket::rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};

    let key_file = read(".keys/key.pem")
        .expect("Il n'y a pas la clé privée .keys/key.pem");
    let key_pem = pem::parse(key_file)
        .expect(".keys/key.pem n'est pas un fichier pem");
    let key_der = key_pem.contents().to_vec();

    let cert_file = read(".keys/cert.pem")
        .expect("Il n'y a pas le certificat .keys/cert.pem");
    let cert_pem = pem::parse(cert_file)
        .expect(".keys/cert.pem n'est pas un fichier pem");
    let cert_der = cert_pem.contents().to_vec();

    let private_key = PrivateKeyDer::Pkcs8(
        PrivatePkcs8KeyDer::from(key_der)
    );

    let certificate = CertificateDer::from(cert_der);

    Identity::new(
        vec![certificate],
        private_key
    )
}

pub fn config(identity: Identity) -> ServerConfig {
    use std::net::SocketAddr;
    let port = match var("PORT") {
        Ok(s) => s.parse::<u16>().unwrap_or(DEFAULT_PORT),
        _ => DEFAULT_PORT
    };

    ServerConfig::builder()
        .with_bind_address(
            SocketAddr::V4(
                SocketAddrV4::new(
                    Ipv4Addr::new(
                        0, 
                        0, 
                        0, 
                        0
                    ), 
                    port + 1
                )
            )
        )
        .with_identity(identity)
}