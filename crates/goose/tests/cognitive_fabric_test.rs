//! Integration tests for cognitive fabric

use goose::cognitive_fabric::*;
use uuid::Uuid;

#[tokio::test]
async fn test_cognitive_coordinator_basic() {
    let coordinator = CognitiveCoordinator::new();

    let mut state = CognitiveState::default();
    state.capabilities.push("test_capability".to_string());

    coordinator.register_agent(state.clone()).await.unwrap();

    let states = coordinator.get_agent_states().await;
    assert_eq!(states.len(), 1);
    assert_eq!(states[0].agent_id, state.agent_id);
}

#[tokio::test]
async fn test_distributed_knowledge() {
    let coordinator = CognitiveCoordinator::new();
    let agent_id = Uuid::new_v4();

    coordinator
        .share_knowledge(agent_id, "test_key".to_string(), "test_value".to_string())
        .await
        .unwrap();

    let value = coordinator.get_knowledge("test_key").await;
    assert_eq!(value, Some("test_value".to_string()));
}

#[tokio::test]
async fn test_self_healing_health_check() {
    let coordinator = CognitiveCoordinator::new();
    let event_tx = coordinator.get_event_sender();
    let healing_manager = SelfHealingManager::new(event_tx);

    let mut state = CognitiveState::default();
    state.health_score = 0.2;

    healing_manager.check_health(&state).await.unwrap();
}

#[tokio::test]
async fn test_autognosis_introspection() {
    let engine = AutognosisEngine::new();

    let mut state = CognitiveState::default();
    state.capabilities.push("skill_1".to_string());
    state.active_tasks.push("task_1".to_string());

    let report = engine.introspect(&state).await.unwrap();

    assert_eq!(report.agent_id, state.agent_id);
    assert!(report.metrics.contains_key("health_score"));
    assert!(report.metrics.contains_key("task_count"));
    assert!(!report.insights.is_empty());
}

#[tokio::test]
async fn test_autogenesis_evolution() {
    let coordinator = CognitiveCoordinator::new();
    let event_tx = coordinator.get_event_sender();
    let engine = AutogenesisEngine::new(event_tx);
    let autognosis = AutognosisEngine::new();

    let mut state = CognitiveState::default();
    state.health_score = 0.5;

    let report = autognosis.introspect(&state).await.unwrap();
    let proposals = engine.propose_evolution(&state, &report).await.unwrap();

    assert!(!proposals.is_empty());
}

#[tokio::test]
async fn test_autobuild_framework() {
    let coordinator = CognitiveCoordinator::new();
    let event_tx = coordinator.get_event_sender();
    let mut framework = AutobuildFramework::new(event_tx);
    let autognosis = AutognosisEngine::new();

    let mut state = CognitiveState::default();
    state.health_score = 0.6;

    let report = autognosis.introspect(&state).await.unwrap();
    let config = framework.generate_config(&state, &report).await.unwrap();

    assert_eq!(config.agent_id, state.agent_id);
    assert!(!config.settings.is_empty());
}

#[tokio::test]
async fn test_cognitive_lifecycle() {
    let coordinator = CognitiveCoordinator::new();
    let event_tx = coordinator.get_event_sender();

    let mut state = CognitiveState::default();
    state.capabilities.push("basic_capability".to_string());

    coordinator.register_agent(state.clone()).await.unwrap();

    coordinator
        .share_knowledge(
            state.agent_id,
            "domain_knowledge".to_string(),
            "cognitive_fabric".to_string(),
        )
        .await
        .unwrap();

    let healing_manager = SelfHealingManager::new(event_tx.clone());
    healing_manager.check_health(&state).await.unwrap();

    let autognosis = AutognosisEngine::new();
    let report = autognosis.introspect(&state).await.unwrap();
    assert!(!report.metrics.is_empty());

    let autogenesis = AutogenesisEngine::new(event_tx.clone());
    let proposals = autogenesis
        .propose_evolution(&state, &report)
        .await
        .unwrap();

    if !proposals.is_empty() {
        autogenesis
            .apply_evolution(&mut state, &proposals[0])
            .await
            .unwrap();
    }

    let mut framework = AutobuildFramework::new(event_tx);
    let config = framework.generate_config(&state, &report).await.unwrap();
    framework.apply_config(&mut state, &config).await.unwrap();

    assert!(!state.knowledge_base.is_empty());
}
