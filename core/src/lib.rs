use std::sync::Arc;
use tokio::sync::RwLock;

pub mod block;
pub mod chain;
pub mod consensus;
pub mod network;
pub mod storage;
pub mod types;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Consensus error: {0}")]
    Consensus(String),
    #[error("Network error: {0}")]
    Network(String),
    #[error("Storage error: {0}")]
    Storage(String),
    #[error("Invalid block: {0}")]
    InvalidBlock(String),
    #[error("Chain error: {0}")]
    Chain(String),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Core blockchain state
#[derive(Clone)]
pub struct BlockchainState {
    /// Chain storage
    storage: Arc<RwLock<storage::Storage>>,
    /// Network state
    network: Arc<RwLock<network::Network>>,
    /// Consensus engine
    consensus: Arc<RwLock<consensus::Consensus>>,
}

impl BlockchainState {
    pub async fn new(config: Config) -> Result<Self> {
        let storage = Arc::new(RwLock::new(storage::Storage::new(config.storage_path)?));
        let network = Arc::new(RwLock::new(network::Network::new(config.network_config)?));
        let consensus = Arc::new(RwLock::new(consensus::Consensus::new(
            config.consensus_config,
            storage.clone(),
        )?));

        Ok(Self {
            storage,
            network,
            consensus,
        })
    }

    pub async fn start(&self) -> Result<()> {
        // Start network
        self.network.write().await.start().await?;

        // Start consensus
        self.consensus.write().await.start().await?;

        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        // Stop consensus
        self.consensus.write().await.stop().await?;

        // Stop network
        self.network.write().await.stop().await?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub storage_path: String,
    pub network_config: network::Config,
    pub consensus_config: consensus::Config,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            storage_path: "./data".to_string(),
            network_config: network::Config::default(),
            consensus_config: consensus::Config::default(),
        }
    }
}
