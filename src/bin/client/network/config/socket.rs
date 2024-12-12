use aeronet_websocket::client::ClientConfig;

#[cfg(target_family = "wasm")]
pub fn config() -> ClientConfig {
    ClientConfig::default()
}

#[cfg(not(target_family = "wasm"))]
pub fn config() -> ClientConfig {
    ClientConfig::builder().with_no_cert_validation()
}
