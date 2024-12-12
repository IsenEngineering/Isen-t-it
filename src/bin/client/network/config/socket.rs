use aeronet_websocket::client::ClientConfig;

pub fn config() -> ClientConfig {
    ClientConfig::builder()
        .with_no_cert_validation()
}