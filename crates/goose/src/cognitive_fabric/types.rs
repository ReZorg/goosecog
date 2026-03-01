//! Core types for the cognitive fabric

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Represents the cognitive state of an agent or system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveState {
    pub agent_id: Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub health_score: f64,
    pub knowledge_base: HashMap<String, String>,
    pub capabilities: Vec<String>,
    pub active_tasks: Vec<String>,
    pub error_history: Vec<String>,
}

impl Default for CognitiveState {
    fn default() -> Self {
        Self {
            agent_id: Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            health_score: 1.0,
            knowledge_base: HashMap::new(),
            capabilities: Vec::new(),
            active_tasks: Vec::new(),
            error_history: Vec::new(),
        }
    }
}

/// Events in the cognitive system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CognitiveEvent {
    StateUpdate {
        agent_id: Uuid,
        state: CognitiveState,
    },
    KnowledgeShare {
        source: Uuid,
        target: Uuid,
        knowledge: String,
    },
    HealthAlert {
        agent_id: Uuid,
        severity: AlertSeverity,
        message: String,
    },
    RecoveryInitiated {
        agent_id: Uuid,
        action: String,
    },
    EvolutionProposal {
        agent_id: Uuid,
        proposal: String,
    },
    ConfigUpdate {
        agent_id: Uuid,
        config_change: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Distributed knowledge store
#[derive(Debug, Clone)]
pub struct DistributedKnowledge {
    store: Arc<RwLock<HashMap<String, KnowledgeEntry>>>,
}

impl Default for DistributedKnowledge {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEntry {
    pub key: String,
    pub value: String,
    pub source_agent: Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub confidence: f64,
}

impl DistributedKnowledge {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn insert(&self, key: String, value: String, source_agent: Uuid) {
        let entry = KnowledgeEntry {
            key: key.clone(),
            value,
            source_agent,
            timestamp: chrono::Utc::now(),
            confidence: 1.0,
        };

        let mut store = self.store.write().await;
        store.insert(key, entry);
    }

    pub async fn get(&self, key: &str) -> Option<KnowledgeEntry> {
        let store = self.store.read().await;
        store.get(key).cloned()
    }

    pub async fn all_entries(&self) -> Vec<KnowledgeEntry> {
        let store = self.store.read().await;
        store.values().cloned().collect()
    }
}
