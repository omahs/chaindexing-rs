use crate::{ChaindexingRepo, Chains, Contract};

#[derive(Clone)]
pub struct Config {
    pub chains: Chains,
    pub repo: ChaindexingRepo,
    pub contracts: Vec<Contract>,
    pub reset_count: u8,
    pub blocks_per_batch: u64,
    pub handler_interval_ms: u64,
    pub ingestion_interval_ms: u64,
}

impl Config {
    pub fn new(repo: ChaindexingRepo, chains: Chains) -> Self {
        Self {
            repo,
            chains,
            contracts: vec![],
            reset_count: 0,
            blocks_per_batch: 20,
            handler_interval_ms: 10000,
            ingestion_interval_ms: 10000,
        }
    }

    pub fn add_contract(mut self, contract: Contract) -> Self {
        self.contracts.push(contract);

        self
    }

    pub fn reset(mut self, count: u8) -> Self {
        self.reset_count = count;

        self
    }

    pub fn with_blocks_per_batch(&self, blocks_per_batch: u64) -> Self {
        Self {
            blocks_per_batch,
            ..self.clone()
        }
    }

    pub fn with_handler_interval_ms(&self, handler_interval_ms: u64) -> Self {
        Self {
            handler_interval_ms,
            ..self.clone()
        }
    }

    pub fn with_ingestion_interval_ms(&self, ingestion_interval_ms: u64) -> Self {
        Self {
            ingestion_interval_ms,
            ..self.clone()
        }
    }
}
