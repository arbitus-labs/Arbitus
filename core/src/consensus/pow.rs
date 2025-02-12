use super::*;
use crate::block::Block;
use blake3::Hash;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

/// PoW difficulty target
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Target(pub [u8; 32]);

impl Target {
    pub fn to_difficulty(&self) -> f64 {
        let max_target = [0xff; 32];
        let current = u256_from_bytes(&self.0);
        let max = u256_from_bytes(&max_target);
        (max as f64) / (current as f64)
    }
}

/// Mining result
#[derive(Debug)]
pub struct MiningResult {
    pub nonce: u64,
    pub hash: Hash,
}

/// PoW miner implementation
pub struct Miner {
    target: Target,
}

impl Miner {
    pub fn new(target: Target) -> Self {
        Self { target }
    }

    /// Mine a new block
    pub fn mine(&self, block: &mut Block) -> MiningResult {
        let mut rng = rand::thread_rng();
        
        loop {
            // Generate random nonce
            block.header.nonce = rng.gen();
            
            // Calculate block hash
            let hash = self.calculate_hash(block);
            
            // Check if hash meets target
            if self.check_hash(&hash) {
                return MiningResult {
                    nonce: block.header.nonce,
                    hash,
                };
            }
        }
    }

    /// Calculate block hash
    fn calculate_hash(&self, block: &Block) -> Hash {
        let mut hasher = blake3::Hasher::new();
        
        // Add block header fields
        hasher.update(&block.header.previous_hash);
        hasher.update(&block.header.merkle_root);
        hasher.update(&block.header.timestamp.to_be_bytes());
        hasher.update(&block.header.nonce.to_be_bytes());
        
        hasher.finalize()
    }

    /// Check if hash meets target
    fn check_hash(&self, hash: &Hash) -> bool {
        let hash_bytes = hash.as_bytes();
        for i in 0..32 {
            if hash_bytes[i] > self.target.0[i] {
                return false;
            }
            if hash_bytes[i] < self.target.0[i] {
                return true;
            }
        }
        true
    }
}

/// Convert bytes to u256
fn u256_from_bytes(bytes: &[u8; 32]) -> u128 {
    let mut result = 0u128;
    for &byte in bytes.iter().take(16) {
        result = (result << 8) | (byte as u128);
    }
    result
}

/// Adjust difficulty based on block time
pub fn adjust_difficulty(
    current_target: Target,
    actual_time: u64,
    expected_time: u64,
) -> Target {
    let ratio = actual_time as f64 / expected_time as f64;
    let current_difficulty = current_target.to_difficulty();
    let new_difficulty = current_difficulty * ratio;
    
    // Convert difficulty back to target
    let max_target = [0xff; 32];
    let max = u256_from_bytes(&max_target) as f64;
    let new_target = (max / new_difficulty) as u128;
    
    let mut target = [0u8; 32];
    for i in 0..16 {
        target[i] = ((new_target >> (8 * (15 - i))) & 0xff) as u8;
    }
    
    Target(target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mining() {
        let target = Target([0x00, 0xff; 16]);
        let miner = Miner::new(target);
        
        let mut block = Block::new(
            vec![0; 32],
            vec![0; 32],
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            vec![],
        );
        
        let result = miner.mine(&mut block);
        assert!(miner.check_hash(&result.hash));
    }

    #[test]
    fn test_difficulty_adjustment() {
        let initial_target = Target([0x00, 0xff; 16]);
        
        // Test increase difficulty
        let new_target = adjust_difficulty(initial_target, 5, 10);
        assert!(new_target.to_difficulty() < initial_target.to_difficulty());
        
        // Test decrease difficulty
        let new_target = adjust_difficulty(initial_target, 20, 10);
        assert!(new_target.to_difficulty() > initial_target.to_difficulty());
    }
}
