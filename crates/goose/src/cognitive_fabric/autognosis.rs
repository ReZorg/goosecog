//! Autognosis: Self-introspection and self-knowledge engine

use super::types::CognitiveState;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntrospectionReport {
    pub agent_id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metrics: HashMap<String, f64>,
    pub insights: Vec<String>,
    pub recommendations: Vec<String>,
}

pub struct AutognosisEngine;

impl AutognosisEngine {
    pub fn new() -> Self {
        Self
    }

    pub async fn introspect(&self, state: &CognitiveState) -> Result<IntrospectionReport> {
        debug!("Performing introspection for agent {}", state.agent_id);

        let mut metrics = HashMap::new();
        metrics.insert("health_score".to_string(), state.health_score);
        metrics.insert("task_count".to_string(), state.active_tasks.len() as f64);
        metrics.insert("error_count".to_string(), state.error_history.len() as f64);
        metrics.insert("knowledge_entries".to_string(), state.knowledge_base.len() as f64);
        metrics.insert("capability_count".to_string(), state.capabilities.len() as f64);

        let insights = self.generate_insights(state, &metrics);
        let recommendations = self.generate_recommendations(state, &metrics);

        Ok(IntrospectionReport {
            agent_id: state.agent_id,
            timestamp: chrono::Utc::now(),
            metrics,
            insights,
            recommendations,
        })
    }

    fn generate_insights(&self, state: &CognitiveState, _metrics: &HashMap<String, f64>) -> Vec<String> {
        let mut insights = Vec::new();

        if state.health_score < 0.5 {
            insights.push("Agent health is degraded, may need recovery".to_string());
        }

        if state.active_tasks.len() > 5 {
            insights.push("High task load detected".to_string());
        }

        if state.error_history.len() > 3 {
            insights.push("Recurring errors detected, pattern analysis recommended".to_string());
        }

        if state.knowledge_base.is_empty() {
            insights.push("Knowledge base is empty, consider knowledge acquisition".to_string());
        }

        insights
    }

    fn generate_recommendations(&self, state: &CognitiveState, _metrics: &HashMap<String, f64>) -> Vec<String> {
        let mut recommendations = Vec::new();

        if state.health_score < 0.7 {
            recommendations.push("Consider reducing task load or initiating recovery".to_string());
        }

        if state.error_history.len() > 2 {
            recommendations.push("Review error patterns and implement preventive measures".to_string());
        }

        if state.capabilities.is_empty() {
            recommendations.push("Register available capabilities for better task distribution".to_string());
        }

        recommendations
    }
}

impl Default for AutognosisEngine {
    fn default() -> Self {
        Self::new()
    }
}
