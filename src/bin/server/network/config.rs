use aeronet_webtransport::wtransport::{tls::{Certificate, CertificateChain, PrivateKey}, Identity, ServerConfig};
use bevy::utils::Duration;
use std::env::var;

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

    let certificat = Certificate::from_der(cert_der)
        .expect("Le certificat n'est pas valide!");

    let private_key = PrivateKey::from_der_pkcs8(key_der);

    Identity::new(
        CertificateChain::single(certificat), 
        private_key
    )
} 

const DEFAULT_PORT: u16 = 25565;
pub fn server_config(identity: &Identity) -> ServerConfig {
    use aeronet_webtransport::wtransport::config::IpBindConfig::InAddrAnyV4;
    let port = match var("PORT") {
        Ok(s) => s.parse::<u16>().unwrap_or(DEFAULT_PORT),
        _ => DEFAULT_PORT
    };

    ServerConfig::builder()
        .with_bind_config(InAddrAnyV4, port)
        .with_identity(identity)
        .keep_alive_interval(Some(Duration::from_secs(1)))
        .max_idle_timeout(Some(Duration::from_secs(5)))
        .expect("should be a valid idle timeout")
        .build()
}