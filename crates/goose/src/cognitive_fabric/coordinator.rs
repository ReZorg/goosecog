//! Cognitive coordinator for distributed cognition

use super::types::{CognitiveEvent, CognitiveState, DistributedKnowledge};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, info};
use uuid::Uuid;

/// Coordinates cognitive operations across distributed agents
pub struct CognitiveCoordinator {
    knowledge: DistributedKnowledge,
    agents: Arc<RwLock<Vec<CognitiveState>>>,
    event_tx: mpsc::Sender<CognitiveEvent>,
    _event_rx: Arc<RwLock<mpsc::Receiver<CognitiveEvent>>>,
}

impl CognitiveCoordinator {
    pub fn new() -> Self {
        let (event_tx, event_rx) = mpsc::channel(100);

        Self {
            knowledge: DistributedKnowledge::new(),
            agents: Arc::new(RwLock::new(Vec::new())),
            event_tx,
            _event_rx: Arc::new(RwLock::new(event_rx)),
        }
    }

    pub async fn register_agent(&self, state: CognitiveState) -> Result<()> {
        info!("Registering agent: {}", state.agent_id);

        let mut agents = self.agents.write().await;
        agents.push(state.clone());

        self.event_tx
            .send(CognitiveEvent::StateUpdate {
                agent_id: state.agent_id,
                state,
            })
            .await?;

        Ok(())
    }

    pub async fn share_knowledge(&self, source: Uuid, key: String, value: String) -> Result<()> {
        debug!("Sharing knowledge from {}: {} = {}", source, key, value);

        self.knowledge.insert(key.clone(), value, source).await;

        let agents = self.agents.read().await;
        for agent in agents.iter() {
            if agent.agent_id != source {
                self.event_tx
                    .send(CognitiveEvent::KnowledgeShare {
                        source,
                        target: agent.agent_id,
                        knowledge: key.clone(),
                    })
                    .await?;
            }
        }

        Ok(())
    }

    pub async fn get_knowledge(&self, key: &str) -> Option<String> {
        self.knowledge.get(key).await.map(|entry| entry.value)
    }

    pub async fn update_agent_state(&self, agent_id: Uuid, state: CognitiveState) -> Result<()> {
        let mut agents = self.agents.write().await;

        if let Some(agent) = agents.iter_mut().find(|a| a.agent_id == agent_id) {
            *agent = state.clone();

            self.event_tx
                .send(CognitiveEvent::StateUpdate { agent_id, state })
                .await?;
        }

        Ok(())
    }

    pub async fn get_agent_states(&self) -> Vec<CognitiveState> {
        let agents = self.agents.read().await;
        agents.clone()
    }

    pub fn get_event_sender(&self) -> mpsc::Sender<CognitiveEvent> {
        self.event_tx.clone()
    }
}

impl Default for CognitiveCoordinator {
    fn default() -> Self {
        Self::new()
    }
}
