use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Block header containing metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    /// Version number
    pub version: u32,
    /// Previous block hash
    pub previous_hash: [u8; 32],
    /// Merkle root of transactions
    pub merkle_root: [u8; 32],
    /// Block timestamp
    pub timestamp: u64,
    /// Target difficulty
    pub target: [u8; 32],
    /// Nonce for PoW
    pub nonce: u64,
}

/// Block containing transactions and header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Block header
    pub header: BlockHeader,
    /// List of transactions
    pub transactions: Vec<Transaction>,
    /// AI intent data
    pub intent_data: Option<IntentData>,
}

/// Transaction in a block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction ID
    pub id: [u8; 32],
    /// Sender's public key
    pub sender: [u8; 32],
    /// Receiver's public key
    pub receiver: [u8; 32],
    /// Transaction amount
    pub amount: u64,
    /// Transaction fee
    pub fee: u64,
    /// Transaction data
    pub data: Vec<u8>,
    /// Transaction signature
    pub signature: [u8; 64],
}

/// AI Intent data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentData {
    /// Intent type
    pub intent_type: IntentType,
    /// Intent content
    pub content: String,
    /// Processing requirements
    pub requirements: ProcessingRequirements,
    /// Verification proof
    pub proof: Vec<u8>,
}

/// Types of AI intents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntentType {
    Prediction,
    Classification,
    Generation,
    Optimization,
    Custom(String),
}

/// Processing requirements for AI intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingRequirements {
    /// Required computational resources
    pub compute_units: u64,
    /// Required memory
    pub memory_mb: u64,
    /// Maximum processing time
    pub max_time_ms: u64,
}

impl Block {
    /// Create a new block
    pub fn new(
        previous_hash: [u8; 32],
        merkle_root: [u8; 32],
        timestamp: u64,
        transactions: Vec<Transaction>,
    ) -> Self {
        Self {
            header: BlockHeader {
                version: 1,
                previous_hash,
                merkle_root,
                timestamp,
                target: [0; 32],
                nonce: 0,
            },
            transactions,
            intent_data: None,
        }
    }

    /// Add AI intent data to block
    pub fn with_intent(mut self, intent_data: IntentData) -> Self {
        self.intent_data = Some(intent_data);
        self
    }

    /// Calculate block hash
    pub fn calculate_hash(&self) -> [u8; 32] {
        use blake3::Hasher;
        let mut hasher = Hasher::new();
        
        // Hash header fields
        hasher.update(&self.header.version.to_be_bytes());
        hasher.update(&self.header.previous_hash);
        hasher.update(&self.header.merkle_root);
        hasher.update(&self.header.timestamp.to_be_bytes());
        hasher.update(&self.header.target);
        hasher.update(&self.header.nonce.to_be_bytes());
        
        // Hash transactions
        for tx in &self.transactions {
            hasher.update(&bincode::serialize(tx).unwrap());
        }
        
        // Hash intent data if present
        if let Some(intent) = &self.intent_data {
            hasher.update(&bincode::serialize(intent).unwrap());
        }
        
        *hasher.finalize().as_bytes()
    }

    /// Validate block structure and contents
    pub fn validate(&self) -> bool {
        // Check timestamp
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        if self.header.timestamp > current_time + 7200 { // 2 hours in the future
            return false;
        }

        // Validate transactions
        for tx in &self.transactions {
            if !self.validate_transaction(tx) {
                return false;
            }
        }

        // Validate intent data if present
        if let Some(intent) = &self.intent_data {
            if !self.validate_intent(intent) {
                return false;
            }
        }

        true
    }

    /// Validate a single transaction
    fn validate_transaction(&self, tx: &Transaction) -> bool {
        // Basic validation
        if tx.amount == 0 || tx.sender == [0; 32] || tx.receiver == [0; 32] {
            return false;
        }

        // Validate signature
        // TODO: Implement signature validation
        true
    }

    /// Validate intent data
    fn validate_intent(&self, intent: &IntentData) -> bool {
        // Validate processing requirements
        if intent.requirements.compute_units == 0 
            || intent.requirements.memory_mb == 0 
            || intent.requirements.max_time_ms == 0 {
            return false;
        }

        // Validate proof
        // TODO: Implement proof validation
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let block = Block::new(
            [0; 32],
            [0; 32],
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            vec![],
        );
        
        assert_eq!(block.header.version, 1);
        assert!(block.validate());
    }

    #[test]
    fn test_block_with_intent() {
        let intent = IntentData {
            intent_type: IntentType::Prediction,
            content: "Test intent".to_string(),
            requirements: ProcessingRequirements {
                compute_units: 100,
                memory_mb: 1024,
                max_time_ms: 5000,
            },
            proof: vec![],
        };

        let block = Block::new([0; 32], [0; 32], 0, vec![])
            .with_intent(intent);
        
        assert!(block.intent_data.is_some());
    }
}
