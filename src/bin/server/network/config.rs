use aeronet_webtransport::wtransport::{Identity, ServerConfig};
use bevy::utils::Duration;

pub fn server_config(identity: &Identity) -> ServerConfig {
    ServerConfig::builder()
        .with_bind_default(25565)
        .with_identity(identity)
        .keep_alive_interval(Some(Duration::from_secs(1)))
         .max_idle_timeout(Some(Duration::from_secs(5)))
        .expect("should be a valid idle timeout")
        .build()
}