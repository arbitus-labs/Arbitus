use orbitus_core::block::IntentData;
use rust_bert::pipelines::sequence_classification::SequenceClassificationModel;
use tokenizers::Tokenizer;
use tch::{Device, Tensor};

#[derive(Debug, Clone)]
pub struct Config {
    pub model_path: String,
    pub tokenizer_path: String,
    pub max_length: usize,
    pub batch_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model_path: "models/intent_classifier".to_string(),
            tokenizer_path: "models/tokenizer.json".to_string(),
            max_length: 512,
            batch_size: 32,
        }
    }
}

pub struct IntentProcessor {
    config: Config,
    model: Option<SequenceClassificationModel>,
    tokenizer: Option<Tokenizer>,
}

impl IntentProcessor {
    pub fn new(config: Config) -> crate::Result<Self> {
        Ok(Self {
            config,
            model: None,
            tokenizer: None,
        })
    }

    pub async fn initialize(&mut self) -> crate::Result<()> {
        // Load model
        self.model = Some(SequenceClassificationModel::new(Default::default())?);
        
        // Load tokenizer
        self.tokenizer = Some(Tokenizer::from_file(&self.config.tokenizer_path)
            .map_err(|e| crate::Error::Model(e.to_string()))?);

        Ok(())
    }

    pub async fn process(&self, intent: &IntentData) -> crate::Result<ProcessedIntent> {
        let tokenizer = self.tokenizer.as_ref()
            .ok_or_else(|| crate::Error::Model("Tokenizer not initialized".to_string()))?;
        
        let model = self.model.as_ref()
            .ok_or_else(|| crate::Error::Model("Model not initialized".to_string()))?;

        // Tokenize input
        let encoding = tokenizer.encode(
            intent.content.clone(),
            true
        ).map_err(|e| crate::Error::Model(e.to_string()))?;

        // Convert to tensors
        let input_ids = Tensor::from_slice(&encoding.get_ids())
            .view((1, -1))
            .to(Device::Cpu);
        
        let attention_mask = Tensor::from_slice(&encoding.get_attention_mask())
            .view((1, -1))
            .to(Device::Cpu);

        // Get model prediction
        let output = model.forward(
            &[input_ids], 
            Some(&[attention_mask]),
            None,
            false,
        )?;

        // Process output
        let processed = ProcessedIntent {
            original: intent.clone(),
            classification: self.process_output(&output)?,
            embeddings: output.hidden_states.unwrap_or_default(),
        };

        Ok(processed)
    }

    fn process_output(&self, output: &rust_bert::pipelines::common::ModelOutput) 
        -> crate::Result<IntentClassification> {
        // Get logits
        let logits = output.logits.as_ref()
            .ok_or_else(|| crate::Error::Model("No logits in output".to_string()))?;

        // Get predictions
        let predictions = logits.softmax(-1, logits.kind());
        let (scores, classes) = predictions.max_dim(-1, false);

        Ok(IntentClassification {
            class_id: classes.int64_value(&[0]) as u32,
            confidence: scores.double_value(&[0]),
        })
    }

    pub async fn cleanup(&mut self) -> crate::Result<()> {
        self.model = None;
        self.tokenizer = None;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ProcessedIntent {
    pub original: IntentData,
    pub classification: IntentClassification,
    pub embeddings: Vec<Tensor>,
}

#[derive(Debug, Clone)]
pub struct IntentClassification {
    pub class_id: u32,
    pub confidence: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbitus_core::block::{IntentType, ProcessingRequirements};

    #[tokio::test]
    async fn test_intent_processing() {
        let config = Config::default();
        let mut processor = IntentProcessor::new(config).unwrap();
        processor.initialize().await.unwrap();

        let intent = IntentData {
            intent_type: IntentType::Prediction,
            content: "Test prediction intent".to_string(),
            requirements: ProcessingRequirements {
                compute_units: 100,
                memory_mb: 1024,
                max_time_ms: 5000,
            },
            proof: vec![],
        };

        let result = processor.process(&intent).await.unwrap();
        assert!(result.classification.confidence > 0.0);
    }
}
