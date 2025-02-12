use std::sync::Arc;
use tokio::sync::RwLock;
use orbitus_core::block::IntentData;

pub mod intent;
pub mod execution;
pub mod verification;
pub mod models;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Intent processing error: {0}")]
    IntentProcessing(String),
    #[error("Model error: {0}")]
    Model(String),
    #[error("Execution error: {0}")]
    Execution(String),
    #[error("Verification error: {0}")]
    Verification(String),
}

pub type Result<T> = std::result::Result<T, Error>;

/// AI processing state
#[derive(Clone)]
pub struct AIProcessor {
    /// Intent processor
    intent_processor: Arc<RwLock<intent::IntentProcessor>>,
    /// Execution engine
    execution_engine: Arc<RwLock<execution::ExecutionEngine>>,
    /// Verification system
    verification_system: Arc<RwLock<verification::VerificationSystem>>,
}

impl AIProcessor {
    pub async fn new(config: Config) -> Result<Self> {
        let intent_processor = Arc::new(RwLock::new(
            intent::IntentProcessor::new(config.intent_config)?
        ));
        
        let execution_engine = Arc::new(RwLock::new(
            execution::ExecutionEngine::new(config.execution_config)?
        ));
        
        let verification_system = Arc::new(RwLock::new(
            verification::VerificationSystem::new(config.verification_config)?
        ));

        Ok(Self {
            intent_processor,
            execution_engine,
            verification_system,
        })
    }

    /// Process an AI intent
    pub async fn process_intent(&self, intent: IntentData) -> Result<Vec<u8>> {
        // Parse and validate intent
        let processed_intent = self.intent_processor
            .read()
            .await
            .process(&intent)
            .await?;

        // Execute intent
        let result = self.execution_engine
            .read()
            .await
            .execute(&processed_intent)
            .await?;

        // Verify result
        let verified_result = self.verification_system
            .read()
            .await
            .verify(&result)
            .await?;

        Ok(verified_result)
    }

    /// Start the AI processor
    pub async fn start(&self) -> Result<()> {
        // Initialize models
        self.intent_processor.write().await.initialize().await?;
        self.execution_engine.write().await.initialize().await?;
        self.verification_system.write().await.initialize().await?;

        Ok(())
    }

    /// Stop the AI processor
    pub async fn stop(&self) -> Result<()> {
        // Cleanup resources
        self.intent_processor.write().await.cleanup().await?;
        self.execution_engine.write().await.cleanup().await?;
        self.verification_system.write().await.cleanup().await?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub intent_config: intent::Config,
    pub execution_config: execution::Config,
    pub verification_config: verification::Config,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            intent_config: intent::Config::default(),
            execution_config: execution::Config::default(),
            verification_config: verification::Config::default(),
        }
    }
}
