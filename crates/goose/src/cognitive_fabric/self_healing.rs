//! Self-healing mechanisms for cognitive agents

use super::types::{AlertSeverity, CognitiveEvent, CognitiveState};
use anyhow::Result;
use tokio::sync::mpsc;
use tracing::{info, warn};
use uuid::Uuid;

const HEALTH_THRESHOLD_LOW: f64 = 0.7;
const HEALTH_THRESHOLD_CRITICAL: f64 = 0.3;
const MAX_ERRORS_BEFORE_RECOVERY: usize = 5;

pub struct SelfHealingManager {
    event_tx: mpsc::Sender<CognitiveEvent>,
}

impl SelfHealingManager {
    pub fn new(event_tx: mpsc::Sender<CognitiveEvent>) -> Self {
        Self { event_tx }
    }

    pub async fn check_health(&self, state: &CognitiveState) -> Result<()> {
        if state.health_score < HEALTH_THRESHOLD_CRITICAL {
            warn!("Critical health detected for agent {}", state.agent_id);
            
            self.event_tx
                .send(CognitiveEvent::HealthAlert {
                    agent_id: state.agent_id,
                    severity: AlertSeverity::Critical,
                    message: format!("Health score: {}", state.health_score),
                })
                .await?;
            
            self.initiate_recovery(state.agent_id, "critical_health").await?;
        } else if state.health_score < HEALTH_THRESHOLD_LOW {
            info!("Low health detected for agent {}", state.agent_id);
            
            self.event_tx
                .send(CognitiveEvent::HealthAlert {
                    agent_id: state.agent_id,
                    severity: AlertSeverity::Medium,
                    message: format!("Health score: {}", state.health_score),
                })
                .await?;
        }

        if state.error_history.len() >= MAX_ERRORS_BEFORE_RECOVERY {
            warn!(
                "High error count detected for agent {}: {}",
                state.agent_id,
                state.error_history.len()
            );
            
            self.initiate_recovery(state.agent_id, "error_threshold").await?;
        }

        Ok(())
    }

    pub async fn initiate_recovery(&self, agent_id: Uuid, reason: &str) -> Result<()> {
        info!("Initiating recovery for agent {}: {}", agent_id, reason);
        
        let action = match reason {
            "critical_health" => "reset_state_and_restart",
            "error_threshold" => "clear_errors_and_restart",
            _ => "standard_recovery",
        };

        self.event_tx
            .send(CognitiveEvent::RecoveryInitiated {
                agent_id,
                action: action.to_string(),
            })
            .await?;

        Ok(())
    }

    pub fn calculate_health_score(state: &CognitiveState) -> f64 {
        let base_score = 1.0;
        
        let error_penalty = (state.error_history.len() as f64) * 0.05;
        
        let task_load_penalty = if state.active_tasks.len() > 10 {
            0.1
        } else {
            0.0
        };

        (base_score - error_penalty - task_load_penalty).max(0.0).min(1.0)
    }
}
