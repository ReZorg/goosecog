//! Autobuild configuration framework
//!
//! Automatically generates and updates configurations based on introspection

use super::autognosis::IntrospectionReport;
use super::types::{CognitiveEvent, CognitiveState};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::mpsc;
use tracing::info;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoConfig {
    pub agent_id: Uuid,
    pub version: u32,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub settings: HashMap<String, ConfigValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

pub struct AutobuildFramework {
    event_tx: mpsc::Sender<CognitiveEvent>,
    configs: HashMap<Uuid, AutoConfig>,
}

impl AutobuildFramework {
    pub fn new(event_tx: mpsc::Sender<CognitiveEvent>) -> Self {
        Self {
            event_tx,
            configs: HashMap::new(),
        }
    }

    pub async fn generate_config(
        &mut self,
        state: &CognitiveState,
        report: &IntrospectionReport,
    ) -> Result<AutoConfig> {
        info!("Generating auto-configuration for agent {}", state.agent_id);

        let mut settings = HashMap::new();

        let health_score = report.metrics.get("health_score").unwrap_or(&1.0);
        if health_score < &0.8 {
            settings.insert("recovery_mode".to_string(), ConfigValue::Boolean(true));
            settings.insert("max_concurrent_tasks".to_string(), ConfigValue::Number(3.0));
        } else {
            settings.insert("recovery_mode".to_string(), ConfigValue::Boolean(false));
            settings.insert("max_concurrent_tasks".to_string(), ConfigValue::Number(10.0));
        }

        let error_count = report.metrics.get("error_count").unwrap_or(&0.0);
        if error_count > &5.0 {
            settings.insert("strict_validation".to_string(), ConfigValue::Boolean(true));
            settings.insert("error_threshold".to_string(), ConfigValue::Number(3.0));
        }

        settings.insert(
            "monitoring_interval".to_string(),
            ConfigValue::Number(60.0),
        );
        settings.insert(
            "knowledge_sync_enabled".to_string(),
            ConfigValue::Boolean(true),
        );

        let version = self
            .configs
            .get(&state.agent_id)
            .map(|c| c.version + 1)
            .unwrap_or(1);

        let config = AutoConfig {
            agent_id: state.agent_id,
            version,
            timestamp: chrono::Utc::now(),
            settings,
        };

        self.configs.insert(state.agent_id, config.clone());

        self.event_tx
            .send(CognitiveEvent::ConfigUpdate {
                agent_id: state.agent_id,
                config_change: format!("Auto-config v{} generated", version),
            })
            .await?;

        Ok(config)
    }

    pub fn get_config(&self, agent_id: &Uuid) -> Option<&AutoConfig> {
        self.configs.get(agent_id)
    }

    pub async fn apply_config(&self, state: &mut CognitiveState, config: &AutoConfig) -> Result<()> {
        info!("Applying auto-configuration v{} to agent {}", config.version, state.agent_id);

        for (key, value) in &config.settings {
            let value_str = match value {
                ConfigValue::String(s) => s.clone(),
                ConfigValue::Number(n) => n.to_string(),
                ConfigValue::Boolean(b) => b.to_string(),
            };
            state.knowledge_base.insert(format!("config:{}", key), value_str);
        }

        Ok(())
    }
}
