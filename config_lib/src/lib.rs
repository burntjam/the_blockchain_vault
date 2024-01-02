use config::{Config, ConfigError, File, Environment};
use serde::Deserialize;
use environment_lib::*;

#[derive(Debug, Deserialize)]
pub struct ChainConfig {
    pub version: String,
    pub peer_client: PeerClientConfig,
    pub peer_server: PeerServerConfig,
    pub spool: SpoolConfig,
    pub rdf: RDFConfig,
    pub block_db: BlockDBConfig,
    pub http: HttpApiConfig,
}

#[derive(Debug, Deserialize)]
pub struct PeerClientConfig {
    pub account: String,
    pub master: String,
}

#[derive(Debug, Deserialize)]
pub struct PeerServerConfig {
    pub account: String,
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct SpoolConfig {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct RDFConfig {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct BlockDBConfig {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct HttpApiConfig {
    pub binding: String,
}

impl ChainConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut config = Config::new();
        let environment = environment_lib::Environment::new();
        let configFile = format!("{}/etc/config.toml",&environment.home_directory);
        config.merge(File::with_name(configFile.as_str()))?;
        config.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_config() {
        let config = ChainConfig::new().unwrap();
    }
}