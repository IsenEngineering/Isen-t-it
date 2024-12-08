use aeronet_webtransport::{
    client::ClientConfig,
    cert
};
use aeronet::io::anyhow;

#[cfg(target_family = "wasm")]
pub fn client_config(cert_hash: String) -> Result<ClientConfig, anyhow::Error> {
    use aeronet_webtransport::xwt_web_sys::{CertificateHash, HashAlgorithm};

    let server_certificate_hashes = match cert::hash_from_b64(&cert_hash) {
        Ok(hash) => vec![CertificateHash {
            algorithm: HashAlgorithm::Sha256,
            value: Vec::from(hash),
        }],
        Err(err) => {
            println!("Failed to read certificate hash from string: {err:?}",);
            Vec::new()
        }
    };

    Ok(ClientConfig {
        server_certificate_hashes,
        ..Default::default()
    })
}

#[cfg(not(target_family = "wasm"))]
pub fn client_config(cert_hash: String) -> Result<ClientConfig, anyhow::Error> {
    use bevy::utils::Duration;
    use aeronet_webtransport::wtransport::tls::Sha256Digest;

    let config = ClientConfig::builder().with_bind_default();

    let config = if cert_hash.is_empty() {
        config.with_server_certificate_hashes([])
    } else {
        let hash = cert::hash_from_b64(&cert_hash)?;
        config.with_server_certificate_hashes([Sha256Digest::new(hash)])
    };

    Ok(config
        .keep_alive_interval(Some(Duration::from_secs(1)))
        .max_idle_timeout(Some(Duration::from_secs(5)))
        .expect("should be a valid idle timeout")
        .build())
}