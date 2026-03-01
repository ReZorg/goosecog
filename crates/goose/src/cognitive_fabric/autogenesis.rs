//! Autogenesis: Self-evolution and adaptive improvement engine

use super::autognosis::IntrospectionReport;
use super::types::{CognitiveEvent, CognitiveState};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::info;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionProposal {
    pub id: Uuid,
    pub agent_id: Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub proposal_type: EvolutionType,
    pub description: String,
    pub expected_impact: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvolutionType {
    CapabilityAddition { capability: String },
    ConfigurationUpdate { key: String, value: String },
    BehaviorModification { behavior: String },
    KnowledgeAcquisition { topic: String },
}

pub struct AutogenesisEngine {
    event_tx: mpsc::Sender<CognitiveEvent>,
}

impl AutogenesisEngine {
    pub fn new(event_tx: mpsc::Sender<CognitiveEvent>) -> Self {
        Self { event_tx }
    }

    pub async fn propose_evolution(
        &self,
        state: &CognitiveState,
        report: &IntrospectionReport,
    ) -> Result<Vec<EvolutionProposal>> {
        info!(
            "Generating evolution proposals for agent {}",
            state.agent_id
        );

        let mut proposals = Vec::new();

        if report.metrics.get("health_score").unwrap_or(&1.0) < &0.8 {
            proposals.push(EvolutionProposal {
                id: Uuid::new_v4(),
                agent_id: state.agent_id,
                timestamp: chrono::Utc::now(),
                proposal_type: EvolutionType::BehaviorModification {
                    behavior: "error_handling".to_string(),
                },
                description: "Improve error handling to increase health score".to_string(),
                expected_impact: 0.2,
            });
        }

        if state.capabilities.len() < 3 {
            proposals.push(EvolutionProposal {
                id: Uuid::new_v4(),
                agent_id: state.agent_id,
                timestamp: chrono::Utc::now(),
                proposal_type: EvolutionType::CapabilityAddition {
                    capability: "adaptive_learning".to_string(),
                },
                description: "Add adaptive learning capability".to_string(),
                expected_impact: 0.3,
            });
        }

        if state.knowledge_base.is_empty() {
            proposals.push(EvolutionProposal {
                id: Uuid::new_v4(),
                agent_id: state.agent_id,
                timestamp: chrono::Utc::now(),
                proposal_type: EvolutionType::KnowledgeAcquisition {
                    topic: "domain_knowledge".to_string(),
                },
                description: "Acquire domain-specific knowledge".to_string(),
                expected_impact: 0.25,
            });
        }

        for proposal in &proposals {
            self.event_tx
                .send(CognitiveEvent::EvolutionProposal {
                    agent_id: state.agent_id,
                    proposal: format!("{:?}", proposal.proposal_type),
                })
                .await?;
        }

        Ok(proposals)
    }

    pub async fn apply_evolution(
        &self,
        state: &mut CognitiveState,
        proposal: &EvolutionProposal,
    ) -> Result<()> {
        info!("Applying evolution proposal: {}", proposal.description);

        match &proposal.proposal_type {
            EvolutionType::CapabilityAddition { capability } => {
                state.capabilities.push(capability.clone());
            }
            EvolutionType::ConfigurationUpdate { key, value } => {
                state.knowledge_base.insert(key.clone(), value.clone());
            }
            EvolutionType::BehaviorModification { behavior } => {
                state
                    .knowledge_base
                    .insert(format!("behavior:{}", behavior), "modified".to_string());
            }
            EvolutionType::KnowledgeAcquisition { topic } => {
                state
                    .knowledge_base
                    .insert(format!("knowledge:{}", topic), "acquired".to_string());
            }
        }

        state.health_score = (state.health_score + proposal.expected_impact).min(1.0);

        Ok(())
    }
}
